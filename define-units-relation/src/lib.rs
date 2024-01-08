extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::ParseStream;

// Every relation has the format: "unit: dim op unit: dim op unit: dim"
// where
// "unit" is an identifier or "1"
// "dim" is "1", "2", or "3"
// "op" is "*", "/", "==", or "X"

#[proc_macro]
pub fn define_units_relation(input: TokenStream) -> TokenStream {
    match define_units_relation_impl(input) {
        Ok(expanded) => expanded,
        Err(error) => error.into_compile_error().into(),
    }
}

enum UnitsOperation {
    Mul,
    Div,
    Eq,
    Cross,
}

enum UnitsItem {
    Id(syn::Ident),
    One,
}

enum UnitDimensions {
    D1,
    D2,
    D3,
}

struct UnitsRelation {
    unit1: UnitsItem,
    dim1: UnitDimensions, //syn::LitInt,
    operator1: UnitsOperation,
    unit2: UnitsItem,
    dim2: UnitDimensions, //syn::LitInt,
    operator2: UnitsOperation,
    unit3: UnitsItem,
    dim3: UnitDimensions, //syn::LitInt,
}

impl syn::parse::Parse for UnitsRelation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let unit1 = parse_unit_name(&input)?;
        let dim1 = parse_dimensions(&input)?;
        let operator1 = parse_operator(&input)?;
        let unit2 = parse_unit_name(&input)?;
        let dim2 = parse_dimensions(&input)?;
        let operator2 = parse_operator(&input)?;
        let unit3 = parse_unit_name(&input)?;
        let dim3 = parse_dimensions(&input)?;
        Ok(Self {
            unit1,
            dim1,
            operator1,
            unit2,
            dim2,
            operator2,
            unit3,
            dim3,
        })
    }
}

fn parse_unit_name(input: &ParseStream) -> syn::Result<UnitsItem> {
    Ok(if input.lookahead1().peek(syn::Ident) {
        UnitsItem::Id(input.parse::<syn::Ident>()?)
    } else if input.lookahead1().peek(syn::LitInt)
        && input.parse::<syn::LitInt>()?.to_string() == "1"
    {
        UnitsItem::One
    } else {
        return Err(syn::Error::new(
            input.span(),
            "expected identifier or literal `1`",
        ));
    })
}

fn parse_dimensions(input: &ParseStream) -> syn::Result<UnitDimensions> {
    use UnitDimensions::*;
    Ok(if input.lookahead1().peek(syn::Token![:]) {
        input.parse::<syn::Token![:]>()?;
        let lit: syn::LitInt = input.parse()?;
        let n: u8 = lit.base10_parse()?;
        match n {
            1 => D1,
            2 => D2,
            3 => D3,
            _ => {
                return Err(syn::Error::new_spanned(
                    lit,
                    "dimensions can be only 1, 2, or 3",
                ));
            }
        }
    } else {
        D1
    })
}

fn parse_operator(input: &ParseStream) -> syn::Result<UnitsOperation> {
    let lookahead = input.lookahead1();
    Ok(if lookahead.peek(syn::Token![*]) {
        input.parse::<syn::Token![*]>()?;
        UnitsOperation::Mul
    } else if lookahead.peek(syn::Token![/]) {
        input.parse::<syn::Token![/]>()?;
        UnitsOperation::Div
    } else if lookahead.peek(syn::Token![==]) {
        input.parse::<syn::Token![==]>()?;
        UnitsOperation::Eq
    } else if lookahead.peek(syn::Ident) && input.parse::<syn::Ident>().unwrap() == "X" {
        UnitsOperation::Cross
    } else {
        return Err(lookahead.error());
    })
}

fn expand_1_1(unit1: syn::Ident, unit2: syn::Ident, unit3: syn::Ident) -> TokenStream {
    let expanded = if unit1 == unit2 {
        quote! {
            // Measure<U1> * Measure<U1> -> Measure<U3>
            impl<Number: ArithmeticOps> Mul<Measure<#unit1, Number>> for Measure<#unit1, Number> {
                type Output = Measure<#unit3, Number>;
                fn mul(self, other: Measure<#unit1, Number>) -> Self::Output {
                    Self::Output::new(self.value * other.value)
                }
            }

            // Measure<U3> / Measure<U1> -> Measure<U1>
            impl<Number: ArithmeticOps> Div<Measure<#unit1, Number>> for Measure<#unit3, Number> {
                type Output = Measure<#unit1, Number>;
                fn div(self, other: Measure<#unit1, Number>) -> Self::Output {
                    Self::Output::new(self.value / other.value)
                }
            }

            // Measure<U1>.squared() -> Measure<U3>
            impl<Number: ArithmeticOps> Measure<#unit1, Number> {
                fn squared(self) -> Measure<#unit3, Number> {
                    Measure::<#unit3, Number>::new(self.value * self.value)
                }
            }
            // Measure<U3>.sqrt() -> Measure<U1>
            impl<Number: ArithmeticOps> Sqrt for Measure<#unit3, Number> {
                type Output = Measure<#unit1, Number>;
                fn sqrt(self) -> Self::Output {
                    Self::Output::new(self.value.sqrt())
                }
            }
        }
    } else {
        quote! {
            // Measure<U1> * Measure<U2> -> Measure<U3>
            impl<Number: ArithmeticOps> Mul<Measure<#unit2, Number>> for Measure<#unit1, Number> {
                type Output = Measure<#unit3, Number>;
                fn mul(self, other: Measure<#unit2, Number>) -> Self::Output {
                    Self::Output::new(self.value * other.value)
                }
            }

            // Measure<U2> * Measure<U1> -> Measure<U3>
            impl<Number: ArithmeticOps> Mul<Measure<#unit1, Number>> for Measure<#unit2, Number> {
                type Output = Measure<#unit3, Number>;
                fn mul(self, other: Measure<#unit1, Number>) -> Self::Output {
                    Self::Output::new(self.value * other.value)
                }
            }

            // Measure<U3> / Measure<U1> -> Measure<U2>
            impl<Number: ArithmeticOps> Div<Measure<#unit1, Number>> for Measure<#unit3, Number> {
                type Output = Measure<#unit2, Number>;
                fn div(self, other: Measure<#unit1, Number>) -> Self::Output {
                    Self::Output::new(self.value / other.value)
                }
            }

            // Measure<U3> / Measure<U2> -> Measure<U1>
            impl<Number: ArithmeticOps> Div<Measure<#unit2, Number>> for Measure<#unit3, Number> {
                type Output = Measure<#unit1, Number>;
                fn div(self, other: Measure<#unit2, Number>) -> Self::Output {
                    Self::Output::new(self.value / other.value)
                }
            }
        }
    };
    expanded.into()
}

fn expand_1_2(unit1: syn::Ident, unit2: syn::Ident, unit3: syn::Ident) -> TokenStream {
    let expanded = quote! {
        // Measure<U1> * Measure2d<U2> -> Measure2d<U3>
        impl<Number: ArithmeticOps> Mul<Measure2d<#unit2, Number>> for Measure<#unit1, Number> {
            type Output = Measure2d<#unit3, Number>;
            fn mul(self, other: Measure2d<#unit2, Number>) -> Self::Output {
                Self::Output::new(self.value * other.x, self.value * other.y)
            }
        }

        // Measure2d<U2> * Measure<U1> -> Measure2d<U3>
        impl<Number: ArithmeticOps> Mul<Measure<#unit1, Number>> for Measure2d<#unit2, Number> {
            type Output = Measure2d<#unit3, Number>;
            fn mul(self, other: Measure<#unit1, Number>) -> Self::Output {
                Self::Output::new(self.x * other.value, self.y * other.value)
            }
        }

        // Measure2d<U3> / Measure<U1> -> Measure2d<U2>
        impl<Number: ArithmeticOps> Div<Measure<#unit1, Number>> for Measure2d<#unit3, Number> {
            type Output = Measure2d<#unit2, Number>;
            fn div(self, other: Measure<#unit1, Number>) -> Self::Output {
                Self::Output::new(self.x / other.value, self.y / other.value)
            }
        }
    };
    expanded.into()
}

fn expand_1_3(unit1: syn::Ident, unit2: syn::Ident, unit3: syn::Ident) -> TokenStream {
    let expanded = quote! {
        // Measure<U1> * Measure3d<U2> -> Measure3d<U3>
        impl<Number: ArithmeticOps> Mul<Measure3d<#unit2, Number>> for Measure<#unit1, Number> {
            type Output = Measure3d<#unit3, Number>;
            fn mul(self, other: Measure3d<#unit2, Number>) -> Self::Output {
                Self::Output::new(
                    self.value * other.x,
                    self.value * other.y,
                    self.value * other.z,
                )
            }
        }

        // Measure3d<U2> * Measure<U1> -> Measure3d<U3>
        impl<Number: ArithmeticOps> Mul<Measure<#unit1, Number>> for Measure3d<#unit2, Number> {
            type Output = Measure3d<#unit3, Number>;
            fn mul(self, other: Measure<#unit1, Number>) -> Self::Output {
                Self::Output::new(
                    self.x * other.value,
                    self.y * other.value,
                    self.z * other.value,
                )
            }
        }

        // Measure3d<U3> / Measure<U1> -> Measure3d<U2>
        impl<Number: ArithmeticOps> Div<Measure<#unit1, Number>> for Measure3d<#unit3, Number> {
            type Output = Measure3d<#unit2, Number>;
            fn div(self, other: Measure<#unit1, Number>) -> Self::Output {
                Self::Output::new(
                    self.x / other.value,
                    self.y / other.value,
                    self.z / other.value,
                )
            }
        }
    };
    expanded.into()
}

fn expand_inverse(unit1: syn::Ident, unit2: syn::Ident) -> TokenStream {
    let expanded = quote! {
        // Measure<U1> * Measure<U2> -> N
        impl<Number: ArithmeticOps> Mul<Measure<#unit2, Number>> for Measure<#unit1, Number> {
            type Output = Number;
            fn mul(self, other: Measure<#unit2, Number>) -> Self::Output {
                self.value * other.value
            }
        }

        // Measure<U2> * Measure<U1> -> N
        impl<Number: ArithmeticOps> Mul<Measure<#unit1, Number>> for Measure<#unit2, Number> {
            type Output = Number;
            fn mul(self, other: Measure<#unit1, Number>) -> Self::Output {
                self.value * other.value
            }
        }

        // N64 / Measure<U1> -> Measure<U2>
        impl<Number: ArithmeticOps> Div<Measure<#unit1, Number>> for f64 {
            type Output = Measure<#unit2, Number>;
            fn div(self, other: Measure<#unit1, Number>) -> Self::Output {
                Self::Output::new(Number::from_f64(self) / other.value)
            }
        }

        // N32 / Measure<U1> -> Measure<U2>
        impl<Number: ArithmeticOps> Div<Measure<#unit1, Number>> for f32 {
            type Output = Measure<#unit2, Number>;
            fn div(self, other: Measure<#unit1, Number>) -> Self::Output {
                Self::Output::new(Number::from_f64(self as f64) / other.value)
            }
        }

        // N64 / Measure<U2> -> Measure<U1>
        impl<Number: ArithmeticOps> Div<Measure<#unit2, Number>> for f64 {
            type Output = Measure<#unit1, Number>;
            fn div(self, other: Measure<#unit2, Number>) -> Self::Output {
                Self::Output::new(Number::from_f64(self) / other.value)
            }
        }

        // N32 / Measure<U2> -> Measure<U1>
        impl<Number: ArithmeticOps> Div<Measure<#unit2, Number>> for f32 {
            type Output = Measure<#unit1, Number>;
            fn div(self, other: Measure<#unit2, Number>) -> Self::Output {
                Self::Output::new(Number::from_f64(self as f64) / other.value)
            }
        }
        //TODO: try to avoid to duplicate the code for every supported numeric type
    };
    expanded.into()
}

fn expand_2_2(unit1: syn::Ident, unit2: syn::Ident, unit3: syn::Ident) -> TokenStream {
    let expanded = if unit1 == unit2 {
        quote! {
            // Measure2d<U1> * Measure2d<U1> -> Measure<U3>
            impl<Number: ArithmeticOps> Mul<Measure2d<#unit1, Number>> for Measure2d<#unit1, Number> {
                type Output = Measure<#unit3, Number>;
                fn mul(self, other: Measure2d<#unit1, Number>) -> Self::Output {
                    Self::Output::new(self.x * other.x + self.y * other.y)
                }
            }

            // Measure2d<U1>.squared() -> Measure<U3>
            impl<Number: ArithmeticOps> Measure2d<#unit1, Number> {
                fn squared(self) -> Measure<#unit3, Number> {
                    Measure::<#unit3, Number>::new(self.x * self.x + self.y * self.y)
                }
            }
        }
    } else {
        quote! {
            // Measure2d<U1> * Measure2d<U2> -> Measure<U3>
            impl<Number: ArithmeticOps> Mul<Measure2d<#unit2, Number>> for Measure2d<#unit1, Number> {
                type Output = Measure<#unit3, Number>;
                fn mul(self, other: Measure2d<#unit2, Number>) -> Self::Output {
                    Self::Output::new(self.x * other.x + self.y * other.y)
                }
            }

            // Measure2d<U2> * Measure2d<U1> -> Measure<U3>
            impl<Number: ArithmeticOps> Mul<Measure2d<#unit1, Number>> for Measure2d<#unit2, Number> {
                type Output = Measure<#unit3, Number>;
                fn mul(self, other: Measure2d<#unit1, Number>) -> Self::Output {
                    Self::Output::new(self.x * other.x + self.y * other.y)
                }
            }
        }
    };
    expanded.into()
}

fn expand_3_3(unit1: syn::Ident, unit2: syn::Ident, unit3: syn::Ident) -> TokenStream {
    let expanded = if unit1 == unit2 {
        quote! {
            // Measure3d<U1> * Measure3d<U1> -> Measure<U3>
            impl<Number: ArithmeticOps> Mul<Measure3d<#unit1, Number>> for Measure3d<#unit1, Number> {
                type Output = Measure<#unit3, Number>;
                fn mul(self, other: Measure3d<#unit1, Number>) -> Self::Output {
                    Self::Output::new(self.x * other.x + self.y * other.y + self.z * other.z)
                }
            }

            // Measure3d<U1>.squared() -> Measure<U3>
            impl<Number: ArithmeticOps> Measure3d<#unit1, Number> {
                fn squared(self) -> Measure<#unit3, Number> {
                    Measure::<#unit3, Number>::new(self.x * self.x + self.y * self.y + self.z * self.z)
                }
            }
        }
    } else {
        quote! {
            // Measure3d<U1> * Measure3d<U2> -> Measure<U3>
            impl<Number: ArithmeticOps> Mul<Measure3d<#unit2, Number>> for Measure3d<#unit1, Number> {
                type Output = Measure<#unit3, Number>;
                fn mul(self, other: Measure3d<#unit2, Number>) -> Self::Output {
                    Self::Output::new(self.x * other.x + self.y * other.y + self.z * other.z)
                }
            }

            // Measure3d<U2> * Measure3d<U1> -> Measure<U3>
            impl<Number: ArithmeticOps> Mul<Measure3d<#unit1, Number>> for Measure3d<#unit2, Number> {
                type Output = Measure<#unit3, Number>;
                fn mul(self, other: Measure3d<#unit1, Number>) -> Self::Output {
                    Self::Output::new(self.x * other.x + self.y * other.y + self.z * other.z)
                }
            }
        }
    };
    expanded.into()
}

fn expand_cross_2(unit1: syn::Ident, unit2: syn::Ident, unit3: syn::Ident) -> TokenStream {
    let expanded = if unit1 == unit2 {
        quote! {
            // Measure2d<U1>.cross_product(Measure2d<U1>) -> Measure<U3>
            impl<Number: ArithmeticOps> CrossProduct<Measure2d<#unit1, Number>> for Measure2d<#unit1, Number> {
                type Output = Measure<#unit3, Number>;
                fn cross_product(self, other: Measure2d<#unit1, Number>) -> Self::Output {
                    Self::Output::new(self.x * other.y - self.y * other.x)
                }
            }
        }
    } else {
        quote! {
            // Measure2d<U1>.cross_product(Measure2d<U2>) -> Measure<U3>
            impl<Number: ArithmeticOps> CrossProduct<Measure2d<#unit2, Number>> for Measure2d<#unit1, Number> {
                type Output = Measure<#unit3, Number>;
                fn cross_product(self, other: Measure2d<#unit2, Number>) -> Self::Output {
                    Self::Output::new(self.x * other.y - self.y * other.x)
                }
            }

            // Measure2d<U2>.cross_product(Measure2d<U1>) -> Measure<U3>
            impl<Number: ArithmeticOps> CrossProduct<Measure2d<#unit1, Number>> for Measure2d<#unit2, Number> {
                type Output = Measure<#unit3, Number>;
                fn cross_product(self, other: Measure2d<#unit1, Number>) -> Self::Output {
                    Self::Output::new(self.x * other.y - self.y * other.x)
                }
            }
        }
    };
    expanded.into()
}

fn expand_cross_3(unit1: syn::Ident, unit2: syn::Ident, unit3: syn::Ident) -> TokenStream {
    let expanded = if unit1 == unit2 {
        quote! {
            // Measure3d<U1>.cross_product(Measure3d<U1>) -> Measure<U3>
            impl<Number: ArithmeticOps> CrossProduct<Measure3d<#unit1, Number>> for Measure3d<#unit1, Number> {
                type Output = Measure3d<#unit3, Number>;
                fn cross_product(self, other: Measure3d<#unit1, Number>) -> Self::Output {
                    Self::Output::new(
                        self.y * other.z - self.z * other.y,
                        self.z * other.x - self.x * other.z,
                        self.x * other.y - self.y * other.x,
                    )
                }
            }
        }
    } else {
        quote! {
            // Measure3d<U1>.cross_product(Measure3d<U2>) -> Measure<U4>
            impl<Number: ArithmeticOps> CrossProduct<Measure3d<#unit2, Number>> for Measure3d<#unit1, Number> {
                type Output = Measure3d<#unit3, Number>;
                fn cross_product(self, other: Measure3d<#unit2, Number>) -> Self::Output {
                    Self::Output::new(
                        self.y * other.z - self.z * other.y,
                        self.z * other.x - self.x * other.z,
                        self.x * other.y - self.y * other.x,
                    )
                }
            }

            // Measure3d<U2>.cross_product(Measure3d<U1>) -> Measure<U4>
            impl<Number: ArithmeticOps> CrossProduct<Measure3d<#unit1, Number>> for Measure3d<#unit2, Number> {
                type Output = Measure3d<#unit3, Number>;
                fn cross_product(self, other: Measure3d<#unit1, Number>) -> Self::Output {
                    Self::Output::new(
                        self.y * other.z - self.z * other.y,
                        self.z * other.x - self.x * other.z,
                        self.x * other.y - self.y * other.x,
                    )
                }
            }
        }
    };
    expanded.into()
}

fn define_units_relation_impl(input: TokenStream) -> std::result::Result<TokenStream, syn::Error> {
    use UnitDimensions::{D1, D2, D3};
    use UnitsItem::{Id, One};
    use UnitsOperation::{Cross, Div, Eq, Mul};

    // The supported cases are these:
    // id1:1 == id2:1 * id3:1  =>  expand_1_1(id2, id3, id1)
    // id1:2 == id2:1 * id3:2  =>  expand_1_2(id2, id3, id1)
    // id1:2 == id2:2 * id3:1  =>  expand_1_2(id3, id2, id1)
    // id1:3 == id2:1 * id3:3  =>  expand_1_3(id2, id3, id1)
    // id1:3 == id2:3 * id3:1  =>  expand_1_3(id3, id2, id1)
    // id1:1 == id2:2 * id3:2  =>  expand_2_2(id2, id3, id1)
    // id1:1 == id2:3 * id3:3  =>  expand_3_3(id2, id3, id1)
    // id1:1 == id2:1 / id3:1  =>  expand_1_1(id3, id1, id2)
    // id1:2 == id2:2 / id3:1  =>  expand_1_2(id3, id1, id2)
    // id1:3 == id2:3 / id3:1  =>  expand_1_3(id3, id1, id2)
    // id1:1 == 1 / id3:1      =>  expand_inverse(id1, id3)
    // id1:1 == id2:2 X id3:2  =>  expand_cross_2(id2, id3, id1)
    // id1:3 == id2:3 X id3:3  =>  expand_cross_3(id2, id3, id1)
    Ok(match syn::parse::<UnitsRelation>(input)? {
        // id:1 == id:1 * id:1
        UnitsRelation {
            unit1: Id(id1),
            dim1: D1,
            operator1: Eq,
            unit2: Id(id2),
            dim2: D1,
            operator2: Mul,
            unit3: Id(id3),
            dim3: D1,
        } => expand_1_1(id2, id3, id1),

        // id:2 == id:1 * id:2
        UnitsRelation {
            unit1: Id(id1),
            dim1: D2,
            operator1: Eq,
            unit2: Id(id2),
            dim2: D1,
            operator2: Mul,
            unit3: Id(id3),
            dim3: D2,
        } => expand_1_2(id2, id3, id1),

        // id:2 == id:2 * id:1
        UnitsRelation {
            unit1: Id(id1),
            dim1: D2,
            operator1: Eq,
            unit2: Id(id2),
            dim2: D2,
            operator2: Mul,
            unit3: Id(id3),
            dim3: D1,
        } => expand_1_2(id3, id2, id1),

        // id:3 == id:1 * id:3
        UnitsRelation {
            unit1: Id(id1),
            dim1: D3,
            operator1: Eq,
            unit2: Id(id2),
            dim2: D1,
            operator2: Mul,
            unit3: Id(id3),
            dim3: D3,
        } => expand_1_3(id2, id3, id1),

        // id:3 == id:3 * id:1
        UnitsRelation {
            unit1: Id(id1),
            dim1: D3,
            operator1: Eq,
            unit2: Id(id2),
            dim2: D3,
            operator2: Mul,
            unit3: Id(id3),
            dim3: D1,
        } => expand_1_3(id3, id2, id1),

        // id:1 == id:2 * id:2
        UnitsRelation {
            unit1: Id(id1),
            dim1: D1,
            operator1: Eq,
            unit2: Id(id2),
            dim2: D2,
            operator2: Mul,
            unit3: Id(id3),
            dim3: D2,
        } => expand_2_2(id2, id3, id1),

        // id:1 == id:3 * id:3
        UnitsRelation {
            unit1: Id(id1),
            dim1: D1,
            operator1: Eq,
            unit2: Id(id2),
            dim2: D3,
            operator2: Mul,
            unit3: Id(id3),
            dim3: D3,
        } => expand_3_3(id2, id3, id1),

        // id:1 == id:1 / id:1
        UnitsRelation {
            unit1: Id(id1),
            dim1: D1,
            operator1: Eq,
            unit2: Id(id2),
            dim2: D1,
            operator2: Div,
            unit3: Id(id3),
            dim3: D1,
        } => expand_1_1(id3, id1, id2),

        // id:2 == id:2 / id:1
        UnitsRelation {
            unit1: Id(id1),
            dim1: D2,
            operator1: Eq,
            unit2: Id(id2),
            dim2: D2,
            operator2: Div,
            unit3: Id(id3),
            dim3: D1,
        } => expand_1_2(id3, id1, id2),

        // id:3 == id:3 / id:1
        UnitsRelation {
            unit1: Id(id1),
            dim1: D3,
            operator1: Eq,
            unit2: Id(id2),
            dim2: D3,
            operator2: Div,
            unit3: Id(id3),
            dim3: D1,
        } => expand_1_3(id3, id1, id2),

        // id:1 == 1 / id:1
        UnitsRelation {
            unit1: Id(id1),
            dim1: D1,
            operator1: Eq,
            unit2: One,
            dim2: _,
            operator2: Div,
            unit3: Id(id3),
            dim3: D1,
        } => expand_inverse(id1, id3),

        // id:1 == id:2 X id:2
        UnitsRelation {
            unit1: Id(id1),
            dim1: D1,
            operator1: Eq,
            unit2: Id(id2),
            dim2: D2,
            operator2: Cross,
            unit3: Id(id3),
            dim3: D2,
        } => expand_cross_2(id2, id3, id1),

        // id:3 == id:3 X id:3
        UnitsRelation {
            unit1: Id(id1),
            dim1: D3,
            operator1: Eq,
            unit2: Id(id2),
            dim2: D3,
            operator2: Cross,
            unit3: Id(id3),
            dim3: D3,
        } => expand_cross_3(id2, id3, id1),

        _ => {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "bad expression",
            ))
        }
    })
}
