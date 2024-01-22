#[macro_export]
macro_rules! define_1d_and_directions {
    {} => {
        rs_measures::define_1d!();
        rs_measures::inner_define_unsigned_direction!();
        rs_measures::inner_define_signed_direction!();
    };
}
