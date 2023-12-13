#[cfg(test)]
mod test {
    struct Length;
    struct Metre;
    impl MeasurementUnit for Metre {
        type Property = Length;
        const RATIO: f64 = 1.;
        const OFFSET: f64 = 0.;
        const SUFFIX: &'static str = " m";
    }

    #[test]
    fn measure_value() {
        let m = Measure::<f32, Metre>::new(1.23);
        assert_eq!(m.value, 1.238);
    }

    /*
        MEASURES_ANGLE_UNIT(degrees, "^", 360, 0)
        MEASURES_ANGLE_UNIT(turns, " rev", 1, 0)
        #define AZIMUTH_TOLERANCE 0.00002f
        #define MIN_THRESHOLD 1e-14
        #define MAX_THRESHOLD 1e14

        MEASURES_UNIT(km, Length, " Km", 1000, 0)
        MEASURES_UNIT(inches, Length, "\"", 0.0254, 0)

        MEASURES_MAGNITUDE(Time, seconds, " s")
        MEASURES_UNIT(hours, Time, " h", 3600, 0)
        MEASURES_UNIT(days, Time, " d", 86400, 0)

        MEASURES_MAGNITUDE(Speed, MetrePerSecond, " m/s")
        MEASURES_UNIT(km_per_hour, Speed, " Km/h", 1 / 3.6, 0)
        MEASURES_UNIT(inches_per_day, Speed, "\"/day", 86400 / 0.0254, 0)

        MEASURES_MAGNITUDE(Area, SquareMetre, " m2")
        MEASURES_UNIT(square_km, Area, " Km2", 1000000, 0)
        MEASURES_UNIT(square_inches, Area, "\"2", 0.0254 * 0.0254, 0)

        MEASURES_MAGNITUDE(Temperature, kelvin, "^K")
        MEASURES_UNIT(celsius, Temperature, "^C", 1, 273.15)
        MEASURES_UNIT(fahrenheit, Temperature, "^F", 5. / 9., 273.15 - 32. * 5. / 9.)

        MEASURES_MAGNITUDE(Volume, CubicMetre, " m3")
        MEASURES_MAGNITUDE(Density, kg_per_cubic_metre, " Kg/m3")
        MEASURES_MAGNITUDE(Mass, kg, " Kg")
        MEASURES_MAGNITUDE(Force, newtons, " N")
        MEASURES_MAGNITUDE(Energy, joules, " J")
        MEASURES_MAGNITUDE(Torque, NewtonMetre, " Nm")
        MEASURES_MAGNITUDE(Unitless, units, " u.")
        MEASURES_UNIT(dozens, Unitless, " doz.", 12, 0)
        MEASURES_MAGNITUDE(MagneticField, tesla, " T")
        MEASURES_MAGNITUDE(ElectricField, volts_per_metre, " V/m")

        MEASURES_DERIVED_1_1(CubicMetre, kg_per_cubic_metre, kg)
        MEASURES_DERIVED_SQ_1(Metre, SquareMetre)
        MEASURES_DERIVED_SQ_2(Metre, SquareMetre, SquareMetre)
        MEASURES_DERIVED_SQ_3(Metre, CubicMetre, CubicMetre)
        MEASURES_DERIVED_1_3(seconds, MetrePerSecond, Metre)
        MEASURES_DERIVED_1_1(newtons, Metre, joules)
        MEASURES_DERIVED_2_2(newtons, Metre, joules, NewtonMetre)
        MEASURES_DERIVED_3_3(newtons, metres, joules, NewtonMetre)
        MEASURES_DERIVED_1_1(MetrePerSecond, tesla, volts_per_metre)
        MEASURES_DERIVED_2_2(MetrePerSecond, tesla, volts_per_metre, volts_per_metre)
        MEASURES_DERIVED_3_3(MetrePerSecond, tesla, volts_per_metre, volts_per_metre)
        MEASURES_DERIVED_SQ_1(units, units)
        MEASURES_DERIVED_SQ_2(units, units, units)
        MEASURES_DERIVED_SQ_3(units, units, units)

        MEASURES_DERIVED_SQ_1(inches, square_inches)
        MEASURES_DERIVED_1_1(hours, km_per_hour, km)
        MEASURES_DERIVED_1_2(hours, km_per_hour, km)
        MEASURES_DERIVED_1_3(hours, km_per_hour, km)


        template <typename ValueType, class MeasureType>
        bool equality(ValueType expected_value, MeasureType actual_measure)
        {
            typedef decltype(abs(typename MeasureType::value_type())) val_type;
            auto tolerance = numeric_limits<val_type>::is_integer ?
                1 : 512 * numeric_limits<val_type>::epsilon();
            auto rel_diff = abs(
                (static_cast<typename MeasureType::value_type>(expected_value) -
                actual_measure.value())
                / (abs(expected_value) == 0
                ? static_cast<decltype(expected_value)>(1) : expected_value));
            return rel_diff < tolerance;
        }

        #define EQUAL(expected, actual) EXPECT_TRUE(equality(expected, actual));

        template <typename Num1, typename Num2>
        decltype(Num1()/Num2()) modulo(Num1 a, Num2 b)
        {
          const auto result = fmod(a, b);
          return result >= 0 ? result : result + b;
        }

        // These are all the supported inner types
        // that support any operation.
        template <class T>
        class fractional_test: public testing::Test { };
        typedef testing::Types<float,double,long double>
            FractionalImplementations;
        TYPED_TEST_CASE(fractional_test, FractionalImplementations);

        // These are all the supported inner types
        // that may not support angles and maps.
        template <class T>
        class comparison_test: public testing::Test { };
        typedef testing::Types<int,long,long long,float,double,long double>
            ComparableImplementations;
        TYPED_TEST_CASE(comparison_test, ComparableImplementations);

        // These are all the supported inner types
        // that may not support comparison operations, angle operations, and mappings.
        template <class T>
        class general_test: public testing::Test { };
        typedef testing::Types<int,long,long long,float,double,long double,
            complex<float>,complex<double>,complex<long double>> AllImplementations;
        TYPED_TEST_CASE(general_test, AllImplementations);

        template <typename T>
        struct test_values
        {
            static int const count = 11;
            static T const v[count];
        };

        template <typename T>
        T const test_values<T>::v[] = {
            numeric_limits<T>::lowest() + static_cast<T>(1),
            (numeric_limits<T>::lowest() + static_cast<T>(1)) / static_cast<T>(319),
            -static_cast<T>(std::sqrt(numeric_limits<T>::max() / static_cast<T>(10))),
            static_cast<T>(-1) - numeric_limits<T>::epsilon(),
            numeric_limits<T>::is_integer ? -1 : -numeric_limits<T>::min(),
            0,
            numeric_limits<T>::is_integer ? 1 : numeric_limits<T>::min(),
            static_cast<T>(1) + numeric_limits<T>::epsilon(),
            static_cast<T>(std::sqrt(numeric_limits<T>::max() / static_cast<T>(10))),
            numeric_limits<T>::max() / static_cast<T>(319),
            numeric_limits<T>::max(),
        };
    */

    /*
        #[test]
        {
            assert_eq!(Degree::turn_fraction<int>(), 360);
            assert_eq!(Radian::TURN_FRACTION, 2. * 3.141592653589793238462643383279);
            assert_eq!(Turn::TURN_FRACTION, 1);
            assert_eq!(111, 222);
        }
    */
    /*
    TYPED_TEST(general_test, vect1)
    {
        auto epsilon = numeric_limits<TypeParam>::is_integer ?
            1 : numeric_limits<float>::epsilon();
        auto tolerance = Measure<f32, Metre>(epsilon * 2);
        auto zero = Measure<f32, Metre>(0);

        // Try several numeric values.
        for (int i = 0; i < test_values<TypeParam>::count; ++i)
        {
            TypeParam val1 = test_values<TypeParam>::v[i];
            TypeParam val2 = static_cast<TypeParam>(2.19);
            TypeParam val3 = 3;
            auto m1 = Measure<TypeParam, Metre>(val1);
            auto m2 = Measure<TypeParam, Metre>(val2);

            // Construction and "value".
            EXPECT_EQ(val1, m1.value());
            EXPECT_EQ(val2, m2.value());

            // Operator +=.
            // All test-values except the largest one,
            // as it would cause overflow by incrementing.
            if (i < test_values<TypeParam>::count - 1)
            {
                m1 = Measure<TypeParam, Metre>(val1);
                auto m3 = m1 += m2;
                EXPECT_EQ(val1 + val2, m1.value());
                EXPECT_EQ(val2, m2.value());
                EXPECT_EQ(val1 + val2, m3.value());
            }

            // Operator -=.
            // All test-values except the lowest one,
            // as it would cause overflow by decrementing.
            if (0 < i)
            {
                m1 = Measure<TypeParam, Metre>(val1);
                auto m3 = m1 -= m2;
                EXPECT_EQ(val1 - val2, m1.value());
                EXPECT_EQ(val2, m2.value());
                EXPECT_EQ(val1 - val2, m3.value());
            }

            // Operator *=.
            // All test-values except the lowest and the largest ones,
            // as they would cause overflow by multiplying.
            if (0 < i && i < test_values<TypeParam>::count - 1)
            {
                m1 = Measure<TypeParam, Metre>(val1);
                auto m3 = m1 *= val2;
                EQUAL(val1 * val2, m1)
                EQUAL(val1 * val2, m3)
                auto m4 = m1 *= val3;
                EQUAL(val1 * val2 * val3, m1)
                EQUAL(val1 * val2 * val3, m4)
            }

            // Operator /=.
            // All test-values.
            if (abs(val2) != 0)
            {
                m1 = Measure<TypeParam, Metre>(val1);
                auto m3 = m1 /= val2;
                EXPECT_EQ(val1 / val2, m1.value());
                EXPECT_EQ(val1 / val2, m3.value());
                if (abs(val3) != 0)
                {
                    auto m4 = m1 /= val3;
                    EQUAL(val1 / val2 / val3, m1)
                    EQUAL(val1 / val2 / val3, m4)
                }
            }

            // Relational operators.

            // Comparing equal measures.
            m1 = Measure<TypeParam, Metre>(val1);
            m2 = m1;
            EXPECT_TRUE(m1 == m2);
            EXPECT_FALSE(m1 != m2);
            EXPECT_TRUE(is_equal(m1, m2, zero));

            // Comparing equal measures with a tolerance.
            // Avoid extreme test-values with a non-zero tolerance,
            // as they would cause overflow by incrementing or decrementing.
            if (0 < i && i < test_values<TypeParam>::count - 1)
            {
                EXPECT_TRUE(is_equal(m1, m2, tolerance));
            }

            // Comparing a measure with its half, if it is large,
            // with its double if it is small.
            if (abs(m1.value()) > 1) m2 /= 2;
            else m1 *= 2;

            // Ignore zero case, as its half is the same as itself.
            if (abs(m2.value()) != 0)
            {
                EXPECT_FALSE(m1 == m2);
                EXPECT_TRUE(m1 != m2);
            }
        }

        // Comparing 1 with a bit more than 1.
        auto m3 = Measure<TypeParam, Metre>(1);
        auto m4 = Measure<TypeParam, Metre>(1 + epsilon);
        EXPECT_FALSE(m3 == m4);
        EXPECT_TRUE(m3 != m4);
        EXPECT_TRUE(is_equal(m3, m4, tolerance));

        // Comparing -1 with a bit less than -1.
        auto m5 = Measure<TypeParam, Metre>(-1);
        auto m6 = Measure<TypeParam, Metre>(-1 - epsilon);
        EXPECT_FALSE(m5 == m6);
        EXPECT_TRUE(m5 != m6);
        EXPECT_TRUE(is_equal(m5, m6, tolerance));

        // Compare non-extreme numbers, far from zero,
        // with a small tolerance or with a large tolerance.
        auto m7 = Measure<TypeParam, Metre>(45);
        auto m8 = Measure<TypeParam, Metre>(48);
        auto tol1 = Measure<f32, Metre>(2);
        auto tol2 = Measure<f32, Metre>(4);
        EXPECT_FALSE(is_equal(m7, m8, tol1));
        EXPECT_FALSE(is_equal(m8, m7, tol1));
        EXPECT_TRUE(is_equal(m7, m8, tol2));
        EXPECT_TRUE(is_equal(m8, m7, tol2));
    }

    TYPED_TEST(comparison_test, vect1)
    {
        auto epsilon = numeric_limits<TypeParam>::is_integer ?
            1 : numeric_limits<float>::epsilon();
        auto tolerance = Measure<f32, Metre>(epsilon * 2);
        auto zero = Measure<f32, Metre>(0);

        // Try several numeric values.
        for (int i = 0; i < test_values<TypeParam>::count; ++i)
        {
            TypeParam val1 = test_values<TypeParam>::v[i];

            // Relational operators.

            // Comparing equal measures.
            auto m1 = Measure<TypeParam, Metre>(val1);
            auto m2 = m1;
            EXPECT_FALSE(m1 < m2);
            EXPECT_TRUE(m1 <= m2);
            EXPECT_FALSE(m1 > m2);
            EXPECT_TRUE(m1 >= m2);
            EXPECT_FALSE(is_less(m1, m2, zero));
            EXPECT_TRUE(is_less_or_equal(m1, m2, zero));

            // Comparing equal measures with a tolerance.
            // Avoid extreme test-values with a non-zero tolerance,
            // as they would cause overflow by incrementing or decrementing.
            if (0 < i && i < test_values<TypeParam>::count - 1)
            {
                EXPECT_FALSE(is_less(m1, m2, tolerance));
                EXPECT_TRUE(is_less_or_equal(m1, m2, tolerance));
            }

            // Comparing a measure with its half, if it is large,
            // with its double if it is small.
            if (abs(m1.value()) > 1) m2 /= 2;
            else m1 *= 2;

            // Consider separately positive and negative cases.
            // Ignore zero case, as its half is the same as itself.
            if (m2.value() > 0)
            {
                EXPECT_FALSE(m1 < m2);
                EXPECT_FALSE(m1 <= m2);
                EXPECT_TRUE(m1 > m2);
                EXPECT_TRUE(m1 >= m2);
                EXPECT_FALSE(is_equal(m1, m2, zero));
                EXPECT_FALSE(is_less(m1, m2, zero));
                EXPECT_FALSE(is_less_or_equal(m1, m2, zero));
            }
            else if (m2.value() < 0)
            {
                EXPECT_TRUE(m1 < m2);
                EXPECT_TRUE(m1 <= m2);
                EXPECT_FALSE(m1 > m2);
                EXPECT_FALSE(m1 >= m2);
                EXPECT_FALSE(is_equal(m1, m2, zero));
                EXPECT_TRUE(is_less(m1, m2, zero));
                EXPECT_TRUE(is_less_or_equal(m1, m2, zero));
            }
        }

        // Comparing 1 with a bit more than 1.
        auto m3 = Measure<TypeParam, Metre>(1);
        auto m4 = Measure<TypeParam, Metre>(1 + epsilon);
        EXPECT_TRUE(m3 < m4);
        EXPECT_TRUE(m3 <= m4);
        EXPECT_FALSE(m3 > m4);
        EXPECT_FALSE(m3 >= m4);
        EXPECT_TRUE(is_equal(m3, m4, tolerance));
        EXPECT_FALSE(is_less(m3, m4, tolerance));
        EXPECT_TRUE(is_less_or_equal(m3, m4, tolerance));

        // Comparing -1 with a bit less than -1.
        auto m5 = Measure<TypeParam, Metre>(-1);
        auto m6 = Measure<TypeParam, Metre>(-1 - epsilon);
        EXPECT_FALSE(m5 < m6);
        EXPECT_FALSE(m5 <= m6);
        EXPECT_TRUE(m5 > m6);
        EXPECT_TRUE(m5 >= m6);
        EXPECT_TRUE(is_equal(m5, m6, tolerance));
        EXPECT_FALSE(is_less(m5, m6, tolerance));
        EXPECT_TRUE(is_less_or_equal(m5, m6, tolerance));

        // Compare non-extreme numbers, far from zero,
        // with a small tolerance or with a large tolerance.
        auto m7 = Measure<TypeParam, Metre>(45);
        auto m8 = Measure<TypeParam, Metre>(48);
        auto tol1 = Measure<f32, Metre>(2);
        auto tol2 = Measure<f32, Metre>(4);

        EXPECT_TRUE(is_less(m7, m8, tol1));
        EXPECT_FALSE(is_less(m8, m7, tol1));
        EXPECT_FALSE(is_less(m7, m8, tol2));
        EXPECT_FALSE(is_less(m8, m7, tol2));

        EXPECT_TRUE(is_less_or_equal(m7, m8, tol1));
        EXPECT_FALSE(is_less_or_equal(m8, m7, tol1));
        EXPECT_TRUE(is_less_or_equal(m7, m8, tol2));
        EXPECT_TRUE(is_less_or_equal(m8, m7, tol2));
    }

    TYPED_TEST(general_test, point1)
    {
        auto epsilon = numeric_limits<TypeParam>::is_integer ?
            1 : numeric_limits<float>::epsilon();
        auto tolerance = Measure<f32, Metre>(epsilon * 2);
        auto zero = Measure<f32, Metre>(0);

        // Try several numeric values.
        for (int i = 0; i < test_values<TypeParam>::count; ++i)
        {
            TypeParam val1 = test_values<TypeParam>::v[i];
            TypeParam val2 = static_cast<TypeParam>(2.19);
            auto m1 = MeasurePoint<TypeParam, Metre>(val1);
            auto m2 = Measure<TypeParam, Metre>(val2);

            // Construction and "value".
            EXPECT_EQ(val1, m1.value());
            EXPECT_EQ(val2, m2.value());

            // Operator +=.
            // All test-values except the largest one,
            // as it would cause overflow by incrementing.
            if (i < test_values<TypeParam>::count - 1)
            {
                m1 = MeasurePoint<TypeParam, Metre>(val1);
                auto m3 = m1 += m2;
                EXPECT_EQ(val1 + val2, m1.value());
                EXPECT_EQ(val1 + val2, m3.value());
            }

            // Operator -=.
            // All test-values except the lowest one,
            // as it would cause overflow by decrementing.
            if (0 < i)
            {
                m1 = MeasurePoint<TypeParam, Metre>(val1);
                auto m3 = m1 -= m2;
                EXPECT_EQ(val1 - val2, m1.value());
                EXPECT_EQ(val1 - val2, m3.value());
            }

            // Relational operators.

            // Comparing equal measures.
            m1 = MeasurePoint<TypeParam, Metre>(val1);
            auto m3 = m1;
            EXPECT_TRUE(m1 == m3);
            EXPECT_FALSE(m1 != m3);
            EXPECT_TRUE(is_equal(m1, m3, zero));

            // Comparing equal measures with a tolerance.
            // Avoid extreme test-values with a non-zero tolerance,
            // as they would cause overflow by incrementing or decrementing.
            if (0 < i && i < test_values<TypeParam>::count - 1)
            {
                EXPECT_TRUE(is_equal(m1, m3, tolerance));
            }

            // Comparing a measure with its half, if it is large,
            // with its double if it is small.
            m3 = MeasurePoint<TypeParam, Metre>(abs(m1.value()) > 1 ?
                val1 / static_cast<TypeParam>(2) : val1 * static_cast<TypeParam>(2));

            // Ignore zero case, as its half is the same as itself.
            if (abs(m3.value()) != 0)
            {
                EXPECT_FALSE(m1 == m3);
                EXPECT_TRUE(m1 != m3);
            }
        }

        // Comparing 1 with a bit more than 1.
        auto m3 = MeasurePoint<TypeParam, Metre>(1);
        auto m4 = MeasurePoint<TypeParam, Metre>(1 + epsilon);
        EXPECT_FALSE(m3 == m4);
        EXPECT_TRUE(m3 != m4);
        EXPECT_TRUE(is_equal(m3, m4, tolerance));

        // Comparing -1 with a bit less than -1.
        auto m5 = MeasurePoint<TypeParam, Metre>(-1);
        auto m6 = MeasurePoint<TypeParam, Metre>(-1 - epsilon);
        EXPECT_FALSE(m5 == m6);
        EXPECT_TRUE(m5 != m6);
        EXPECT_TRUE(is_equal(m5, m6, tolerance));

        // Compare non-extreme numbers, far from zero,
        // with a small tolerance or with a large tolerance.
        auto m7 = MeasurePoint<TypeParam, Metre>(45);
        auto m8 = MeasurePoint<TypeParam, Metre>(48);
        auto tol1 = Measure<f32, Metre>(2);
        auto tol2 = Measure<f32, Metre>(4);
        EXPECT_FALSE(is_equal(m7, m8, tol1));
        EXPECT_FALSE(is_equal(m8, m7, tol1));
        EXPECT_TRUE(is_equal(m7, m8, tol2));
        EXPECT_TRUE(is_equal(m8, m7, tol2));
    }

    TYPED_TEST(comparison_test, point1)
    {
        auto epsilon = numeric_limits<TypeParam>::is_integer ?
            1 : numeric_limits<float>::epsilon();
        auto tolerance = Measure<f32, Metre>(epsilon * 2);
        auto zero = Measure<f32, Metre>(0);

        // Try several numeric values.
        for (int i = 0; i < test_values<TypeParam>::count; ++i)
        {
            TypeParam val1 = test_values<TypeParam>::v[i];
            TypeParam val2 = static_cast<TypeParam>(2.19);
            auto m1 = MeasurePoint<TypeParam, Metre>(val1);
            auto m2 = Measure<TypeParam, Metre>(val2);

            // Construction and "value".
            EXPECT_EQ(val1, m1.value());
            EXPECT_EQ(val2, m2.value());

            // Operator +=.
            // All test-values except the largest one,
            // as it would cause overflow by incrementing.
            if (i < test_values<TypeParam>::count - 1)
            {
                m1 = MeasurePoint<TypeParam, Metre>(val1);
                auto m3 = m1 += m2;
                EXPECT_EQ(val1 + val2, m1.value());
                EXPECT_EQ(val1 + val2, m3.value());
            }

            // Operator -=.
            // All test-values except the lowest one,
            // as it would cause overflow by decrementing.
            if (0 < i)
            {
                m1 = MeasurePoint<TypeParam, Metre>(val1);
                auto m3 = m1 -= m2;
                EXPECT_EQ(val1 - val2, m1.value());
                EXPECT_EQ(val1 - val2, m3.value());
            }

            // Relational operators.

            // Comparing equal measures.
            m1 = MeasurePoint<TypeParam, Metre>(val1);
            auto m3 = m1;
            EXPECT_TRUE(m1 == m3);
            EXPECT_FALSE(m1 != m3);
            EXPECT_FALSE(m1 < m3);
            EXPECT_TRUE(m1 <= m3);
            EXPECT_FALSE(m1 > m3);
            EXPECT_TRUE(m1 >= m3);
            EXPECT_TRUE(is_equal(m1, m3, zero));
            EXPECT_FALSE(is_less(m1, m3, zero));
            EXPECT_TRUE(is_less_or_equal(m1, m3, zero));

            // Comparing equal measures with a tolerance.
            // Avoid extreme test-values with a non-zero tolerance,
            // as they would cause overflow by incrementing or decrementing.
            if (0 < i && i < test_values<TypeParam>::count - 1)
            {
                EXPECT_TRUE(is_equal(m1, m3, tolerance));
                EXPECT_FALSE(is_less(m1, m3, tolerance));
                EXPECT_TRUE(is_less_or_equal(m1, m3, tolerance));
            }

            // Comparing a measure with its half.
            m3 = MeasurePoint<TypeParam, Metre>(val1 / static_cast<TypeParam>(2));

            // Consider separately positive and negative cases.
            // Ignore zero case, as its half is the same as itself.
            if (m1.value() > 0)
            {
                EXPECT_FALSE(m1 == m3);
                EXPECT_TRUE(m1 != m3);
                EXPECT_FALSE(m1 < m3);
                EXPECT_FALSE(m1 <= m3);
                EXPECT_TRUE(m1 > m3);
                EXPECT_TRUE(m1 >= m3);
                EXPECT_FALSE(is_equal(m1, m3, zero));
                EXPECT_FALSE(is_less(m1, m3, zero));
                EXPECT_FALSE(is_less_or_equal(m1, m3, zero));
            }
            else if (m1.value() > 0)
            {
                EXPECT_FALSE(m1 == m3);
                EXPECT_TRUE(m1 != m3);
                EXPECT_TRUE(m1 < m3);
                EXPECT_TRUE(m1 <= m3);
                EXPECT_FALSE(m1 > m3);
                EXPECT_FALSE(m1 >= m3);
                EXPECT_FALSE(is_equal(m1, m3, zero));
                EXPECT_FALSE(is_less(m1, m3, zero));
                EXPECT_FALSE(is_less_or_equal(m1, m3, zero));
            }
        }

        // Comparing 1 with a bit more than 1.
        auto m3 = MeasurePoint<TypeParam, Metre>(1);
        auto m4 = MeasurePoint<TypeParam, Metre>(1 + epsilon);
        EXPECT_FALSE(m3 == m4);
        EXPECT_TRUE(m3 != m4);
        EXPECT_TRUE(m3 < m4);
        EXPECT_TRUE(m3 <= m4);
        EXPECT_FALSE(m3 > m4);
        EXPECT_FALSE(m3 >= m4);
        EXPECT_TRUE(is_equal(m3, m4, tolerance));
        EXPECT_FALSE(is_less(m3, m4, tolerance));
        EXPECT_TRUE(is_less_or_equal(m3, m4, tolerance));

        // Comparing -1 with a bit less than -1.
        auto m5 = MeasurePoint<TypeParam, Metre>(-1);
        auto m6 = MeasurePoint<TypeParam, Metre>(-1 - epsilon);
        EXPECT_FALSE(m5 == m6);
        EXPECT_TRUE(m5 != m6);
        EXPECT_FALSE(m5 < m6);
        EXPECT_FALSE(m5 <= m6);
        EXPECT_TRUE(m5 > m6);
        EXPECT_TRUE(m5 >= m6);
        EXPECT_TRUE(is_equal(m5, m6, tolerance));
        EXPECT_FALSE(is_less(m5, m6, tolerance));
        EXPECT_TRUE(is_less_or_equal(m5, m6, tolerance));

        // Compare non-extreme numbers, far from zero,
        // with a small tolerance or with a large tolerance.
        auto m7 = MeasurePoint<TypeParam, Metre>(45);
        auto m8 = MeasurePoint<TypeParam, Metre>(48);
        auto tol1 = Measure<f32, Metre>(2);
        auto tol2 = Measure<f32, Metre>(4);
        EXPECT_FALSE(is_equal(m7, m8, tol1));
        EXPECT_FALSE(is_equal(m8, m7, tol1));
        EXPECT_TRUE(is_equal(m7, m8, tol2));
        EXPECT_TRUE(is_equal(m8, m7, tol2));

        EXPECT_TRUE(is_less(m7, m8, tol1));
        EXPECT_FALSE(is_less(m8, m7, tol1));
        EXPECT_FALSE(is_less(m7, m8, tol2));
        EXPECT_FALSE(is_less(m8, m7, tol2));

        EXPECT_TRUE(is_less_or_equal(m7, m8, tol1));
        EXPECT_FALSE(is_less_or_equal(m8, m7, tol1));
        EXPECT_TRUE(is_less_or_equal(m7, m8, tol2));
        EXPECT_TRUE(is_less_or_equal(m8, m7, tol2));
    }

    TYPED_TEST(general_test, vectpoint1)
    {
        // Midpoint.
        // Try all pairs of numeric values except extremes.
        auto fraction = abs(static_cast<TypeParam>(0.23f));
        for (int i1 = 1; i1 < test_values<TypeParam>::count - 1; ++i1)
        {
            TypeParam val1 = test_values<TypeParam>::v[i1];
            auto m1 = MeasurePoint<TypeParam, Metre>(val1);
            for (int i2 = 1; i2 < test_values<TypeParam>::count - 1; ++i2)
            {
                TypeParam val2 = test_values<TypeParam>::v[i2];
                auto m2 = MeasurePoint<TypeParam, Metre>(val2);
                EQUAL((val1 + val2) / static_cast<TypeParam>(2), midpoint(m1, m2))
                EQUAL(val1, midpoint(m1, m2, 0))
                EQUAL(val2, midpoint(m1, m2, 1))
                EQUAL(val1 * (1 - fraction) + val2 * fraction,
                    midpoint(m1, m2, fraction))
            }
        }

        // Barycentric combination.
        // Try all pairs of numeric values except extremes,
        // and add some other points.
        for (int i1 = 1; i1 < test_values<TypeParam>::count - 1; ++i1)
        {
            TypeParam val1 = test_values<TypeParam>::v[i1];
            auto m1 = MeasurePoint<TypeParam, Metre>(val1);
            for (int i2 = 1; i2 < test_values<TypeParam>::count - 1; ++i2)
            {
                TypeParam val2 = test_values<TypeParam>::v[i2];
                auto m2 = MeasurePoint<TypeParam, Metre>(val2);
                TypeParam val3 = 3;
                auto m3 = MeasurePoint<TypeParam, Metre>(val3);
                TypeParam val4 = 4;
                auto m4 = MeasurePoint<TypeParam, Metre>(val4);
                MeasurePoint<TypeParam, Metre> point1array[] = { m1, m2, m3, m4 };
                TypeParam weights[] = { 2, 3, 7, 4 };
                EQUAL(val1 * weights[0],
                    barycentric_combination(1, point1array, weights))
                EQUAL(val1 * weights[0] + val2 * weights[1],
                    barycentric_combination(2, point1array, weights))
                EQUAL(val1 * weights[0] + val2 * weights[1]
                    + val3 * weights[2],
                    barycentric_combination(3, point1array, weights))
                EQUAL(val1 * weights[0] + val2 * weights[1]
                    + val3 * weights[2] + val4 * weights[3],
                    barycentric_combination(4, point1array, weights))
            }
        }

        // Try all pairs of numeric values except extremes.
        auto sqrt_of_max = abs(static_cast<TypeParam>(std::sqrt(test_values<TypeParam>
            ::v[test_values<TypeParam>::count - 1])));
        for (int i1 = 1; i1 < test_values<TypeParam>::count - 1; ++i1)
        {
            TypeParam val1 = test_values<TypeParam>::v[i1];
            auto v1 = Measure<TypeParam, Metre>(val1);
            auto p1 = MeasurePoint<TypeParam, Metre>(val1);
            for (int i2 = 1; i2 < test_values<TypeParam>::count - 1; ++i2)
            {
                TypeParam val2 = test_values<TypeParam>::v[i2];
                auto v2 = Measure<TypeParam, Metre>(val2);
                auto p2 = MeasurePoint<TypeParam, Metre>(val2);

                // point1 - point1 -> vect1
                EXPECT_EQ(val1 - val2, (p1 - p2).value());

                // point1 + vect1 -> point1
                EXPECT_EQ(val1 + val2, (p1 + v2).value());

                // point1 - vect1 -> point1
                EXPECT_EQ(val1 - val2, (p1 - v2).value());

                // vect1 + vect1 -> vect1
                EXPECT_EQ(val1 + val2, (v1 + v2).value());

                // vect1 - vect1 -> vect1
                EXPECT_EQ(val1 - val2, (v1 - v2).value());

                if (abs(val1) < sqrt_of_max && abs(val2) < sqrt_of_max)
                {
                    // N * vect1 -> vect1
                    EXPECT_EQ(val1 * val2, (val1 * v2).value());

                    // vect1 * N -> vect1
                    EXPECT_EQ(val1 * val2, (v1 * val2).value());
                }

                // vect1 / N -> vect1
                if (abs(val2) != 0)
                {
                    EXPECT_EQ(val1 / val2, (v1 / val2).value());
                }

                // vect1 / vect1 -> N
                if (abs(val2) != 0)
                {
                    EXPECT_EQ(val1 / val2, v1 / v2);
                }
            }
        }

        for (int i = 2; i < test_values<TypeParam>::count - 2; ++i)
        {
            TypeParam val1 = test_values<TypeParam>::v[i];
            auto m1 = Measure<TypeParam, Metre>(val1);
            if (abs(val1) < sqrt_of_max)
            {
                EQUAL(val1 * val1, m1 * m1);
            }
            EQUAL(abs(val1), norm(m1));
        }
    }

    //////////////////////////////////////////////////////////////////////
    TYPED_TEST(fractional_test, linear_map2)
    {
        long double x = 17.8L;
        long double y = 13.3L;
        Measure2d<TypeParam, Metre> v1(x, y);

        { //// Rotation ////
            auto v2 = v1;
            auto v3 = v1;
            Measure<degrees,long double> ang(36.L);

            // Create two rotators, presumed equal.
            auto lm1 = linear_map2<TypeParam>::rotation(ang);
            auto lm2 = make_rotation(ang);

            // Apply 10 rotations by 1/10 of a turn
            // by using both rotators.
            for (int i = 0; i < 10; ++i)
            {
                v2 = v2.mapped_by(lm1);
                v3 = v3.mapped_by(lm2);
            }

            // The resulting vectors should be equal to the original one.
            EQUAL(x, v2.x())
            EQUAL(y, v2.y())
            EQUAL(x, v3.x())
            EQUAL(y, v3.y())
        }

        { //// Rotation at right ////
            auto v2 = v1.mapped_by(linear_map2<TypeParam>::rotation_at_right());
            auto v3 = v1.mapped_by(make_rotation_at_right<TypeParam>());

            // The resulting vectors should have x equal to the original y,
            // and y equal to the opposite of the original x.
            EQUAL(y, v2.x())
            EQUAL(-x, v2.y())
            EQUAL(y, v3.x())
            EQUAL(-x, v3.y())

            // The resulting vectors should be orthogonal to the original one.
            EQUAL(0, v2 * v1)
            EQUAL(0, v3 * v1)
        }

        { //// Rotation at left ////
            auto v2 = v1.mapped_by(linear_map2<TypeParam>::rotation_at_left());
            auto v3 = v1.mapped_by(make_rotation_at_left<TypeParam>());

            // The resulting vectors should have x equal to the opposite
            // of the original y, and y equal to the original x.
            EQUAL(-y, v2.x())
            EQUAL(x, v2.y())
            EQUAL(-y, v3.x())
            EQUAL(x, v3.y())

            // The resulting vectors should be orthogonal to the original one.
            EQUAL(0, v2 * v1)
            EQUAL(0, v3 * v1)
        }

        { //// Projection ////
            long double const ang_val = 0.9L;
            MeasurePoint<radians,TypeParam> ang1(ang_val);
            SignedDirection<radians,TypeParam> ang2(ang_val);
            UnsignedDirection<radians,TypeParam> ang3(ang_val);
            auto dir = Measure2d<TypeParam, Metre>::normalized(ang1);

            // Create eight projectors, presumed equal, and apply each of them
            // to the original vector.
            auto v2 = v1.mapped_by(linear_map2<TypeParam>::projection(ang1));
            auto v3 = v1.mapped_by(make_projection(ang1));
            auto v4 = v1.mapped_by(linear_map2<TypeParam>::projection(ang2));
            auto v5 = v1.mapped_by(make_projection(ang2));
            auto v6 = v1.mapped_by(linear_map2<TypeParam>::projection(ang3));
            auto v7 = v1.mapped_by(make_projection(ang3));
            auto v8 = v1.mapped_by(linear_map2<TypeParam>::projection(dir));
            auto v9 = v1.mapped_by(make_projection(dir));

            // Check that the resulting vectors have the specified direction.
            EQUAL(0, cross_product(dir, v2))
            EQUAL(0, cross_product(dir, v3))
            EQUAL(0, cross_product(dir, v4))
            EQUAL(0, cross_product(dir, v5))
            EQUAL(0, cross_product(dir, v6))
            EQUAL(0, cross_product(dir, v7))
            EQUAL(0, cross_product(dir, v8))
            EQUAL(0, cross_product(dir, v9))

            // Check that the resulting vectors are orthogonal
            // to the difference between them and the original vector.
            EQUAL(0, (v2 - v1) * v2)
            EQUAL(0, (v3 - v1) * v3)
            EQUAL(0, (v4 - v1) * v4)
            EQUAL(0, (v5 - v1) * v5)
            EQUAL(0, (v6 - v1) * v6)
            EQUAL(0, (v7 - v1) * v7)
            EQUAL(0, (v8 - v1) * v8)
            EQUAL(0, (v9 - v1) * v9)
        }

        { //// Reflection ////
            long double const ang_val = 0.9L;
            MeasurePoint<radians,TypeParam> ang1(ang_val);
            SignedDirection<radians,TypeParam> ang2(ang_val);
            UnsignedDirection<radians,TypeParam> ang3(ang_val);
            auto dir = Measure2d<TypeParam, Metre>::normalized(ang1);

            // Create eight reflectors, presumed equal, and apply each of them
            // to the original vector.
            auto v2 = v1.mapped_by(linear_map2<TypeParam>::reflection(ang1));
            auto v3 = v1.mapped_by(make_reflection(ang1));
            auto v4 = v1.mapped_by(linear_map2<TypeParam>::reflection(ang2));
            auto v5 = v1.mapped_by(make_reflection(ang2));
            auto v6 = v1.mapped_by(linear_map2<TypeParam>::reflection(ang3));
            auto v7 = v1.mapped_by(make_reflection(ang3));
            auto v8 = v1.mapped_by(linear_map2<TypeParam>::reflection(dir));
            auto v9 = v1.mapped_by(make_reflection(dir));

            // Check that the resulting vectors have a cross product
            // with the direction vector that is the opposite
            // of that of the original vector.
            auto expected = -cross_product(v1, dir).value();
            EQUAL(expected, cross_product(v2, dir))
            EQUAL(expected, cross_product(v3, dir))
            EQUAL(expected, cross_product(v4, dir))
            EQUAL(expected, cross_product(v5, dir))
            EQUAL(expected, cross_product(v6, dir))
            EQUAL(expected, cross_product(v7, dir))
            EQUAL(expected, cross_product(v8, dir))
            EQUAL(expected, cross_product(v9, dir))

            // Check that the difference between the resulting vectors
            // and the original vector is orthogonal to the specified direction.
            EQUAL(0, (v2 - v1) * dir)
            EQUAL(0, (v3 - v1) * dir)
            EQUAL(0, (v4 - v1) * dir)
            EQUAL(0, (v5 - v1) * dir)
            EQUAL(0, (v6 - v1) * dir)
            EQUAL(0, (v7 - v1) * dir)
            EQUAL(0, (v8 - v1) * dir)
            EQUAL(0, (v9 - v1) * dir)
        }

        { //// Scaling ////
            // Create some scalers, and apply each of them to the vector.
            auto v2 = v1.mapped_by(linear_map2<TypeParam>::scaling(0, 0));
            EQUAL(0, v2.x()) EQUAL(0, v2.y())
            v2 = v1.mapped_by(linear_map2<TypeParam>::scaling(1, 0));
            EQUAL(x, v2.x()) EQUAL(0, v2.y())
            v2 = v1.mapped_by(linear_map2<TypeParam>::scaling(0, 1));
            EQUAL(0, v2.x()) EQUAL(y, v2.y())
            v2 = v1.mapped_by(linear_map2<TypeParam>::scaling(1, 1));
            EQUAL(x, v2.x()) EQUAL(y, v2.y())
            v2 = v1.mapped_by(linear_map2<TypeParam>::scaling(-3.7L, -1.921L));
            EQUAL(x * -3.7L, v2.x()) EQUAL(y * -1.921L, v2.y())
        }

        { //// Inversion ////
            linear_map2<TypeParam> lm;

            // Set a non-invertible matrix.
            lm.coeff(0, 0) = 2; lm.coeff(0, 1) = 3;
            lm.coeff(1, 0) = 6; lm.coeff(1, 1) = 9;

            // Check that its inverse has all zeros.
            lm = lm.inverted();
            EXPECT_EQ(0, lm.coeff(0, 0));
            EXPECT_EQ(0, lm.coeff(0, 1));
            EXPECT_EQ(0, lm.coeff(1, 0));
            EXPECT_EQ(0, lm.coeff(1, 1));

            // Set an invertible matrix.
            lm.coeff(0, 0) = 2; lm.coeff(0, 1) = 3;
            lm.coeff(1, 0) = 9; lm.coeff(1, 1) = 6;

            // Check that by applying it before or after its inverse,
            // the same vector is obtained.
            auto inv_lm = lm.inverted();
            auto v2 = v1.mapped_by(lm).mapped_by(inv_lm);
            EQUAL(x, v2.x())
            EQUAL(y, v2.y())

            v2 = v1.mapped_by(inv_lm).mapped_by(lm);
            EQUAL(x, v2.x())
            EQUAL(y, v2.y())
        }

        { //// Composition ////
            // Create three linear 2D transformations.
            auto lm1 = make_scaling(TypeParam(3), TypeParam(4));
            auto lm2 = make_projection(MeasurePoint<degrees,TypeParam>(83));
            auto lm3 = make_rotation(Measure<degrees,TypeParam>(147));

            // Apply them one at a time.
            auto v2 = v1.mapped_by(lm1).mapped_by(lm2).mapped_by(lm3);

            // Combine the first with the second, and the result with the third,
            // and apply the result.
            auto v3 = v1.mapped_by(combine(combine(lm1, lm2), lm3));

            // Combine the second with the third, and the first with the result,
            // and apply the result.
            auto v4 = v1.mapped_by(combine(lm1, combine(lm2, lm3)));

            // Check that the three transformed vectors are the same.
            EQUAL(v2.x().value(), v3.x())
            EQUAL(v2.y().value(), v3.y())
            EQUAL(v2.x().value(), v4.x())
            EQUAL(v2.y().value(), v4.y())

            // Set an invertible matrix.
            lm1.coeff(0, 0) = 2; lm1.coeff(0, 1) = 3;
            lm1.coeff(1, 0) = 9; lm1.coeff(1, 1) = 6;

            // Combine it with its inverse,
            // and its inverse with it.
            // In both cases, the result should be the identity.
            auto lm1_inv = lm1.inverted();
            lm2 = combine(lm1, lm1_inv);
            lm3 = combine(lm1_inv, lm1);

            // Apply to a vector the resulting transformations.
            v2 = v1.mapped_by(lm2);
            v3 = v1.mapped_by(lm3);

            // Check that the resulting vectors are (almost) unchanged.
            EQUAL(x, v2.x())
            EQUAL(y, v2.y())
            EQUAL(x, v3.x())
            EQUAL(y, v3.y())
        }
    }

    TYPED_TEST(fractional_test, affine_map2)
    {
        long double fx = -3.2L;
        long double fy = -7.9L;
        MeasurePoint2d<TypeParam, Metre> fp(fx, fy);
        long double x = 17.8L;
        long double y = 13.3L;
        MeasurePoint2d<TypeParam, Metre> p1(x, y);

        { //// Translation ////
            // Create two translators, presumed equal.
            long double dx = -13.2L;
            long double dy = -27.9L;
            Measure2d<TypeParam, Metre> delta(dx, dy);
            auto am1 = affine_map2<TypeParam, Metre>::translation(delta);
            auto am2 = make_translation(delta);

            // Apply them to the same point.
            auto p2 = p1.mapped_by(am1);
            auto p3 = p1.mapped_by(am2);

            // The resulting vectors should be equal.
            EQUAL(x + dx, p2.x())
            EQUAL(y + dy, p2.y())
            EQUAL(x + dx, p3.x())
            EQUAL(y + dy, p3.y())
        }

        { //// Rotation ////
            auto p2 = p1;
            auto p3 = p1;
            Measure<degrees,long double> ang(36.L);

            // Create two rotators, presumed equal.
            auto am1 = affine_map2<TypeParam, Metre>::rotation(fp, ang);
            auto am2 = make_rotation(fp, ang);

            // Apply 10 rotations by 1/10 of a turn
            // by using both rotators.
            for (int i = 0; i < 10; ++i)
            {
                p2 = p2.mapped_by(am1);
                p3 = p3.mapped_by(am2);
            }

            // The resulting vectors should be equal to the original one.
            EQUAL(x, p2.x())
            EQUAL(y, p2.y())
            EQUAL(x, p3.x())
            EQUAL(y, p3.y())
        }

        { //// Rotation at right ////
            auto p2 = p1.mapped_by(affine_map2<TypeParam, Metre>::rotation_at_right(fp));
            auto p3 = p1.mapped_by(make_rotation_at_right(fp));

            // The resulting points should make a vectors with the fixed point
            // that has x equal to the original,
            // and y equal to the opposite of the original.
            auto v1 = p1 - fp;
            auto v2 = p2 - fp;
            auto v3 = p3 - fp;
            EQUAL(y - fy, v2.x())
            EQUAL(-(x - fx), v2.y())
            EQUAL(y - fy, v3.x())
            EQUAL(-(x - fx), v3.y())

            // The resulting vectors should be orthogonal to the original one.
            EQUAL(0, v2 * v1)
            EQUAL(0, v3 * v1)
        }

        { //// Rotation at left ////
            auto p2 = p1.mapped_by(affine_map2<TypeParam, Metre>::rotation_at_left(fp));
            auto p3 = p1.mapped_by(make_rotation_at_left(fp));

            // The resulting points should make a vectors with the fixed point
            // that has x equal to the opposite of the original,
            // and y equal to the original.
            auto v1 = p1 - fp;
            auto v2 = p2 - fp;
            auto v3 = p3 - fp;
            EQUAL(-(y - fy), v2.x())
            EQUAL(x - fx, v2.y())
            EQUAL(-(y - fy), v3.x())
            EQUAL(x - fx, v3.y())

            // The resulting vectors should be orthogonal to the original one.
            EQUAL(0, v2 * v1)
            EQUAL(0, v3 * v1)
        }

        { //// Projection ////
            long double const ang_val = 0.9L;
            MeasurePoint<radians,TypeParam> ang1(ang_val);
            SignedDirection<radians,TypeParam> ang2(ang_val);
            UnsignedDirection<radians,TypeParam> ang3(ang_val);
            auto dir = Measure2d<TypeParam, Metre>::normalized(ang1);

            // Create eight projectors, presumed equal, and apply each of them
            // to the original point.
            auto p2 = p1.mapped_by(affine_map2<TypeParam, Metre>::projection(fp, ang1));
            auto p3 = p1.mapped_by(make_projection(fp, ang1));
            auto p4 = p1.mapped_by(affine_map2<TypeParam, Metre>::projection(fp, ang2));
            auto p5 = p1.mapped_by(make_projection(fp, ang2));
            auto p6 = p1.mapped_by(affine_map2<TypeParam, Metre>::projection(fp, ang3));
            auto p7 = p1.mapped_by(make_projection(fp, ang3));
            auto p8 = p1.mapped_by(affine_map2<TypeParam, Metre>::projection(fp, dir));
            auto p9 = p1.mapped_by(make_projection(fp, dir));

            // Check that the resulting points belong to the projection line.
            EQUAL(0, cross_product(dir, p2 - fp))
            EQUAL(0, cross_product(dir, p3 - fp))
            EQUAL(0, cross_product(dir, p4 - fp))
            EQUAL(0, cross_product(dir, p5 - fp))
            EQUAL(0, cross_product(dir, p6 - fp))
            EQUAL(0, cross_product(dir, p7 - fp))
            EQUAL(0, cross_product(dir, p8 - fp))
            EQUAL(0, cross_product(dir, p9 - fp))

            // Check that the vectors from the resulting points to the fixed point
            // are orthogonal to the vectors from the resulting points
            // to the original point.
            EQUAL(0, (p2 - p1) * (p2 - fp))
            EQUAL(0, (p3 - p1) * (p3 - fp))
            EQUAL(0, (p4 - p1) * (p4 - fp))
            EQUAL(0, (p5 - p1) * (p5 - fp))
            EQUAL(0, (p6 - p1) * (p6 - fp))
            EQUAL(0, (p7 - p1) * (p7 - fp))
            EQUAL(0, (p8 - p1) * (p8 - fp))
            EQUAL(0, (p9 - p1) * (p9 - fp))
        }

        { //// Reflection ////
            long double const ang_val = 0.9L;
            MeasurePoint<radians,TypeParam> ang1(ang_val);
            SignedDirection<radians,TypeParam> ang2(ang_val);
            UnsignedDirection<radians,TypeParam> ang3(ang_val);
            auto dir = Measure2d<TypeParam, Metre>::normalized(ang1);

            // Create eight reflectors, presumed equal, and apply each of them
            // to the original point.
            auto p2 = p1.mapped_by(affine_map2<TypeParam, Metre>::reflection(fp, ang1));
            auto p3 = p1.mapped_by(make_reflection(fp, ang1));
            auto p4 = p1.mapped_by(affine_map2<TypeParam, Metre>::reflection(fp, ang2));
            auto p5 = p1.mapped_by(make_reflection(fp, ang2));
            auto p6 = p1.mapped_by(affine_map2<TypeParam, Metre>::reflection(fp, ang3));
            auto p7 = p1.mapped_by(make_reflection(fp, ang3));
            auto p8 = p1.mapped_by(affine_map2<TypeParam, Metre>::reflection(fp, dir));
            auto p9 = p1.mapped_by(make_reflection(fp, dir));

            // Check that the vectors from the fixed point to the resulting point
            // have a cross product with the director vector,
            // that is the opposite of that of the original point.
            auto expected = -cross_product(p1 - fp, dir).value();
            EQUAL(expected, cross_product(p2 - fp, dir))
            EQUAL(expected, cross_product(p3 - fp, dir))
            EQUAL(expected, cross_product(p4 - fp, dir))
            EQUAL(expected, cross_product(p5 - fp, dir))
            EQUAL(expected, cross_product(p6 - fp, dir))
            EQUAL(expected, cross_product(p7 - fp, dir))
            EQUAL(expected, cross_product(p8 - fp, dir))
            EQUAL(expected, cross_product(p9 - fp, dir))

            // Check that the vectors from the resulting points
            // to the original point are orthogonal to the projection line.
            EQUAL(0, (p2 - p1) * dir)
            EQUAL(0, (p3 - p1) * dir)
            EQUAL(0, (p4 - p1) * dir)
            EQUAL(0, (p5 - p1) * dir)
            EQUAL(0, (p6 - p1) * dir)
            EQUAL(0, (p7 - p1) * dir)
            EQUAL(0, (p8 - p1) * dir)
            EQUAL(0, (p9 - p1) * dir)
        }

        { //// Scaling ////
            // Create some scalers, and apply each of them to the vector.
            auto p2 = p1.mapped_by(affine_map2<TypeParam, Metre>
                ::scaling(fp, 0, 0));
            EQUAL(fx, p2.x()) EQUAL(fy, p2.y())
            p2 = p1.mapped_by(affine_map2<TypeParam, Metre>::scaling(fp, 1, 0));
            EQUAL(x, p2.x()) EQUAL(fy, p2.y())
            p2 = p1.mapped_by(affine_map2<TypeParam, Metre>::scaling(fp, 0, 1));
            EQUAL(fx, p2.x()) EQUAL(y, p2.y())
            p2 = p1.mapped_by(affine_map2<TypeParam, Metre>::scaling(fp, 1, 1));
            EQUAL(x, p2.x()) EQUAL(y, p2.y())
            p2 = p1.mapped_by(affine_map2<TypeParam, Metre>
                ::scaling(fp, -3.7L, -1.921L));
            EQUAL(fx + (x - fx) * -3.7L, p2.x())
            EQUAL(fy + (y - fy) * -1.921L, p2.y())
        }

        { //// Inversion ////
            affine_map2<TypeParam, Metre> am;

            // Set a non-invertible matrix.
            am.coeff(0, 0) = 2; am.coeff(0, 1) = 3; am.coeff(0, 2) = -3.1L;
            am.coeff(1, 0) = 6; am.coeff(1, 1) = 9; am.coeff(1, 2) = -7.4L;

            // Check that its inverse has all zeros.
            am = am.inverted();
            EXPECT_EQ(0, am.coeff(0, 0));
            EXPECT_EQ(0, am.coeff(0, 1));
            EXPECT_EQ(0, am.coeff(0, 2));
            EXPECT_EQ(0, am.coeff(1, 0));
            EXPECT_EQ(0, am.coeff(1, 1));
            EXPECT_EQ(0, am.coeff(1, 2));

            // Set an invertible matrix.
            am.coeff(0, 0) = 2; am.coeff(0, 1) = 3; am.coeff(0, 2) = -3.1L;
            am.coeff(1, 0) = 9; am.coeff(1, 1) = 6; am.coeff(1, 2) = -7.4L;

            // Check that by applying it before or after its inverse,
            // the same vector is obtained.
            auto inv_am = am.inverted();
            auto p2 = p1.mapped_by(am).mapped_by(inv_am);
            EQUAL(x, p2.x())
            EQUAL(y, p2.y())

            p2 = p1.mapped_by(inv_am).mapped_by(am);
            EQUAL(x, p2.x())
            EQUAL(y, p2.y())
        }

        { //// Composition ////
            // Create three affine 2D transformations.
            auto am1 = make_scaling(fp, 3., 4.);
            auto am2 = make_projection(fp, MeasurePoint<degrees>(83));
            auto am3 = make_rotation(fp, Measure<degrees>(147));

            // Apply them one at a time.
            auto p2 = p1.mapped_by(am1).mapped_by(am2).mapped_by(am3);

            // Combine the first with the second, and the result with the third,
            // and apply the result.
            auto p3 = p1.mapped_by(combine(combine(am1, am2), am3));

            // Combine the second with the third, and the first with the result,
            // and apply the result.
            auto p4 = p1.mapped_by(combine(am1, combine(am2, am3)));

            // Check that the three transformed vectors are the same.
            EQUAL(p2.x().value(), p3.x())
            EQUAL(p2.y().value(), p3.y())
            EQUAL(p2.x().value(), p4.x())
            EQUAL(p2.y().value(), p4.y())

            // Set an invertible matrix.
            am1.coeff(0, 0) = 2; am1.coeff(0, 1) = 3;
            am1.coeff(1, 0) = 9; am1.coeff(1, 1) = 6;

            // Combine it with its inverse,
            // and its inverse with it.
            // In both cases, the result should be the identity.
            auto am1_inv = am1.inverted();
            am2 = combine(am1, am1_inv);
            am3 = combine(am1_inv, am1);

            // Apply to a vector the resulting transformations.
            p2 = p1.mapped_by(am2);
            p3 = p1.mapped_by(am3);

            // Check that the resulting vectors are (almost) unchanged.
            EQUAL(x, p2.x())
            EQUAL(y, p2.y())
            EQUAL(x, p3.x())
            EQUAL(y, p3.y())
        }
    }

    TYPED_TEST(general_test, vect2)
    {
        auto epsilon = numeric_limits<TypeParam>::is_integer ?
            1 : numeric_limits<float>::epsilon();
        auto tolerance = Measure<float, Metre>(epsilon * 2);
        auto zero = Measure<float, Metre>(0);

        // Try several numeric values.
        for (int i = 0; i < test_values<TypeParam>::count; ++i)
        {
            TypeParam val1x = test_values<TypeParam>::v[i];

            // Try several numeric values.
            for (int j = 0; j < test_values<TypeParam>::count; ++j)
            {
                TypeParam val1y = test_values<TypeParam>::v[j];
                TypeParam val2 = static_cast<TypeParam>(2.19);
                TypeParam val2x = static_cast<TypeParam>(2.27);
                TypeParam val2y = static_cast<TypeParam>(2.13);
                TypeParam val3 = 3;
                auto m1 = Measure2d<TypeParam, Metre>(val1x, val1y);
                auto m2 = Measure2d<TypeParam, Metre>(val2x, val2y);

                // Construction and "value".
                EXPECT_EQ(val1x, m1.x().value());
                EXPECT_EQ(val1y, m1.y().value());
                EXPECT_EQ(val2x, m2.x().value());
                EXPECT_EQ(val2y, m2.y().value());

                // Operator +=.
                // All test-values except the largest one,
                // as it would cause overflow by incrementing.
                if (i < test_values<TypeParam>::count - 1
                    && j < test_values<TypeParam>::count - 1)
                {
                    m1 = Measure2d<TypeParam, Metre>(val1x, val1y);
                    auto m3 = m1 += m2;
                    EXPECT_EQ(val1x + val2x, m1.x().value());
                    EXPECT_EQ(val1y + val2y, m1.y().value());
                    EXPECT_EQ(val2x, m2.x().value());
                    EXPECT_EQ(val2y, m2.y().value());
                    EXPECT_EQ(val1x + val2x, m3.x().value());
                    EXPECT_EQ(val1y + val2y, m3.y().value());
                }

                // Operator -=.
                // All test-values except the lowest one,
                // as it would cause overflow by decrementing.
                if (0 < i && 0 < j)
                {
                    m1 = Measure2d<TypeParam, Metre>(val1x, val1y);
                    auto m3 = m1 -= m2;
                    EXPECT_EQ(val1x - val2x, m1.x().value());
                    EXPECT_EQ(val1y - val2y, m1.y().value());
                    EXPECT_EQ(val2x, m2.x().value());
                    EXPECT_EQ(val2y, m2.y().value());
                    EXPECT_EQ(val1x - val2x, m3.x().value());
                    EXPECT_EQ(val1y - val2y, m3.y().value());
                }

                // Operator *=.
                // All test-values except the lowest and the largest ones,
                // as they would cause overflow by multiplying.
                if (0 < i && i < test_values<TypeParam>::count - 1
                    && 0 < j && j < test_values<TypeParam>::count - 1)
                {
                    m1 = Measure2d<TypeParam, Metre>(val1x, val1y);
                    auto m3 = m1 *= val2;
                    EQUAL(val1x * val2, m1.x())
                    EQUAL(val1y * val2, m1.y())
                    EQUAL(val1x * val2, m3.x())
                    EQUAL(val1y * val2, m3.y())
                    auto m4 = m1 *= val3;
                    EQUAL(val1x * val2 * val3, m1.x())
                    EQUAL(val1y * val2 * val3, m1.y())
                    EQUAL(val1x * val2 * val3, m4.x())
                    EQUAL(val1y * val2 * val3, m4.y())
                }
                // Operator /=.
                // All test-values.
                if (abs(val2) != 0)
                {
                    m1 = Measure2d<TypeParam, Metre>(val1x, val1y);
                    auto m3 = m1 /= val2;
                    EQUAL(val1x / val2, m1.x())
                    EQUAL(val1y / val2, m1.y())
                    EQUAL(val1x / val2, m3.x())
                    EQUAL(val1y / val2, m3.y())
                    if (abs(val3) != 0)
                    {
                        auto m4 = m1 /= val3;
                        EQUAL(val1x / val2 / val3, m1.x())
                        EQUAL(val1y / val2 / val3, m1.y())
                        EQUAL(val1x / val2 / val3, m4.x())
                        EQUAL(val1y / val2 / val3, m4.y())
                    }
                }

                // Relational operators.

                // Comparing equal measures.
                m1 = Measure2d<TypeParam, Metre>(val1x, val1y);
                m2 = m1;
                EXPECT_TRUE(m1 == m2);
                EXPECT_FALSE(m1 != m2);

                // All test-values except the two lowest and the two largest ones,
                // as they would cause overflow by squaring.
                if (2 <= i && i < test_values<TypeParam>::count - 2
                    && 2 <= j && j < test_values<TypeParam>::count - 2)
                {
                    EXPECT_TRUE(is_equal(m1, m2, zero));
                    EXPECT_TRUE(is_equal(m1, m2, tolerance));

                    // Comparing a measure with its half, if it is large,
                    // with its double if it is small.
                    if (abs(m2.x().value()) >= MIN_THRESHOLD)
                    {
                        if (abs(squared_norm_value(m2)) > 1) m2 /= 2;
                        else m1 *= 2;
                        EXPECT_FALSE(m1 == m2);
                        EXPECT_TRUE(m1 != m2);
                        EXPECT_FALSE(is_equal(m1, m2, zero));
                    }
                }
            }
        }

        // Comparing 1 with a bit more than 1.
        auto m3 = Measure2d<TypeParam, Metre>(1, 1);
        auto m4 = Measure2d<TypeParam, Metre>(1 + epsilon, 1);
        auto m4b = Measure2d<TypeParam, Metre>(1, 1 + epsilon);
        EXPECT_FALSE(m3 == m4);
        EXPECT_FALSE(m3 == m4b);
        EXPECT_TRUE(m3 != m4);
        EXPECT_TRUE(m3 != m4b);
        EXPECT_TRUE(is_equal(m3, m4, tolerance));
        EXPECT_TRUE(is_equal(m3, m4b, tolerance));

        // Comparing -1 with a bit less than -1.
        auto m5 = Measure2d<TypeParam, Metre>(-1, -1);
        auto m6 = Measure2d<TypeParam, Metre>(-1 - epsilon, -1);
        auto m6b = Measure2d<TypeParam, Metre>(-1, -1 - epsilon);
        EXPECT_FALSE(m5 == m6);
        EXPECT_FALSE(m5 == m6b);
        EXPECT_TRUE(m5 != m6);
        EXPECT_TRUE(m5 != m6b);
        EXPECT_TRUE(is_equal(m5, m6, tolerance));
        EXPECT_TRUE(is_equal(m5, m6b, tolerance));

        // Compare non-extreme numbers, far from zero,
        // with a small tolerance or with a large tolerance.
        auto m7 = Measure2d<TypeParam, Metre>(4, 4);
        auto m8 = Measure2d<TypeParam, Metre>(8, 8);
        auto tol1 = Measure<f32, Metre>(5);
        auto tol2 = Measure<f32, Metre>(6);
        EXPECT_FALSE(is_equal(m7, m8, tol1));
        EXPECT_FALSE(is_equal(m8, m7, tol1));
        EXPECT_TRUE(is_equal(m7, m8, tol2));
        EXPECT_TRUE(is_equal(m8, m7, tol2));
    }

    TYPED_TEST(general_test, point2)
    {
        auto epsilon = numeric_limits<TypeParam>::is_integer ?
            1 : numeric_limits<float>::epsilon();
        auto tolerance = Measure<f32, Metre>(epsilon * 2);
        auto zero = Measure<f32, Metre>(0);

        // Try several numeric values.
        for (int i = 0; i < test_values<TypeParam>::count; ++i)
        {
            TypeParam val1x = test_values<TypeParam>::v[i];

            // Try several numeric values.
            for (int j = 0; j < test_values<TypeParam>::count; ++j)
            {
                TypeParam val1y = test_values<TypeParam>::v[j];
                TypeParam val2x = static_cast<TypeParam>(2.27);
                TypeParam val2y = static_cast<TypeParam>(2.13);
                auto m1 = MeasurePoint2d<TypeParam, Metre>(val1x, val1y);
                auto m2 = Measure2d<TypeParam, Metre>(val2x, val2y);

                // Construction and "value".
                EXPECT_EQ(val1x, m1.x().value());
                EXPECT_EQ(val1y, m1.y().value());
                EXPECT_EQ(val2x, m2.x().value());
                EXPECT_EQ(val2y, m2.y().value());

                // Operator +=.
                // All test-values except the largest one,
                // as it would cause overflow by incrementing.
                if (i < test_values<TypeParam>::count - 1
                    && j < test_values<TypeParam>::count - 1)
                {
                    m1 = MeasurePoint2d<TypeParam, Metre>(val1x, val1y);
                    auto m3 = m1 += m2;
                    EXPECT_EQ(val1x + val2x, m1.x().value());
                    EXPECT_EQ(val1y + val2y, m1.y().value());
                    EXPECT_EQ(val2x, m2.x().value());
                    EXPECT_EQ(val2y, m2.y().value());
                    EXPECT_EQ(val1x + val2x, m3.x().value());
                    EXPECT_EQ(val1y + val2y, m3.y().value());
                }

                // Operator -=.
                // All test-values except the lowest one,
                // as it would cause overflow by decrementing.
                if (0 < i && 0 < j)
                {
                    m1 = MeasurePoint2d<TypeParam, Metre>(val1x, val1y);
                    auto m3 = m1 -= m2;
                    EXPECT_EQ(val1x - val2x, m1.x().value());
                    EXPECT_EQ(val1y - val2y, m1.y().value());
                    EXPECT_EQ(val2x, m2.x().value());
                    EXPECT_EQ(val2y, m2.y().value());
                    EXPECT_EQ(val1x - val2x, m3.x().value());
                    EXPECT_EQ(val1y - val2y, m3.y().value());
                }

                // Relational operators.

                // Comparing equal measures.
                m1 = MeasurePoint2d<TypeParam, Metre>(val1x, val1y);
                auto m3 = m1;
                EXPECT_TRUE(m1 == m3);
                EXPECT_FALSE(m1 != m3);

                // All test-values except the two lowest and the two largest ones,
                // as they would cause overflow by squaring.
                if (2 <= i && i < test_values<TypeParam>::count - 2
                    && 2 <= j && j < test_values<TypeParam>::count - 2)
                {
                    EXPECT_TRUE(is_equal(m1, m3, zero));
                    EXPECT_TRUE(is_equal(m1, m3, tolerance));

                    // Comparing a measure with its half, if it is large,
                    // with its double if it is small.
                    if (abs(m3.x().value()) >= MIN_THRESHOLD)
                    {
                        auto m3b = m3;
                        if (abs(m3.x().value()) + abs(m3.y().value()) > 2)
                        {
                            m3 = MeasurePoint2d<TypeParam, Metre>(m3.x().value() / static_cast<TypeParam>(2), m3.y().value());
                            m3b = MeasurePoint2d<TypeParam, Metre>(m3.x().value(), m3.y().value() / static_cast<TypeParam>(2));
                        }
                        else
                        {
                            m3 = MeasurePoint2d<TypeParam, Metre>(m3.x().value() * static_cast<TypeParam>(2), m3.y().value());
                            m3b = MeasurePoint2d<TypeParam, Metre>(m3.x().value(), m3.y().value() * static_cast<TypeParam>(2));
                        }
                        EXPECT_FALSE(m1 == m3);
                        EXPECT_FALSE(m1 == m3b);
                        EXPECT_TRUE(m1 != m3);
                        EXPECT_TRUE(m1 != m3b);
                        EXPECT_FALSE(is_equal(m1, m3, zero));
                        EXPECT_FALSE(is_equal(m1, m3b, zero));
                    }
                }
            }
        }

        // Comparing 1 with a bit more than 1.
        auto m3 = MeasurePoint2d<TypeParam, Metre>(1, 1);
        auto m4 = MeasurePoint2d<TypeParam, Metre>(1 + epsilon, 1);
        auto m4b = MeasurePoint2d<TypeParam, Metre>(1, 1 + epsilon);
        EXPECT_FALSE(m3 == m4);
        EXPECT_FALSE(m3 == m4b);
        EXPECT_TRUE(m3 != m4);
        EXPECT_TRUE(m3 != m4b);
        EXPECT_TRUE(is_equal(m3, m4, tolerance));
        EXPECT_TRUE(is_equal(m3, m4b, tolerance));

        // Comparing -1 with a bit less than -1.
        auto m5 = MeasurePoint2d<TypeParam, Metre>(-1, -1);
        auto m6 = MeasurePoint2d<TypeParam, Metre>(-1 - epsilon, -1);
        auto m6b = MeasurePoint2d<TypeParam, Metre>(-1, -1 - epsilon);
        EXPECT_FALSE(m5 == m6);
        EXPECT_FALSE(m5 == m6b);
        EXPECT_TRUE(m5 != m6);
        EXPECT_TRUE(m5 != m6b);
        EXPECT_TRUE(is_equal(m5, m6, tolerance));
        EXPECT_TRUE(is_equal(m5, m6b, tolerance));

        // Compare non-extreme numbers, far from zero,
        // with a small tolerance or with a large tolerance.
        auto m7 = MeasurePoint2d<TypeParam, Metre>(4, 4);
        auto m8 = MeasurePoint2d<TypeParam, Metre>(8, 8);
        auto tol1 = Measure<f32, Metre>(5);
        auto tol2 = Measure<float, Metre>(6);
        EXPECT_FALSE(is_equal(m7, m8, tol1));
        EXPECT_FALSE(is_equal(m8, m7, tol1));
        EXPECT_TRUE(is_equal(m7, m8, tol2));
        EXPECT_TRUE(is_equal(m8, m7, tol2));
    }

    TYPED_TEST(general_test, vectpoint2)
    {
        // Midpoint.
        // Try all pairs of pairs of numeric values except extremes.
        auto fraction = abs(static_cast<TypeParam>(0.23f));
        for (int i1 = 1; i1 < test_values<TypeParam>::count - 1; ++i1)
        {
            TypeParam val1x = test_values<TypeParam>::v[i1];
            for (int j1 = 1; j1 < test_values<TypeParam>::count - 1; ++j1)
            {
                TypeParam val1y = test_values<TypeParam>::v[j1];
                auto m1 = MeasurePoint2d<TypeParam, Metre>(val1x, val1y);
                for (int i2 = 1; i2 < test_values<TypeParam>::count - 1; ++i2)
                {
                    TypeParam val2x = test_values<TypeParam>::v[i2];
                    for (int j2 = 1; j2 < test_values<TypeParam>::count - 1; ++j2)
                    {
                        TypeParam val2y = test_values<TypeParam>::v[j2];
                        auto m2 = MeasurePoint2d<TypeParam, Metre>(val2x, val2y);
                        EQUAL((val1x + val2x) / static_cast<TypeParam>(2), midpoint(m1, m2).x())
                        EQUAL((val1y + val2y) / static_cast<TypeParam>(2), midpoint(m1, m2).y())
                        EQUAL(val1x, midpoint(m1, m2, 0).x())
                        EQUAL(val1y, midpoint(m1, m2, 0).y())
                        EQUAL(val2x, midpoint(m1, m2, 1).x())
                        EQUAL(val2y, midpoint(m1, m2, 1).y())
                        EQUAL(val1x * (1 - fraction) + val2x * fraction, midpoint(m1, m2, fraction).x())
                        EQUAL(val1y * (1 - fraction) + val2y * fraction, midpoint(m1, m2, fraction).y())
                    }
                }
            }
        }

        // Barycentric combination.
        // Try all pairs of pairs of numeric values except extremes,
        // and add some other points.
        for (int i1 = 1; i1 < test_values<TypeParam>::count - 1; ++i1)
        {
            TypeParam val1x = test_values<TypeParam>::v[i1];
            for (int j1 = 1; j1 < test_values<TypeParam>::count - 1; ++j1)
            {
                TypeParam val1y = test_values<TypeParam>::v[j1];
                auto m1 = MeasurePoint2d<TypeParam, Metre>(val1x, val1y);
                for (int i2 = 1; i2 < test_values<TypeParam>::count - 1; ++i2)
                {
                    TypeParam val2x = test_values<TypeParam>::v[i2];
                    for (int j2 = 1; j2 < test_values<TypeParam>::count - 1; ++j2)
                    {
                        TypeParam val2y = test_values<TypeParam>::v[j2];
                        auto m2 = MeasurePoint2d<TypeParam, Metre>(val2x, val2y);

                        TypeParam val3x = static_cast<TypeParam>(3.1);
                        TypeParam val3y = static_cast<TypeParam>(3.2);
                        auto m3 = MeasurePoint2d<TypeParam, Metre>(val3x, val3y);
                        TypeParam val4x = static_cast<TypeParam>(4.3);
                        TypeParam val4y = static_cast<TypeParam>(4.4);
                        auto m4 = MeasurePoint2d<TypeParam, Metre>(val4x, val4y);
                        MeasurePoint2d<TypeParam, Metre> point2array[] = { m1, m2, m3, m4 };
                        TypeParam weights[] = { 2, 3, 7, 4 };
                        EQUAL(val1x * weights[0],
                            barycentric_combination(1, point2array, weights).x())
                        EQUAL(val1y * weights[0],
                            barycentric_combination(1, point2array, weights).y())
                        EQUAL(val1x * weights[0] + val2x * weights[1],
                            barycentric_combination(2, point2array, weights).x())
                        EQUAL(val1y * weights[0] + val2y * weights[1],
                            barycentric_combination(2, point2array, weights).y())
                        EQUAL(val1x * weights[0] + val2x * weights[1]
                            + val3x * weights[2],
                            barycentric_combination(3, point2array, weights).x())
                        EQUAL(val1y * weights[0] + val2y * weights[1]
                            + val3y * weights[2],
                            barycentric_combination(3, point2array, weights).y())
                        EQUAL(val1x * weights[0] + val2x * weights[1]
                            + val3x * weights[2] + val4x * weights[3],
                            barycentric_combination(4, point2array, weights).x())
                        EQUAL(val1y * weights[0] + val2y * weights[1]
                            + val3y * weights[2] + val4y * weights[3],
                            barycentric_combination(4, point2array, weights).y())
                    }
                }
            }
        }

        // Try all pairs of pairs of numeric values except extremes.
        auto sqrt_of_max = abs(static_cast<TypeParam>(std::sqrt(test_values<TypeParam>
            ::v[test_values<TypeParam>::count - 1])));
        auto sqrt_of_min = abs(static_cast<TypeParam>(std::sqrt(test_values<TypeParam>
            ::v[(test_values<TypeParam>::count + 1) / 2])));
        for (int i1 = 1; i1 < test_values<TypeParam>::count - 1; ++i1)
        {
            TypeParam val1x = test_values<TypeParam>::v[i1];
            for (int j1 = 1; j1 < test_values<TypeParam>::count - 1; ++j1)
            {
                TypeParam val1y = test_values<TypeParam>::v[j1];
                auto v1 = Measure2d<TypeParam, Metre>(val1x, val1y);
                auto p1 = MeasurePoint2d<TypeParam, Metre>(val1x, val1y);

                for (int i2 = 1; i2 < test_values<TypeParam>::count - 1; ++i2)
                {
                    TypeParam val2x = test_values<TypeParam>::v[i2];
                    for (int j2 = 1; j2 < test_values<TypeParam>::count - 1; ++j2)
                    {
                        TypeParam val2y = test_values<TypeParam>::v[j2];
                        auto v2 = Measure2d<TypeParam, Metre>(val2x, val2y);
                        auto p2 = MeasurePoint2d<TypeParam, Metre>(val2x, val2y);

                        // point2 - point2 -> vect2
                        EXPECT_EQ(val1x - val2x, (p1 - p2).x().value());
                        EXPECT_EQ(val1y - val2y, (p1 - p2).y().value());

                        // point2 + vect2 -> point2
                        EXPECT_EQ(val1x + val2x, (p1 + v2).x().value());
                        EXPECT_EQ(val1y + val2y, (p1 + v2).y().value());

                        // point2 - vect2 -> point2
                        EXPECT_EQ(val1x - val2x, (p1 - v2).x().value());
                        EXPECT_EQ(val1y - val2y, (p1 - v2).y().value());

                        // vect2 + vect2 -> vect2
                        EXPECT_EQ(val1x + val2x, (v1 + v2).x().value());
                        EXPECT_EQ(val1y + val2y, (v1 + v2).y().value());

                        // vect2 - vect2 -> vect2
                        EXPECT_EQ(val1x - val2x, (v1 - v2).x().value());
                        EXPECT_EQ(val1y - val2y, (v1 - v2).y().value());

                        if (abs(val1x) < sqrt_of_max && abs(val2x) < sqrt_of_max)
                        {
                            // N * vect2 -> vect2
                            EXPECT_EQ(val1x * val2x, (val1x * v2).x().value());

                            // vect2 * N -> vect2
                            EXPECT_EQ(val1x * val2x, (v1 * val2x).x().value());
                        }

                        if (abs(val1y) < sqrt_of_max && abs(val2y) < sqrt_of_max)
                        {
                            // N * vect2 -> vect2
                            EXPECT_EQ(val1y * val2y, (val1y * v2).y().value());

                            // vect2 * N -> vect2
                            EXPECT_EQ(val1y * val2y, (v1 * val2y).y().value());
                        }

                        // vect2 / N -> vect2
                        if (abs(val2x) != 0) EXPECT_EQ(val1x / val2x, (v1 / val2x).x().value());
                        if (abs(val2y) != 0) EXPECT_EQ(val1y / val2y, (v1 / val2y).y().value());
                    }
                }
            }
        }

        for (int i = 2; i < test_values<TypeParam>::count - 2; ++i)
        {
            for (int j = 2; j < test_values<TypeParam>::count - 2; ++j)
            {
                TypeParam val1x = test_values<TypeParam>::v[i];
                TypeParam val1y = test_values<TypeParam>::v[j];
                auto m1 = Measure2d<TypeParam, Metre>(val1x, val1y);
                if (abs(val1x) < sqrt_of_max && abs(val1y) < sqrt_of_max
                    && abs(val1x) > sqrt_of_min && abs(val1y) > sqrt_of_min)
                {
                    EQUAL(val1x * val1x + val1y * val1y, m1 * m1)
                    EQUAL(static_cast<TypeParam>(
                        std::sqrt(val1x * val1x + val1y * val1y)), norm(m1))
                }
            }
        }
    }

    TYPED_TEST(fractional_test, linear_map3)
    {
        long double dx = 1.9L;
        long double dy = -5.4L;
        long double dz = -0.7L;
        Measure3d<TypeParam, Metre> dir(dx, dy, dz);
        dir = normalized(dir);
        long double x = 17.8L;
        long double y = 13.3L;
        long double z = -2.7L;
        Measure3d<TypeParam, Metre> v1(x, y, z);

        { //// Rotation ////
            auto v2 = v1;
            auto v3 = v1;
            Measure<degrees,long double> ang(36.L);

            // Create two rotators, presumed equal.
            auto lm1 = linear_map3<TypeParam>::rotation(dir, ang);
            auto lm2 = make_rotation(dir, ang);

            // Apply 10 rotations by 1/10 of a turn
            // by using both rotators.
            // Check that every intermediate vector has the same
            // dot product with the direction and the same norm
            // of the cross product with the direction.
            auto dp1v = (v1 * dir).value();
            auto cpn1v = norm(cross_product(v1, dir)).value();
            for (int i = 0; i < 10; ++i)
            {
                v2 = v2.mapped_by(lm1);
                v3 = v3.mapped_by(lm2);

                auto dp2 = v2 * dir;
                auto dp3 = v3 * dir;
                auto cpn2 = norm(cross_product(v2, dir));
                auto cpn3 = norm(cross_product(v3, dir));
                EQUAL(dp1v, dp2)
                EQUAL(dp1v, dp3)
                EQUAL(cpn1v, cpn2)
                EQUAL(cpn1v, cpn3)
            }

            // The resulting vectors should be equal to the original one.
            EQUAL(x, v2.x())
            EQUAL(y, v2.y())
            EQUAL(z, v2.z())
            EQUAL(x, v3.x())
            EQUAL(y, v3.y())
            EQUAL(z, v3.z())
        }

        { //// Rotation at right ////
            auto lm2 = linear_map3<TypeParam>::rotation_at_right(dir);
            auto lm3 = make_rotation_at_right(dir);
            auto v2 = v1;
            auto v3 = v1;

            // Apply 4 right-angle rotations by using both rotators.
            // Check that every intermediate vector has the same
            // dot product with the direction and the same norm
            // of the cross product with the direction.
            auto dp1v = (v1 * dir).value();
            auto cpn1v = norm(cross_product(v1, dir)).value();
            for (int i = 0; i < 4; ++i)
            {
                v2 = v2.mapped_by(lm2);
                v3 = v3.mapped_by(lm3);

                auto dp2 = v2 * dir;
                auto dp3 = v3 * dir;
                auto cpn2 = norm(cross_product(v2, dir));
                auto cpn3 = norm(cross_product(v3, dir));
                EQUAL(dp1v, dp2)
                EQUAL(dp1v, dp3)
                EQUAL(cpn1v, cpn2)
                EQUAL(cpn1v, cpn3)
            }

            // The resulting vectors should be equal to the original one.
            EQUAL(x, v2.x())
            EQUAL(y, v2.y())
            EQUAL(z, v2.z())
            EQUAL(x, v3.x())
            EQUAL(y, v3.y())
            EQUAL(z, v3.z())
        }

        { //// Rotation at left ////
            auto lm2 = linear_map3<TypeParam>::rotation_at_left(dir);
            auto lm3 = make_rotation_at_left(dir);
            auto v2 = v1;
            auto v3 = v1;

            // Apply 4 left-angle rotations by using both rotators.
            // Check that every intermediate vector has the same
            // dot product with the direction and the same norm
            // of the cross product with the direction.
            auto dp1v = (v1 * dir).value();
            auto cpn1v = norm(cross_product(v1, dir)).value();
            for (int i = 0; i < 4; ++i)
            {
                v2 = v2.mapped_by(lm2);
                v3 = v3.mapped_by(lm3);

                auto dp2 = v2 * dir;
                auto dp3 = v3 * dir;
                auto cpn2 = norm(cross_product(v2, dir));
                auto cpn3 = norm(cross_product(v3, dir));
                EQUAL(dp1v, dp2)
                EQUAL(dp1v, dp3)
                EQUAL(cpn1v, cpn2)
                EQUAL(cpn1v, cpn3)
            }

            // The resulting vectors should be equal to the original one.
            EQUAL(x, v2.x())
            EQUAL(y, v2.y())
            EQUAL(z, v2.z())
            EQUAL(x, v3.x())
            EQUAL(y, v3.y())
            EQUAL(z, v3.z())
        }

        { //// Projection ////
            // Create two line projectors, presumed equal, and apply each of them
            // to the original vector.
            auto v2 = v1.mapped_by(linear_map3<TypeParam>
                ::projection_onto_line(dir));
            auto v3 = v1.mapped_by(make_projection_onto_line(dir));

            // Check that the resulting vectors have the specified direction.
            EQUAL(0, norm(cross_product(dir, v2)))
            EQUAL(0, norm(cross_product(dir, v3)))

            // Check that the resulting vectors are orthogonal
            // to the difference between them and the original vector.
            EQUAL(0, (v2 - v1) * v2)
            EQUAL(0, (v3 - v1) * v3)

            // Create four plane projectors, presumed equal, and apply each of them
            // to the original vector.
            auto v4 = v1.mapped_by(linear_map3<TypeParam>
                ::projection_onto_plane(dir));
            auto v5 = v1.mapped_by(linear_map3<TypeParam>
                ::projection_onto_plane(
                dir.x().value(), dir.y().value(), dir.z().value()));
            auto v6 = v1.mapped_by(make_projection_onto_plane(dir));
            auto v7 = v1.mapped_by(make_projection_onto_plane(
                dir.x().value(), dir.y().value(), dir.z().value()));

            // Check that the resulting vectors belong to the specified plane.
            EQUAL(0, dir * v4)
            EQUAL(0, dir * v5)
            EQUAL(0, dir * v6)
            EQUAL(0, dir * v7)

            // Check that the difference between the resulting vectors and
            // the original one is orthogonal to the plane.
            EQUAL(0, norm(cross_product(v4 - v1, dir)))
            EQUAL(0, norm(cross_product(v5 - v1, dir)))
            EQUAL(0, norm(cross_product(v6 - v1, dir)))
            EQUAL(0, norm(cross_product(v7 - v1, dir)))
        }

        { //// Reflection ////
            // Create two line reflectors, presumed equal, and apply each of them
            // to the original vector.
            auto v2 = v1.mapped_by(linear_map3<TypeParam>
                ::reflection_over_line(dir));
            auto v3 = v1.mapped_by(make_reflection_over_line(dir));

            // Check that the resulting vectors have a cross product
            // with the direction vector that is the opposite
            // of that of the original vector.
            auto result1 = cross_product(v1, dir);
            EQUAL(0, norm(cross_product(v2, dir) + result1))
            EQUAL(0, norm(cross_product(v3, dir) + result1))

            // Check that the difference between the resulting vectors
            // and the original vector is orthogonal to the specified direction.
            EQUAL(0, (v2 - v1) * dir)
            EQUAL(0, (v3 - v1) * dir)

            // Create four plane reflectors, presumed equal, and apply each of them
            // to the original vector.
            auto v4 = v1.mapped_by(linear_map3<TypeParam>
                ::reflection_over_plane(dir));
            auto v5 = v1.mapped_by(linear_map3<TypeParam>
                ::reflection_over_plane(
                dir.x().value(), dir.y().value(), dir.z().value()));
            auto v6 = v1.mapped_by(make_reflection_over_plane(dir));
            auto v7 = v1.mapped_by(make_reflection_over_plane(
                dir.x().value(), dir.y().value(), dir.z().value()));

            // Check that the dot product of the resulting vectors
            // with the normal is the opposite of that of the original vector.
            EQUAL(0, dir * v4 + dir * v1)
            EQUAL(0, dir * v5 + dir * v1)
            EQUAL(0, dir * v6 + dir * v1)
            EQUAL(0, dir * v7 + dir * v1)

            // Check that the difference between the resulting vectors and
            // the original one is orthogonal to the plane.
            EQUAL(0, norm(cross_product(v4 - v1, dir)))
            EQUAL(0, norm(cross_product(v5 - v1, dir)))
            EQUAL(0, norm(cross_product(v6 - v1, dir)))
            EQUAL(0, norm(cross_product(v7 - v1, dir)))
        }

        { //// Scaling ////
            // Create some scalers, and apply each of them to the vector.
            auto v2 = v1.mapped_by(linear_map3<TypeParam>::scaling(0, 0, 0));
            EQUAL(0, v2.x()) EQUAL(0, v2.y()) EQUAL(0, v2.z())
            v2 = v1.mapped_by(linear_map3<TypeParam>::scaling(1, 0, 0));
            EQUAL(x, v2.x()) EQUAL(0, v2.y()) EQUAL(0, v2.z())
            v2 = v1.mapped_by(linear_map3<TypeParam>::scaling(0, 1, 0));
            EQUAL(0, v2.x()) EQUAL(y, v2.y()) EQUAL(0, v2.z())
            v2 = v1.mapped_by(linear_map3<TypeParam>::scaling(0, 0, 1));
            EQUAL(0, v2.x()) EQUAL(0, v2.y()) EQUAL(z, v2.z())
            v2 = v1.mapped_by(linear_map3<TypeParam>::scaling(1, 1, 0));
            EQUAL(x, v2.x()) EQUAL(y, v2.y()) EQUAL(0, v2.z())
            v2 = v1.mapped_by(linear_map3<TypeParam>::scaling(1, 0, 1));
            EQUAL(x, v2.x()) EQUAL(0, v2.y()) EQUAL(z, v2.z())
            v2 = v1.mapped_by(linear_map3<TypeParam>::scaling(0, 1, 1));
            EQUAL(0, v2.x()) EQUAL(y, v2.y()) EQUAL(z, v2.z())
            v2 = v1.mapped_by(linear_map3<TypeParam>::scaling(1, 1, 1));
            EQUAL(x, v2.x()) EQUAL(y, v2.y()) EQUAL(z, v2.z())
            v2 = v1.mapped_by(linear_map3<TypeParam>
                ::scaling(-3.7L, -1.921L, 2.84L));
            EQUAL(x * -3.7L, v2.x())
            EQUAL(y * -1.921L, v2.y())
            EQUAL(z * 2.84L, v2.z())
        }

        { //// Inversion ////
            linear_map3<TypeParam> lm;

            // Set a non-invertible matrix.
            lm.coeff(0, 0) = 2; lm.coeff(0, 1) = 3; lm.coeff(0, 2) = 5;
            lm.coeff(1, 0) = 6; lm.coeff(1, 1) = 9; lm.coeff(1, 2) = 15;
            lm.coeff(2, 0) = -6; lm.coeff(2, 1) = 4; lm.coeff(2, 2) = 4;

            // Check that its inverse has all zeros.
            lm = lm.inverted();
            EXPECT_EQ(0, lm.coeff(0, 0));
            EXPECT_EQ(0, lm.coeff(0, 1));
            EXPECT_EQ(0, lm.coeff(0, 2));
            EXPECT_EQ(0, lm.coeff(1, 0));
            EXPECT_EQ(0, lm.coeff(1, 1));
            EXPECT_EQ(0, lm.coeff(1, 2));
            EXPECT_EQ(0, lm.coeff(2, 0));
            EXPECT_EQ(0, lm.coeff(2, 1));
            EXPECT_EQ(0, lm.coeff(2, 2));

            // Set an invertible matrix.
            lm.coeff(0, 0) = 2; lm.coeff(0, 1) = 3; lm.coeff(0, 2) = 5;
            lm.coeff(1, 0) = 0; lm.coeff(1, 1) = 9; lm.coeff(1, 2) = 15;
            lm.coeff(2, 0) = -6; lm.coeff(2, 1) = 4; lm.coeff(2, 2) = 4;

            // Check that by applying it before or after its inverse,
            // the same vector is obtained.
            auto inv_lm = lm.inverted();
            auto v2 = v1.mapped_by(lm).mapped_by(inv_lm);
            EQUAL(x, v2.x())
            EQUAL(y, v2.y())
            EQUAL(z, v2.z())

            v2 = v1.mapped_by(inv_lm).mapped_by(lm);
            EQUAL(x, v2.x())
            EQUAL(y, v2.y())
            EQUAL(z, v2.z())
        }

        { //// Composition ////
            // Create three linear 3D transformations.
            auto lm1 = make_scaling(TypeParam(3), TypeParam(4), TypeParam(5));
            auto lm2 = make_projection_onto_line(dir);
            auto lm3 = make_rotation(normalized(Measure3d<TypeParam, Metre>(3, 1, -5)),
                Measure<degrees,TypeParam>(147));

            // Apply them one at a time.
            auto v2 = v1.mapped_by(lm1).mapped_by(lm2).mapped_by(lm3);

            // Combine the first with the second, and the result with the third,
            // and apply the result.
            auto v3 = v1.mapped_by(combine(combine(lm1, lm2), lm3));

            // Combine the second with the third, and the first with the result,
            // and apply the result.
            auto v4 = v1.mapped_by(combine(lm1, combine(lm2, lm3)));

            // Check that the three transformed vectors are the same.
            EQUAL(v2.x().value(), v3.x())
            EQUAL(v2.y().value(), v3.y())
            EQUAL(v2.z().value(), v3.z())
            EQUAL(v2.x().value(), v4.x())
            EQUAL(v2.y().value(), v4.y())
            EQUAL(v2.z().value(), v4.z())

            // Set an invertible matrix.
            lm1.coeff(0, 0) = 2; lm1.coeff(0, 1) = 3; lm1.coeff(0, 2) = 5;
            lm1.coeff(1, 0) = 0; lm1.coeff(1, 1) = 9; lm1.coeff(1, 2) = 15;
            lm1.coeff(2, 0) = -6; lm1.coeff(2, 1) = 4; lm1.coeff(2, 2) = 4;

            // Combine it with its inverse,
            // and its inverse with it.
            // In both cases, the result should be the identity.
            auto lm1_inv = lm1.inverted();
            lm2 = combine(lm1, lm1_inv);
            lm3 = combine(lm1_inv, lm1);

            // Apply to a vector the resulting transformations.
            v2 = v1.mapped_by(lm2);
            v3 = v1.mapped_by(lm3);

            // Check that the resulting vectors are (almost) unchanged.
            EQUAL(x, v2.x())
            EQUAL(y, v2.y())
            EQUAL(z, v2.z())
            EQUAL(x, v3.x())
            EQUAL(y, v3.y())
            EQUAL(z, v3.z())
        }
    }

    TYPED_TEST(fractional_test, affine_map3)
    {
        long double fx = -3.2L;
        long double fy = -7.9L;
        long double fz = 1.5L;
        MeasurePoint3d<TypeParam, Metre> fp(fx, fy, fz);
        long double dx = 1.9L;
        long double dy = -5.4L;
        long double dz = -0.7L;
        Measure3d<TypeParam, Metre> dir(dx, dy, dz);
        dir = normalized(dir);
        long double x = 17.8L;
        long double y = 13.3L;
        long double z = -2.7L;
        MeasurePoint3d<TypeParam, Metre> p1(x, y, z);

        { //// Translation ////
            // Create two translators, presumed equal.
            Measure3d<TypeParam, Metre> delta(dx, dy, dz);
            auto am1 = affine_map3<TypeParam, Metre>::translation(delta);
            auto am2 = make_translation(delta);

            // Apply them to the same point.
            auto p2 = p1.mapped_by(am1);
            auto p3 = p1.mapped_by(am2);

            // The resulting vectors should be equal.
            EQUAL(x + dx, p2.x())
            EQUAL(y + dy, p2.y())
            EQUAL(z + dz, p2.z())
            EQUAL(x + dx, p3.x())
            EQUAL(y + dy, p3.y())
            EQUAL(z + dz, p3.z())
        }

        { //// Rotation ////
            auto p2 = p1;
            auto p3 = p1;
            Measure<degrees,long double> ang(36.L);

            // Create two rotators, presumed equal.
            auto am1 = affine_map3<TypeParam, Metre>::rotation(fp, dir, ang);
            auto am2 = make_rotation(fp, dir, ang);

            // Apply 10 rotations by 1/10 of a turn
            // by using both rotators.
            // Check that the vector from the fixed point to every intermediate
            // point has the same dot product with the direction and the same norm
            // of the cross product with the direction.
    //cout << setprecision(34) << "    p1=" << p1 << ", fp=" << fp << ", dir = " << dir << ", p2=" << p2 << endl;
            auto dp1v = ((p1 - fp) * dir).value();
            auto cpn1v = norm(cross_product((p1 - fp), dir)).value();
            for (int i = 0; i < 10; ++i)
            {
                p2 = p2.mapped_by(am1);
                p3 = p3.mapped_by(am2);

                auto dp2 = (p2 - fp) * dir;
                auto dp3 = (p3 - fp) * dir;
                auto cpn2 = norm(cross_product(p2 - fp, dir));
                auto cpn3 = norm(cross_product(p3 - fp, dir));
    //cout << setprecision(34) << "      dp1v=" << dp1v << ", dp2=" << dp2 << ", dp1v - dp2 = " << dp1v - dp2.value() << ", p2=" << p2 << endl;
                EQUAL(dp1v, dp2)
                EQUAL(dp1v, dp3)
                EQUAL(cpn1v, cpn2)
                EQUAL(cpn1v, cpn3)
            }

            // The resulting points should be equal to the original one.
            EQUAL(x, p2.x())
            EQUAL(y, p2.y())
            EQUAL(z, p2.z())
            EQUAL(x, p3.x())
            EQUAL(y, p3.y())
            EQUAL(z, p3.z())
        }

        { //// Rotation at right ////
            auto am2 = affine_map3<TypeParam, Metre>::rotation_at_right(fp, dir);
            auto am3 = make_rotation_at_right(fp, dir);
            auto p2 = p1;
            auto p3 = p1;

            // Apply 4 right-angle rotations by using both rotators.
            // Check that every intermediate point has the same
            // dot product with the direction and the same norm
            // of the cross product with the direction.
            auto dp1v = ((p1 - fp) * dir).value();
            auto cpn1v = norm(cross_product(p1 - fp, dir)).value();
            for (int i = 0; i < 4; ++i)
            {
                p2 = p2.mapped_by(am2);
                p3 = p3.mapped_by(am3);

                auto dp2 = (p2 - fp) * dir;
                auto dp3 = (p3 - fp) * dir;
                auto cpn2 = norm(cross_product(p2 - fp, dir));
                auto cpn3 = norm(cross_product(p3 - fp, dir));
                EQUAL(dp1v, dp2)
                EQUAL(dp1v, dp3)
                EQUAL(cpn1v, cpn2)
                EQUAL(cpn1v, cpn3)
            }

            // The resulting points should be equal to the original one.
            EQUAL(x, p2.x())
            EQUAL(y, p2.y())
            EQUAL(z, p2.z())
            EQUAL(x, p3.x())
            EQUAL(y, p3.y())
            EQUAL(z, p3.z())
        }

        { //// Rotation at left ////
            auto am2 = affine_map3<TypeParam, Metre>::rotation_at_left(fp, dir);
            auto am3 = make_rotation_at_left(fp, dir);
            auto p2 = p1;
            auto p3 = p1;

            // Apply 4 right-angle rotations by using both rotators.
            // Check that every intermediate point has the same
            // dot product with the direction and the same norm
            // of the cross product with the direction.
            auto dp1v = ((p1 - fp) * dir).value();
            auto cpn1v = norm(cross_product(p1 - fp, dir)).value();
            for (int i = 0; i < 4; ++i)
            {
                p2 = p2.mapped_by(am2);
                p3 = p3.mapped_by(am3);

                auto dp2 = (p2 - fp) * dir;
                auto dp3 = (p3 - fp) * dir;
                auto cpn2 = norm(cross_product(p2 - fp, dir));
                auto cpn3 = norm(cross_product(p3 - fp, dir));
                EQUAL(dp1v, dp2)
                EQUAL(dp1v, dp3)
                EQUAL(cpn1v, cpn2)
                EQUAL(cpn1v, cpn3)
            }

            // The resulting points should be equal to the original one.
            EQUAL(x, p2.x())
            EQUAL(y, p2.y())
            EQUAL(z, p2.z())
            EQUAL(x, p3.x())
            EQUAL(y, p3.y())
            EQUAL(z, p3.z())
        }

        { //// Projection ////
            // Create two line projectors, presumed equal, and apply each of them
            // to the original point.
            auto p2 = p1.mapped_by(affine_map3<TypeParam, Metre>
                ::projection_onto_line(fp, dir));
            auto p3 = p1.mapped_by(make_projection_onto_line(fp, dir));

            // Check that the resulting points belong to the projection line.
            EQUAL(0, norm(cross_product(dir, p2 - fp)))
            EQUAL(0, norm(cross_product(dir, p3 - fp)))

            // Check that the vectors from the resulting points to the fixed point
            // are orthogonal to the vectors from the resulting points
            // to the original point.
            EQUAL(0, (p2 - p1) * (p2 - fp))
            EQUAL(0, (p3 - p1) * (p3 - fp))

            // Create four plane projectors, presumed equal, and apply each of them
            // to the original point.
            auto a = dir.x().value();
            auto b = dir.y().value();
            auto c = dir.z().value();
            auto d = - a * fp.x().value() - b * fp.y().value()
                - c * fp.z().value();
            auto p4 = p1.mapped_by(affine_map3<TypeParam, Metre>
                ::projection_onto_plane(fp, dir));
            auto p5 = p1.mapped_by(affine_map3<TypeParam, Metre>
                ::projection_onto_plane(a, b, c, d));
            auto p6 = p1.mapped_by(make_projection_onto_plane(fp, dir));
            auto p7 = p1.mapped_by(make_projection_onto_plane<metres>(
                a, b, c, d));

            // Check that the resulting points belong to the specified plane.
            EQUAL(0, dir * (p4 - fp))
            EQUAL(0, dir * (p5 - fp))
            EQUAL(0, dir * (p6 - fp))
            EQUAL(0, dir * (p7 - fp))

            // Check that the difference between the resulting points and
            // the original one is orthogonal to the plane.
            EQUAL(0, norm(cross_product(p4 - p1, dir)))
            EQUAL(0, norm(cross_product(p5 - p1, dir)))
            EQUAL(0, norm(cross_product(p6 - p1, dir)))
            EQUAL(0, norm(cross_product(p7 - p1, dir)))
        }

        { //// Reflection ////
            // Create two line reflectors, presumed equal, and apply each of them
            // to the original point.
            auto p2 = p1.mapped_by(affine_map3<TypeParam, Metre>
                ::reflection_over_line(fp, dir));
            auto p3 = p1.mapped_by(make_reflection_over_line(fp, dir));

            // Check that the cross product between the difference between
            // the resulting points and the fixed point with the direction vector
            // is the opposite of that of the original point.
            auto result1 = cross_product(p1 - fp, dir);
            EQUAL(0, norm(cross_product(p2 - fp, dir) + result1))
            EQUAL(0, norm(cross_product(p3 - fp, dir) + result1))

            // Check that the vectors from the resulting points
            // to the original point are orthogonal to the projection line.
            EQUAL(0, (p2 - p1) * dir)
            EQUAL(0, (p3 - p1) * dir)

            // Create four plane reflectors, presumed equal, and apply each of them
            // to the original point.
            auto a = dir.x().value();
            auto b = dir.y().value();
            auto c = dir.z().value();
            auto d = - a * fp.x().value() - b * fp.y().value()
                - c * fp.z().value();
            auto p4 = p1.mapped_by(affine_map3<metres, TypeParam>
                ::reflection_over_plane(fp, dir));
            auto p5 = p1.mapped_by(affine_map3<metres, TypeParam>
                ::reflection_over_plane(a, b, c, d));
            auto p6 = p1.mapped_by(make_reflection_over_plane(fp, dir));
            auto p7 = p1.mapped_by(make_reflection_over_plane<metres>(
                a, b, c, d));

            // Check that the dot product of the difference between
            // the resulting points and the fixed point
            // with the normal is the opposite of that of the original point.
            EQUAL(0, dir * (p4 - fp) + dir * (p1 - fp))
            EQUAL(0, dir * (p5 - fp) + dir * (p1 - fp))
            EQUAL(0, dir * (p6 - fp) + dir * (p1 - fp))
            EQUAL(0, dir * (p7 - fp) + dir * (p1 - fp))

            // Check that the difference between the resulting points and
            // the original one is orthogonal to the plane.
            EQUAL(0, norm(cross_product(p4 - p1, dir)))
            EQUAL(0, norm(cross_product(p5 - p1, dir)))
            EQUAL(0, norm(cross_product(p6 - p1, dir)))
            EQUAL(0, norm(cross_product(p7 - p1, dir)))
        }

        { //// Scaling ////
            // Create some scalers, and apply each of them to the point.
            auto p2 = p1.mapped_by(affine_map3<TypeParam, Metre>
                ::scaling(fp, 0, 0, 0));
            EQUAL(fx, p2.x()) EQUAL(fy, p2.y()) EQUAL(fz, p2.z())
            p2 = p1.mapped_by(affine_map3<TypeParam, Metre>
                ::scaling(fp, 1, 0, 0));
            EQUAL(x, p2.x()) EQUAL(fy, p2.y()) EQUAL(fz, p2.z())
            p2 = p1.mapped_by(affine_map3<TypeParam, Metre>
                ::scaling(fp, 0, 1, 0));
            EQUAL(fx, p2.x()) EQUAL(y, p2.y()) EQUAL(fz, p2.z())
            p2 = p1.mapped_by(affine_map3<TypeParam, Metre>
                ::scaling(fp, 0, 0, 1));
            EQUAL(fx, p2.x()) EQUAL(fy, p2.y()) EQUAL(z, p2.z())
            p2 = p1.mapped_by(affine_map3<TypeParam, Metre>
                ::scaling(fp, 1, 1, 0));
            EQUAL(x, p2.x()) EQUAL(y, p2.y()) EQUAL(fz, p2.z())
            p2 = p1.mapped_by(affine_map3<TypeParam, Metre>
                ::scaling(fp, 1, 0, 1));
            EQUAL(x, p2.x()) EQUAL(fy, p2.y()) EQUAL(z, p2.z())
            p2 = p1.mapped_by(affine_map3<TypeParam, Metre>
                ::scaling(fp, 0, 1, 1));
            EQUAL(fx, p2.x()) EQUAL(y, p2.y()) EQUAL(z, p2.z())
            p2 = p1.mapped_by(affine_map3<TypeParam, Metre>
                ::scaling(fp, 1, 1, 1));
            EQUAL(x, p2.x()) EQUAL(y, p2.y()) EQUAL(z, p2.z())
            p2 = p1.mapped_by(affine_map3<TypeParam, Metre>
                ::scaling(fp, -3.7L, -1.921L, 2.84L));
            EQUAL(fx + (x - fx) * -3.7L, p2.x())
            EQUAL(fy + (y - fy) * -1.921L, p2.y())
            EQUAL(fz + (z - fz) * 2.84L, p2.z())
        }

        { //// Inversion ////
            affine_map3<TypeParam, Metre> am;

            // Set a non-invertible matrix.
            am.coeff(0, 0) = 2; am.coeff(0, 1) = 3; am.coeff(0, 2) = 5; am.coeff(0, 3) = 5;
            am.coeff(1, 0) = 6; am.coeff(1, 1) = 9; am.coeff(1, 2) = 15; am.coeff(1, 3) = -16;
            am.coeff(2, 0) = -6; am.coeff(2, 1) = 4; am.coeff(2, 2) = 4; am.coeff(2, 3) = 3;

            // Check that its inverse has all zeros.
            am = am.inverted();
            EXPECT_EQ(0, am.coeff(0, 0));
            EXPECT_EQ(0, am.coeff(0, 1));
            EXPECT_EQ(0, am.coeff(0, 2));
            EXPECT_EQ(0, am.coeff(0, 3));
            EXPECT_EQ(0, am.coeff(1, 0));
            EXPECT_EQ(0, am.coeff(1, 1));
            EXPECT_EQ(0, am.coeff(1, 2));
            EXPECT_EQ(0, am.coeff(1, 3));
            EXPECT_EQ(0, am.coeff(2, 0));
            EXPECT_EQ(0, am.coeff(2, 1));
            EXPECT_EQ(0, am.coeff(2, 2));
            EXPECT_EQ(0, am.coeff(2, 3));

            // Set an invertible matrix.
            am.coeff(0, 0) = 2; am.coeff(0, 1) = 3; am.coeff(0, 2) = 5; am.coeff(0, 3) = 5;
            am.coeff(1, 0) = 0; am.coeff(1, 1) = 9; am.coeff(1, 2) = 15; am.coeff(1, 3) = -16;
            am.coeff(2, 0) = -6; am.coeff(2, 1) = 4; am.coeff(2, 2) = 4; am.coeff(2, 3) = 3;

            // Check that by applying it before or after its inverse,
            // the same vector is obtained.
            auto inv_am = am.inverted();
            auto p2 = p1.mapped_by(am).mapped_by(inv_am);
            EQUAL(x, p2.x())
            EQUAL(y, p2.y())
            EQUAL(z, p2.z())

            p2 = p1.mapped_by(inv_am).mapped_by(am);
            EQUAL(x, p2.x())
            EQUAL(y, p2.y())
            EQUAL(z, p2.z())
        }

        { //// Composition ////
            // Create three affine 3D transformations.
            auto am1 = make_scaling(fp, TypeParam(3), TypeParam(4), TypeParam(5));
            auto am2 = make_projection_onto_line(fp, dir);
            auto am3 = make_rotation(fp, normalized(Measure3d<TypeParam, Metre>(3, 1, -5)),
                Measure<degrees,TypeParam>(147));

            // Apply them one at a time.
            auto p2 = p1.mapped_by(am1).mapped_by(am2).mapped_by(am3);

            // Combine the first with the second, and the result with the third,
            // and apply the result.
            auto p3 = p1.mapped_by(combine(combine(am1, am2), am3));

            // Combine the second with the third, and the first with the result,
            // and apply the result.
            auto p4 = p1.mapped_by(combine(am1, combine(am2, am3)));

            // Check that the three transformed vectors are the same.
            EQUAL(p2.x().value(), p3.x())
            EQUAL(p2.y().value(), p3.y())
            EQUAL(p2.z().value(), p3.z())
            EQUAL(p2.x().value(), p4.x())
            EQUAL(p2.y().value(), p4.y())
            EQUAL(p2.z().value(), p4.z())

            // Set an invertible matrix.
            am1.coeff(0, 0) = 2; am1.coeff(0, 1) = 3; am1.coeff(0, 2) = 5;
            am1.coeff(1, 0) = 0; am1.coeff(1, 1) = 9; am1.coeff(1, 2) = 15;
            am1.coeff(2, 0) = -6; am1.coeff(2, 1) = 4; am1.coeff(2, 2) = 4;

            // Combine it with its inverse,
            // and its inverse with it.
            // In both cases, the result should be the identity.
            auto am1_inv = am1.inverted();
            am2 = combine(am1, am1_inv);
            am3 = combine(am1_inv, am1);

            // Apply to a vector the resulting transformations.
            p2 = p1.mapped_by(am2);
            p3 = p1.mapped_by(am3);

            // Check that the resulting vectors are (almost) unchanged.
            EQUAL(x, p2.x())
            EQUAL(y, p2.y())
            EQUAL(z, p2.z())
            EQUAL(x, p3.x())
            EQUAL(y, p3.y())
            EQUAL(z, p3.z())
        }
    }

    TYPED_TEST(general_test, vect3)
    {
        auto epsilon = numeric_limits<TypeParam>::is_integer ?
            1 : numeric_limits<float>::epsilon();
        auto tolerance = Measure<f32, Metre>(epsilon * 2);
        auto zero = Measure<f32, Metre>(0);

        // Try several numeric values.
        for (int i = 0; i < test_values<TypeParam>::count; ++i)
        {
            TypeParam val1x = test_values<TypeParam>::v[i];

            // Try several numeric values.
            for (int j = 0; j < test_values<TypeParam>::count; ++j)
            {
                TypeParam val1y = test_values<TypeParam>::v[j];

                // Try several numeric values.
                for (int k = 0; k < test_values<TypeParam>::count; ++k)
                {
                    TypeParam val1z = test_values<TypeParam>::v[k];
                    TypeParam val2 = static_cast<TypeParam>(2.19);
                    TypeParam val2x = static_cast<TypeParam>(2.27);
                    TypeParam val2y = static_cast<TypeParam>(2.13);
                    TypeParam val2z = static_cast<TypeParam>(2.15);
                    TypeParam val3 = 3;
                    auto m1 = Measure3d<TypeParam, Metre>(val1x, val1y, val1z);
                    auto m2 = Measure3d<TypeParam, Metre>(val2x, val2y, val2z);

                    // Construction and "value".
                    EXPECT_EQ(val1x, m1.x().value());
                    EXPECT_EQ(val1y, m1.y().value());
                    EXPECT_EQ(val1z, m1.z().value());
                    EXPECT_EQ(val2x, m2.x().value());
                    EXPECT_EQ(val2y, m2.y().value());
                    EXPECT_EQ(val2z, m2.z().value());

                    // Operator +=.
                    // All test-values except the largest one,
                    // as it would cause overflow by incrementing.
                    if (i < test_values<TypeParam>::count - 1
                        && j < test_values<TypeParam>::count - 1
                        && k < test_values<TypeParam>::count - 1)
                    {
                        m1 = Measure3d<TypeParam, Metre>(val1x, val1y, val1z);
                        auto m3 = m1 += m2;
                        EXPECT_EQ(val1x + val2x, m1.x().value());
                        EXPECT_EQ(val1y + val2y, m1.y().value());
                        EXPECT_EQ(val1z + val2z, m1.z().value());
                        EXPECT_EQ(val2x, m2.x().value());
                        EXPECT_EQ(val2y, m2.y().value());
                        EXPECT_EQ(val2z, m2.z().value());
                        EXPECT_EQ(val1x + val2x, m3.x().value());
                        EXPECT_EQ(val1y + val2y, m3.y().value());
                        EXPECT_EQ(val1z + val2z, m3.z().value());
                    }

                    // Operator -=.
                    // All test-values except the lowest one,
                    // as it would cause overflow by decrementing.
                    if (0 < i && 0 < j && 0 < k)
                    {
                        m1 = Measure3d<TypeParam, Metre>(val1x, val1y, val1z);
                        auto m3 = m1 -= m2;
                        EQUAL(val1x - val2x, m1.x())
                        EQUAL(val1y - val2y, m1.y())
                        EQUAL(val1z - val2z, m1.z())
                        EQUAL(val2x, m2.x())
                        EQUAL(val2y, m2.y())
                        EQUAL(val2z, m2.z())
                        EQUAL(val1x - val2x, m3.x())
                        EQUAL(val1y - val2y, m3.y())
                        EQUAL(val1z - val2z, m3.z())
                    }

                    // Operator *=.
                    // All test-values except the lowest and the largest ones,
                    // as they would cause overflow by multiplying.
                    if (0 < i && i < test_values<TypeParam>::count - 1
                        && 0 < j && j < test_values<TypeParam>::count - 1
                        && 0 < k && k < test_values<TypeParam>::count - 1)
                    {
                        m1 = Measure3d<TypeParam, Metre>(val1x, val1y, val1z);
                        auto m3 = m1 *= val2;
                        EQUAL(val1x * val2, m1.x())
                        EQUAL(val1y * val2, m1.y())
                        EQUAL(val1z * val2, m1.z())
                        EQUAL(val1x * val2, m3.x())
                        EQUAL(val1y * val2, m3.y())
                        EQUAL(val1z * val2, m3.z())
                        auto m4 = m1 *= val3;
                        EQUAL(val1x * val2 * val3, m1.x())
                        EQUAL(val1y * val2 * val3, m1.y())
                        EQUAL(val1z * val2 * val3, m1.z())
                        EQUAL(val1x * val2 * val3, m4.x())
                        EQUAL(val1y * val2 * val3, m4.y())
                        EQUAL(val1z * val2 * val3, m4.z())
                    }

                    // Operator /=.
                    // All test-values.
                    if (abs(val2) != 0)
                    {
                        m1 = Measure3d<TypeParam, Metre>(val1x, val1y, val1z);
                        auto m3 = m1 /= val2;
                        EQUAL(val1x / val2, m1.x())
                        EQUAL(val1y / val2, m1.y())
                        EQUAL(val1z / val2, m1.z())
                        EQUAL(val1x / val2, m3.x())
                        EQUAL(val1y / val2, m3.y())
                        EQUAL(val1z / val2, m3.z())
                        if (abs(val3) != 0)
                        {
                            auto m4 = m1 /= val3;
                            EQUAL(val1x / val2 / val3, m1.x())
                            EQUAL(val1y / val2 / val3, m1.y())
                            EQUAL(val1z / val2 / val3, m1.z())
                            EQUAL(val1x / val2 / val3, m4.x())
                            EQUAL(val1y / val2 / val3, m4.y())
                            EQUAL(val1z / val2 / val3, m4.z())
                        }
                    }

                    // Relational operators.

                    // Comparing equal measures.
                    m1 = Measure3d<TypeParam, Metre>(val1x, val1y, val1z);
                    m2 = m1;
                    EXPECT_TRUE(m1 == m2);
                    EXPECT_FALSE(m1 != m2);

                    // All test-values except the two lowest and the two largest ones,
                    // as they would cause overflow by squaring.
                    if (2 <= i && i < test_values<TypeParam>::count - 2
                        && 2 <= j && j < test_values<TypeParam>::count - 2
                        && 2 <= k && k < test_values<TypeParam>::count - 2)
                    {
                        EXPECT_TRUE(is_equal(m1, m2, zero));
                        EXPECT_TRUE(is_equal(m1, m2, tolerance));

                        // Comparing a measure with its half, if it is large,
                        // with its double if it is small.
                        if (abs(m2.x().value()) >= MIN_THRESHOLD)
                        {
                            if (abs(squared_norm_value(m2)) > 1.f) m2 /= 2;
                            else m1 *= 2;
                            EXPECT_FALSE(m1 == m2);
                            EXPECT_TRUE(m1 != m2);
                            EXPECT_FALSE(is_equal(m1, m2, zero));
                        }
                    }
                }
            }
        }

        // Comparing 1 with a bit more than 1.
        auto m3 = Measure3d<TypeParam, Metre>(1, 1, 1);
        auto m4 = Measure3d<TypeParam, Metre>(1 + epsilon, 1, 1);
        auto m4b = Measure3d<TypeParam, Metre>(1, 1 + epsilon, 1);
        auto m4c = Measure3d<TypeParam, Metre>(1, 1, 1 + epsilon);
        EXPECT_FALSE(m3 == m4);
        EXPECT_FALSE(m3 == m4b);
        EXPECT_FALSE(m3 == m4c);
        EXPECT_TRUE(m3 != m4);
        EXPECT_TRUE(m3 != m4b);
        EXPECT_TRUE(m3 != m4c);
        EXPECT_TRUE(is_equal(m3, m4, tolerance));
        EXPECT_TRUE(is_equal(m3, m4b, tolerance));
        EXPECT_TRUE(is_equal(m3, m4c, tolerance));

        // Comparing -1 with a bit less than -1.
        auto m5 = Measure3d<TypeParam, Metre>(-1, -1, -1);
        auto m6 = Measure3d<TypeParam, Metre>(-1 - epsilon, -1, -1);
        auto m6b = Measure3d<TypeParam, Metre>(-1, -1 - epsilon, -1);
        auto m6c = Measure3d<TypeParam, Metre>(-1, -1, -1 - epsilon);
        EXPECT_FALSE(m5 == m6);
        EXPECT_FALSE(m5 == m6b);
        EXPECT_FALSE(m5 == m6c);
        EXPECT_TRUE(m5 != m6);
        EXPECT_TRUE(m5 != m6b);
        EXPECT_TRUE(m5 != m6c);
        EXPECT_TRUE(is_equal(m5, m6, tolerance));
        EXPECT_TRUE(is_equal(m5, m6b, tolerance));
        EXPECT_TRUE(is_equal(m5, m6c, tolerance));

        // Compare non-extreme numbers, far from zero,
        // with a small tolerance or with a large tolerance.
        auto m7 = Measure3d<TypeParam, Metre>(4, 4, 4);
        auto m8 = Measure3d<TypeParam, Metre>(6, 6, 6);
        auto tol1 = Measure<f32, Metre>(3);
        auto tol2 = Measure<f32, Metre>(4);
        EXPECT_FALSE(is_equal(m7, m8, tol1));
        EXPECT_FALSE(is_equal(m8, m7, tol1));
        EXPECT_TRUE(is_equal(m7, m8, tol2));
        EXPECT_TRUE(is_equal(m8, m7, tol2));
    }

    TYPED_TEST(general_test, point3)
    {
        auto epsilon = numeric_limits<TypeParam>::is_integer ?
            1 : numeric_limits<float>::epsilon();
        auto tolerance = Measure<f32, Metre>(epsilon * 2);
        auto zero = Measure<f32, Metre>(0);

        // Try several numeric values.
        for (int i = 0; i < test_values<TypeParam>::count; ++i)
        {
            TypeParam val1x = test_values<TypeParam>::v[i];

            // Try several numeric values.
            for (int j = 0; j < test_values<TypeParam>::count; ++j)
            {
                TypeParam val1y = test_values<TypeParam>::v[j];

                // Try several numeric values.
                for (int k = 0; k < test_values<TypeParam>::count; ++k)
                {
                    TypeParam val1z = test_values<TypeParam>::v[k];
                    TypeParam val2x = static_cast<TypeParam>(2.27);
                    TypeParam val2y = static_cast<TypeParam>(2.13);
                    TypeParam val2z = static_cast<TypeParam>(2.15);
                    auto m1 = MeasurePoint3d<TypeParam, Metre>(val1x, val1y, val1z);
                    auto m2 = Measure3d<TypeParam, Metre>(val2x, val2y, val2z);

                    // Construction and "value".
                    EXPECT_EQ(val1x, m1.x().value());
                    EXPECT_EQ(val1y, m1.y().value());
                    EXPECT_EQ(val1z, m1.z().value());
                    EXPECT_EQ(val2x, m2.x().value());
                    EXPECT_EQ(val2y, m2.y().value());
                    EXPECT_EQ(val2z, m2.z().value());

                    // Operator +=.
                    // All test-values except the largest one,
                    // as it would cause overflow by incrementing.
                    if (i < test_values<TypeParam>::count - 1
                        && j < test_values<TypeParam>::count - 1
                        && k < test_values<TypeParam>::count - 1)
                    {
                        m1 = MeasurePoint3d<TypeParam, Metre>(val1x, val1y, val1z);
                        auto m3 = m1 += m2;
                        EXPECT_EQ(val1x + val2x, m1.x().value());
                        EXPECT_EQ(val1y + val2y, m1.y().value());
                        EXPECT_EQ(val1z + val2z, m1.z().value());
                        EXPECT_EQ(val2x, m2.x().value());
                        EXPECT_EQ(val2y, m2.y().value());
                        EXPECT_EQ(val2z, m2.z().value());
                        EXPECT_EQ(val1x + val2x, m3.x().value());
                        EXPECT_EQ(val1y + val2y, m3.y().value());
                        EXPECT_EQ(val1z + val2z, m3.z().value());
                    }

                    // Operator -=.
                    // All test-values except the lowest one,
                    // as it would cause overflow by decrementing.
                    if (0 < i && 0 < j && 0 < k)
                    {
                        m1 = MeasurePoint3d<TypeParam, Metre>(val1x, val1y, val1z);
                        auto m3 = m1 -= m2;
                        EXPECT_EQ(val1x - val2x, m1.x().value());
                        EXPECT_EQ(val1y - val2y, m1.y().value());
                        EXPECT_EQ(val1z - val2z, m1.z().value());
                        EXPECT_EQ(val2x, m2.x().value());
                        EXPECT_EQ(val2y, m2.y().value());
                        EXPECT_EQ(val2z, m2.z().value());
                        EXPECT_EQ(val1x - val2x, m3.x().value());
                        EXPECT_EQ(val1y - val2y, m3.y().value());
                        EXPECT_EQ(val1z - val2z, m3.z().value());
                    }

                    // Relational operators.

                    // Comparing equal measures.
                    m1 = MeasurePoint3d<TypeParam, Metre>(val1x, val1y, val1z);
                    auto m3 = m1;
                    EXPECT_TRUE(m1 == m3);
                    EXPECT_FALSE(m1 != m3);

                    // All test-values except the two lowest and the two largest ones,
                    // as they would cause overflow by squaring.
                    if (2 <= i && i < test_values<TypeParam>::count - 2
                        && 2 <= j && j < test_values<TypeParam>::count - 2
                        && 2 <= k && k < test_values<TypeParam>::count - 2)
                    {
                        EXPECT_TRUE(is_equal(m1, m3, zero));
                        EXPECT_TRUE(is_equal(m1, m3, tolerance));

                        // Comparing a measure with its half, if it is large,
                        // with its double if it is small.
                        if (abs(m3.x().value()) >= MIN_THRESHOLD)
                        {
                            auto two = static_cast<TypeParam>(2);
                            if (abs(m3.x().value()) + abs(m3.y().value()) + abs(m3.z().value()) > 2)
                                m3 = MeasurePoint3d<TypeParam, Metre>(m3.x().value() / two, m3.y().value() / two, m3.z().value() / two);
                            else m1 = MeasurePoint3d<TypeParam, Metre>(m1.x().value() * two, m1.y().value() * two, m1.z().value() * two);
                            EXPECT_FALSE(m1 == m3);
                            EXPECT_TRUE(m1 != m3);
                            EXPECT_FALSE(is_equal(m1, m3, zero));
                        }
                    }
                }
            }
        }

        // Comparing 1 with a bit more than 1.
        auto m3 = MeasurePoint3d<TypeParam, Metre>(1, 1, 1);
        auto m4 = MeasurePoint3d<TypeParam, Metre>(1 + epsilon, 1 + epsilon, 1 + epsilon);
        EXPECT_FALSE(m3 == m4);
        EXPECT_TRUE(m3 != m4);
        EXPECT_TRUE(is_equal(m3, m4, tolerance));

        // Comparing -1 with a bit less than -1.
        auto m5 = MeasurePoint3d<TypeParam, Metre>(-1, -1, -1);
        auto m6 = MeasurePoint3d<TypeParam, Metre>(-1 - epsilon, -1 - epsilon, -1 - epsilon);
        EXPECT_FALSE(m5 == m6);
        EXPECT_TRUE(m5 != m6);
        EXPECT_TRUE(is_equal(m5, m6, tolerance));

        // Compare non-extreme numbers, far from zero,
        // with a small tolerance or with a large tolerance.
        auto m7 = MeasurePoint3d<TypeParam, Metre>(4, 4, 4);
        auto m8 = MeasurePoint3d<TypeParam, Metre>(6, 6, 6);
        auto tol1 = Measure<f32, Metre>(3);
        auto tol2 = Measure<f32, Metre>(4);
        EXPECT_FALSE(is_equal(m7, m8, tol1));
        EXPECT_FALSE(is_equal(m8, m7, tol1));
        EXPECT_TRUE(is_equal(m7, m8, tol2));
        EXPECT_TRUE(is_equal(m8, m7, tol2));
    }

    TYPED_TEST(general_test, vectpoint3)
    {
        // Midpoint.
        // Try all triples of pairs of numeric values except extremes.
        auto fraction = abs(static_cast<TypeParam>(0.23f));
        for (int i1 = 1; i1 < test_values<TypeParam>::count - 1; ++i1)
        {
            TypeParam val1x = test_values<TypeParam>::v[i1];
            for (int j1 = 1; j1 < test_values<TypeParam>::count - 1; ++j1)
            {
                TypeParam val1y = test_values<TypeParam>::v[j1];
                for (int k1 = 1; k1 < test_values<TypeParam>::count - 1; ++k1)
                {
                    TypeParam val1z = test_values<TypeParam>::v[k1];
                    auto m1 = MeasurePoint3d<TypeParam, Metre>(val1x, val1y, val1z);
                    for (int i2 = 1; i2 < test_values<TypeParam>::count - 1; ++i2)
                    {
                        TypeParam val2x = test_values<TypeParam>::v[i2];
                        for (int j2 = 1; j2 < test_values<TypeParam>::count - 1; ++j2)
                        {
                            TypeParam val2y = test_values<TypeParam>::v[j2];
                            for (int k2 = 1; k2 < test_values<TypeParam>::count - 1; ++k2)
                            {
                                TypeParam val2z = test_values<TypeParam>::v[k2];
                                auto m2 = MeasurePoint3d<TypeParam, Metre>(val2x, val2y, val2z);
                                auto two = static_cast<TypeParam>(2);
                                EQUAL((val1x + val2x) / two, midpoint(m1, m2).x())
                                EQUAL((val1y + val2y) / two, midpoint(m1, m2).y())
                                EQUAL((val1z + val2z) / two, midpoint(m1, m2).z())
                                EQUAL(val1x, midpoint(m1, m2, 0.f).x())
                                EQUAL(val1y, midpoint(m1, m2, 0.f).y())
                                EQUAL(val1z, midpoint(m1, m2, 0.f).z())
                                EQUAL(val2x, midpoint(m1, m2, 1.f).x())
                                EQUAL(val2y, midpoint(m1, m2, 1.f).y())
                                EQUAL(val2z, midpoint(m1, m2, 1.f).z())
                                EQUAL(val1x * (1 - fraction) + val2x * fraction, midpoint(m1, m2, fraction).x())
                                EQUAL(val1y * (1 - fraction) + val2y * fraction, midpoint(m1, m2, fraction).y())
                                EQUAL(val1z * (1 - fraction) + val2z * fraction, midpoint(m1, m2, fraction).z())
                            }
                        }
                    }
                }
            }
        }

        // Barycentric combination.
        // Try all triples of pairs of numeric values except extremes,
        // and add some other points.
        for (int i1 = 1; i1 < test_values<TypeParam>::count - 1; ++i1)
        {
            TypeParam val1x = test_values<TypeParam>::v[i1];
            for (int j1 = 1; j1 < test_values<TypeParam>::count - 1; ++j1)
            {
                TypeParam val1y = test_values<TypeParam>::v[j1];
                for (int k1 = 1; k1 < test_values<TypeParam>::count - 1; ++k1)
                {
                    TypeParam val1z = test_values<TypeParam>::v[k1];
                    auto m1 = MeasurePoint3d<TypeParam, Metre>(val1x, val1y, val1z);
                    for (int i2 = 1; i2 < test_values<TypeParam>::count - 1; ++i2)
                    {
                        TypeParam val2x = test_values<TypeParam>::v[i2];
                        for (int j2 = 1; j2 < test_values<TypeParam>::count - 1; ++j2)
                        {
                            TypeParam val2y = test_values<TypeParam>::v[j2];
                            for (int k2 = 1; k2 < test_values<TypeParam>::count - 1; ++k2)
                            {
                                TypeParam val2z = test_values<TypeParam>::v[k2];
                                auto m2 = MeasurePoint3d<TypeParam, Metre>(val2x, val2y, val2z);

                                TypeParam val3x = static_cast<TypeParam>(3.1);
                                TypeParam val3y = static_cast<TypeParam>(3.2);
                                TypeParam val3z = static_cast<TypeParam>(3.3);
                                auto m3 = MeasurePoint3d<TypeParam, Metre>(val3x, val3y, val3z);
                                TypeParam val4x = static_cast<TypeParam>(4.3);
                                TypeParam val4y = static_cast<TypeParam>(4.4);
                                TypeParam val4z = static_cast<TypeParam>(4.5);
                                auto m4 = MeasurePoint3d<TypeParam, Metre>(val4x, val4y, val4z);
                                MeasurePoint3d<TypeParam, Metre> point3array[] = { m1, m2, m3, m4 };
                                TypeParam weights[] = { 2, 3, 7, 4 };
                                EQUAL(val1x * weights[0],
                                    barycentric_combination(1, point3array, weights).x())
                                EQUAL(val1y * weights[0],
                                    barycentric_combination(1, point3array, weights).y())
                                EQUAL(val1z * weights[0],
                                    barycentric_combination(1, point3array, weights).z())
                                EQUAL(val1x * weights[0] + val2x * weights[1],
                                    barycentric_combination(2, point3array, weights).x())
                                EQUAL(val1y * weights[0] + val2y * weights[1],
                                    barycentric_combination(2, point3array, weights).y())
                                EQUAL(val1z * weights[0] + val2z * weights[1],
                                    barycentric_combination(2, point3array, weights).z())
                                EQUAL(val1x * weights[0] + val2x * weights[1]
                                    + val3x * weights[2],
                                    barycentric_combination(3, point3array, weights).x())
                                EQUAL(val1y * weights[0] + val2y * weights[1]
                                    + val3y * weights[2],
                                    barycentric_combination(3, point3array, weights).y())
                                EQUAL(val1z * weights[0] + val2z * weights[1]
                                    + val3z * weights[2],
                                    barycentric_combination(3, point3array, weights).z())
                                EQUAL(val1x * weights[0] + val2x * weights[1]
                                    + val3x * weights[2] + val4x * weights[3],
                                    barycentric_combination(4, point3array, weights).x())
                                EQUAL(val1y * weights[0] + val2y * weights[1]
                                    + val3y * weights[2] + val4y * weights[3],
                                    barycentric_combination(4, point3array, weights).y())
                                EQUAL(val1z * weights[0] + val2z * weights[1]
                                    + val3z * weights[2] + val4z * weights[3],
                                    barycentric_combination(4, point3array, weights).z())
                            }
                        }
                    }
                }
            }
        }

        // Try all triples of pairs of numeric values except extremes.
        auto sqrt_of_max = abs(static_cast<TypeParam>(std::sqrt(test_values<TypeParam>
            ::v[test_values<TypeParam>::count - 1])));
        auto sqrt_of_min = abs(static_cast<TypeParam>(std::sqrt(test_values<TypeParam>
            ::v[(test_values<TypeParam>::count + 1) / 2])));
        for (int i1 = 1; i1 < test_values<TypeParam>::count - 1; ++i1)
        {
            TypeParam val1x = test_values<TypeParam>::v[i1];
            for (int j1 = 1; j1 < test_values<TypeParam>::count - 1; ++j1)
            {
                TypeParam val1y = test_values<TypeParam>::v[j1];
                for (int k1 = 1; k1 < test_values<TypeParam>::count - 1; ++k1)
                {
                    TypeParam val1z = test_values<TypeParam>::v[k1];
                    auto v1 = Measure3d<TypeParam, Metre>(val1x, val1y, val1z);
                    auto p1 = MeasurePoint3d<TypeParam, Metre>(val1x, val1y, val1z);

                    for (int i2 = 1; i2 < test_values<TypeParam>::count - 1; ++i2)
                    {
                        TypeParam val2x = test_values<TypeParam>::v[i2];
                        for (int j2 = 1; j2 < test_values<TypeParam>::count - 1; ++j2)
                        {
                            TypeParam val2y = test_values<TypeParam>::v[j2];
                            for (int k2 = 1; k2 < test_values<TypeParam>::count - 1; ++k2)
                            {
                                TypeParam val2z = test_values<TypeParam>::v[k2];
                                auto v2 = Measure3d<TypeParam, Metre>(val2x, val2y, val2z);
                                auto p2 = MeasurePoint3d<TypeParam, Metre>(val2x, val2y, val2z);

                                // point3 - point3 -> vect3
                                EXPECT_EQ(val1x - val2x, (p1 - p2).x().value());
                                EXPECT_EQ(val1y - val2y, (p1 - p2).y().value());
                                EXPECT_EQ(val1z - val2z, (p1 - p2).z().value());

                                // point3 + vect3 -> point3
                                EXPECT_EQ(val1x + val2x, (p1 + v2).x().value());
                                EXPECT_EQ(val1y + val2y, (p1 + v2).y().value());
                                EXPECT_EQ(val1z + val2z, (p1 + v2).z().value());

                                // point3 - vect3 -> point3
                                EXPECT_EQ(val1x - val2x, (p1 - v2).x().value());
                                EXPECT_EQ(val1y - val2y, (p1 - v2).y().value());
                                EXPECT_EQ(val1z - val2z, (p1 - v2).z().value());

                                // vect3 + vect3 -> vect3
                                EXPECT_EQ(val1x + val2x, (v1 + v2).x().value());
                                EXPECT_EQ(val1y + val2y, (v1 + v2).y().value());
                                EXPECT_EQ(val1z + val2z, (v1 + v2).z().value());

                                // vect3 - vect3 -> vect3
                                EXPECT_EQ(val1x - val2x, (v1 - v2).x().value());
                                EXPECT_EQ(val1y - val2y, (v1 - v2).y().value());
                                EXPECT_EQ(val1z - val2z, (v1 - v2).z().value());

                                if (abs(val1x) < sqrt_of_max && abs(val2x) < sqrt_of_max)
                                {
                                    // N * vect3 -> vect3
                                    EXPECT_EQ(val1x * val2x, (val1x * v2).x().value());

                                    // vect3 * N -> vect3
                                    EXPECT_EQ(val1x * val2x, (v1 * val2x).x().value());
                                }

                                if (abs(val1y) < sqrt_of_max && abs(val2y) < sqrt_of_max)
                                {
                                    // N * vect3 -> vect3
                                    EXPECT_EQ(val1y * val2y, (val1y * v2).y().value());

                                    // vect3 * N -> vect3
                                    EXPECT_EQ(val1y * val2y, (v1 * val2y).y().value());
                                }

                                if (abs(val1z) < sqrt_of_max && abs(val2z) < sqrt_of_max)
                                {
                                    // N * vect3 -> vect3
                                    EXPECT_EQ(val1z * val2z, (val1z * v2).z().value());

                                    // vect3 * N -> vect3
                                    EXPECT_EQ(val1z * val2z, (v1 * val2z).z().value());
                                }

                                // vect3 / N -> vect3
                                if (abs(val2x) != 0) EXPECT_EQ(val1x / val2x, (v1 / val2x).x().value());
                                if (abs(val2y) != 0) EXPECT_EQ(val1y / val2y, (v1 / val2y).y().value());
                                if (abs(val2z) != 0) EXPECT_EQ(val1z / val2z, (v1 / val2z).z().value());
                            }
                        }
                    }
                }
            }
        }

        for (int i = 2; i < test_values<TypeParam>::count - 2; ++i)
        {
            for (int j = 2; j < test_values<TypeParam>::count - 2; ++j)
            {
                for (int k = 2; k < test_values<TypeParam>::count - 2; ++k)
                {
                    TypeParam val1x = test_values<TypeParam>::v[i];
                    TypeParam val1y = test_values<TypeParam>::v[j];
                    TypeParam val1z = test_values<TypeParam>::v[k];
                    auto m1 = Measure3d<units, TypeParam>(val1x, val1y, val1z);
                    if (abs(val1x) < sqrt_of_max && abs(val1y) < sqrt_of_max && abs(val1z) < sqrt_of_max
                        && abs(val1x) > sqrt_of_min && abs(val1y) > sqrt_of_min && abs(val1z) > sqrt_of_min)
                    {
                        EQUAL(val1x * val1x + val1y * val1y + val1z * val1z, m1 * m1)
                        EQUAL(static_cast<TypeParam>(
                            std::sqrt(val1x * val1x + val1y * val1y + val1z * val1z)), norm(m1))
                    }
                }
            }
        }
    }

    //// Directions
    TYPED_TEST(comparison_test, SignedDirection)
    {
        auto epsilon = numeric_limits<TypeParam>::is_integer ?
            1 : numeric_limits<float>::epsilon();
        auto tolerance = Measure<degrees,float>(epsilon * 2);
        auto zero = Measure<degrees,float>(0);

        // Try several numeric values except extremes.
        for (int i = 1; i < test_values<TypeParam>::count - 1; ++i)
        {
            TypeParam val1 = test_values<TypeParam>::v[i];
            TypeParam val2 = static_cast<TypeParam>(2.19);
            if (abs(val1) < MAX_THRESHOLD && abs(val2) < MAX_THRESHOLD)
            {
                auto m1 = SignedDirection<degrees,TypeParam>(val1);
                auto m2 = Measure<degrees,TypeParam>(val2);

                // Construction and "value".
                EXPECT_NEAR(0, abs(modulo(val1 + static_cast<TypeParam>(180), static_cast<TypeParam>(360)) - static_cast<TypeParam>(180) - m1.value()), AZIMUTH_TOLERANCE);
                EXPECT_EQ(val2, m2.value());

                // Operator +=.
                // All test-values except the largest one,
                // as it would cause overflow by incrementing.
                if (i < test_values<TypeParam>::count - 1)
                {
                    m1 = SignedDirection<degrees,TypeParam>(val1);
                    auto m3 = m1 += m2;
                    EXPECT_NEAR(0, abs(modulo(val1 + val2 + static_cast<TypeParam>(180), static_cast<TypeParam>(360)) - static_cast<TypeParam>(180) - m1.value()), AZIMUTH_TOLERANCE);
                    EXPECT_NEAR(0, abs(modulo(val1 + val2 + static_cast<TypeParam>(180), static_cast<TypeParam>(360)) - static_cast<TypeParam>(180) - m3.value()), AZIMUTH_TOLERANCE);
                }

                // Operator -=.
                // All test-values except the lowest one,
                // as it would cause overflow by decrementing.
                if (0 < i)
                {
                    m1 = SignedDirection<degrees,TypeParam>(val1);
                    auto m3 = m1 -= m2;
                    EXPECT_NEAR(0, abs(modulo(val1 - val2 + static_cast<TypeParam>(180), static_cast<TypeParam>(360)) - static_cast<TypeParam>(180) - m1.value()), AZIMUTH_TOLERANCE);
                    EXPECT_NEAR(0, abs(modulo(val1 - val2 + static_cast<TypeParam>(180), static_cast<TypeParam>(360)) - static_cast<TypeParam>(180) - m3.value()), AZIMUTH_TOLERANCE);
                }
            }

            // Relational operators.

            // Comparing equal measures.
            auto m1 = SignedDirection<degrees,TypeParam>(val1);
            auto m3 = m1;
            EXPECT_TRUE(m1 == m3);
            EXPECT_FALSE(m1 != m3);
            EXPECT_TRUE(is_equal(m1, m3, zero));

            // Comparing equal measures with a tolerance.
            // Avoid extreme test-values with a non-zero tolerance,
            // as they would cause overflow by incrementing or decrementing.
            if (0 < i && i < test_values<TypeParam>::count - 1)
            {
                EXPECT_TRUE(is_equal(m1, m3, tolerance));
            }

            // Comparing a measure with its half, if it is large,
            // with its double if it is small.
            // Omit test if it is near zero or larger than a turn.
            if (abs(val1) >= MIN_THRESHOLD && abs(val1) <= 360)
            {
                if (abs(val1) > 2)
                    m3 = SignedDirection<degrees,TypeParam>(val1 / static_cast<TypeParam>(2));
                else m1 = SignedDirection<degrees,TypeParam>(val1 * static_cast<TypeParam>(2));

                EXPECT_FALSE(m1 == m3);
                EXPECT_TRUE(m1 != m3);
                EXPECT_FALSE(is_equal(m1, m3, zero));
            }
        }

        // Comparing 1 with a bit more than 1.
        auto m3 = SignedDirection<degrees,TypeParam>(1);
        auto m4 = SignedDirection<degrees,TypeParam>(1 + epsilon);
        EXPECT_FALSE(m3 == m4);
        EXPECT_TRUE(m3 != m4);
        EXPECT_TRUE(is_equal(m3, m4, tolerance));

        // Comparing -1 with a bit less than -1.
        auto m5 = SignedDirection<degrees,TypeParam>(-1);
        auto m6 = SignedDirection<degrees,TypeParam>(-1 - epsilon);
        EXPECT_FALSE(m5 == m6);
        EXPECT_TRUE(m5 != m6);
        EXPECT_TRUE(is_equal(m5, m6, tolerance));

        // Compare non-extreme numbers, far from zero,
        // with a small tolerance or with a large tolerance.
        auto m7 = SignedDirection<degrees,TypeParam>(45);
        auto m8 = SignedDirection<degrees,TypeParam>(48);
        auto tol1 = Measure<degrees,float>(2);
        auto tol2 = Measure<degrees,float>(4);
        EXPECT_FALSE(is_equal(m7, m8, tol1));
        EXPECT_FALSE(is_equal(m8, m7, tol1));
        EXPECT_TRUE(is_equal(m7, m8, tol2));
        EXPECT_TRUE(is_equal(m8, m7, tol2));

        // Try all pairs of numeric values except extremes.
        for (int i1 = 1; i1 < test_values<TypeParam>::count - 1; ++i1)
        {
            TypeParam val1 = test_values<TypeParam>::v[i1];
            auto p1 = SignedDirection<degrees,TypeParam>(val1);
            for (int i2 = 1; i2 < test_values<TypeParam>::count - 1; ++i2)
            {
                TypeParam val2 = test_values<TypeParam>::v[i2];
                if (abs(val1) < MAX_THRESHOLD && abs(val2) < MAX_THRESHOLD
                    && abs(val1) > MIN_THRESHOLD && abs(val2) > MIN_THRESHOLD)
                {
                    auto v2 = Measure<degrees,TypeParam>(val2);
                    auto p2 = SignedDirection<degrees,TypeParam>(val2);

                    // SignedDirection - SignedDirection -> vect1
                    EXPECT_NEAR(0, abs(modulo(val1 - val2 + static_cast<TypeParam>(180), static_cast<TypeParam>(360)) - static_cast<TypeParam>(180)
                        - (p1 - p2).value()), AZIMUTH_TOLERANCE);

                    // SignedDirection + vect1 -> SignedDirection
                    EXPECT_NEAR(0, abs(modulo(val1 + val2 + static_cast<TypeParam>(180), static_cast<TypeParam>(360)) - static_cast<TypeParam>(180)
                        - (p1 + v2).value()), AZIMUTH_TOLERANCE);

                    // SignedDirection - vect1 -> SignedDirection
                    EXPECT_NEAR(0, abs(modulo(val1 - val2 + static_cast<TypeParam>(180), static_cast<TypeParam>(360)) - static_cast<TypeParam>(180)
                        - (p1 - v2).value()), AZIMUTH_TOLERANCE);
                }
            }
        }
    }
    ///////////////
    TYPED_TEST(comparison_test, UnsignedDirection)
    {
        auto epsilon = numeric_limits<TypeParam>::is_integer ?
            1 : numeric_limits<float>::epsilon();
        auto tolerance = Measure<degrees,float>(epsilon * 2);
        auto zero = Measure<degrees,float>(0);

        // Try several numeric values.
        for (int i = 0; i < test_values<TypeParam>::count; ++i)
        {
            TypeParam val1 = test_values<TypeParam>::v[i];
            TypeParam val2 = static_cast<TypeParam>(2.19);
            if (abs(val1) < MAX_THRESHOLD && abs(val2) < MAX_THRESHOLD)
            {
                auto m1 = UnsignedDirection<degrees,TypeParam>(val1);
                auto m2 = Measure<degrees,TypeParam>(val2);

                // Construction and "value".
                EXPECT_NEAR(0, abs(modulo(val1, static_cast<TypeParam>(360)) - m1.value()), AZIMUTH_TOLERANCE);
                EXPECT_EQ(val2, m2.value());

                // Operator +=.
                // All test-values except the largest one,
                // as it would cause overflow by incrementing.
                if (i < test_values<TypeParam>::count - 1)
                {
                    m1 = UnsignedDirection<degrees,TypeParam>(val1);
                    auto m3 = m1 += m2;
                    EXPECT_NEAR(0, abs(modulo(val1 + val2, static_cast<TypeParam>(360)) - m1.value()), AZIMUTH_TOLERANCE);
                    EXPECT_NEAR(0, abs(modulo(val1 + val2, static_cast<TypeParam>(360)) - m3.value()), AZIMUTH_TOLERANCE);
                }

                // Operator -=.
                // All test-values except the lowest one,
                // as it would cause overflow by decrementing.
                if (0 < i)
                {
                    m1 = UnsignedDirection<degrees,TypeParam>(val1);
                    auto m3 = m1 -= m2;
                    EXPECT_NEAR(0, abs(modulo(val1 - val2, static_cast<TypeParam>(360)) - m1.value()), AZIMUTH_TOLERANCE);
                    EXPECT_NEAR(0, abs(modulo(val1 - val2, static_cast<TypeParam>(360)) - m3.value()), AZIMUTH_TOLERANCE);
                }
            }

            // Relational operators.

            // Comparing equal measures.
            auto m1 = UnsignedDirection<degrees,TypeParam>(val1);
            auto m3 = m1;
            EXPECT_TRUE(m1 == m3);
            EXPECT_FALSE(m1 != m3);
            EXPECT_TRUE(is_equal(m1, m3, zero));

            // Comparing equal measures with a tolerance.
            // Avoid extreme test-values with a non-zero tolerance,
            // as they would cause overflow by incrementing or decrementing.
            if (0 < i && i < test_values<TypeParam>::count - 1)
            {
                EXPECT_TRUE(is_equal(m1, m3, tolerance));
            }

            // Comparing a measure with its half, if it is large,
            // with its double if it is small.
            // Omit test if it is near zero or larger than a turn.
            if (abs(val1) >= MIN_THRESHOLD && abs(val1) <= 360)
            {
                if (abs(val1) > 2)
                    m3 = UnsignedDirection<degrees,TypeParam>(val1 / static_cast<TypeParam>(2));
                else m1 = UnsignedDirection<degrees,TypeParam>(val1 * static_cast<TypeParam>(2));

                EXPECT_FALSE(m1 == m3);
                EXPECT_TRUE(m1 != m3);
                EXPECT_FALSE(is_equal(m1, m3, zero));
            }
        }

        // Comparing 1 with a bit more than 1.
        auto m3 = UnsignedDirection<degrees,TypeParam>(1);
        auto m4 = UnsignedDirection<degrees,TypeParam>(1 + epsilon);
        EXPECT_FALSE(m3 == m4);
        EXPECT_TRUE(m3 != m4);
        EXPECT_TRUE(is_equal(m3, m4, tolerance));

        // Comparing -1 with a bit less than -1.
        auto m5 = UnsignedDirection<degrees,TypeParam>(-1);
        auto m6 = UnsignedDirection<degrees,TypeParam>(-1 - epsilon);
        EXPECT_TRUE(is_equal(m5, m6, tolerance));

        // Compare non-extreme numbers, far from zero,
        // with a small tolerance or with a large tolerance.
        auto m7 = UnsignedDirection<degrees,TypeParam>(45);
        auto m8 = UnsignedDirection<degrees,TypeParam>(48);
        auto tol1 = Measure<degrees,float>(2);
        auto tol2 = Measure<degrees,float>(4);
        EXPECT_FALSE(is_equal(m7, m8, tol1));
        EXPECT_FALSE(is_equal(m8, m7, tol1));
        EXPECT_TRUE(is_equal(m7, m8, tol2));
        EXPECT_TRUE(is_equal(m8, m7, tol2));

        // Try all pairs of numeric values except extremes.
        for (int i1 = 1; i1 < test_values<TypeParam>::count - 1; ++i1)
        {
            TypeParam val1 = test_values<TypeParam>::v[i1];
            auto p1 = UnsignedDirection<degrees,TypeParam>(val1);
            for (int i2 = 1; i2 < test_values<TypeParam>::count - 1; ++i2)
            {
                TypeParam val2 = test_values<TypeParam>::v[i2];
                if (abs(val1) < MAX_THRESHOLD && abs(val2) < MAX_THRESHOLD
                    && abs(val1) > MIN_THRESHOLD && abs(val2) > MIN_THRESHOLD)
                {
                    auto v2 = Measure<degrees,TypeParam>(val2);

                    auto p2 = UnsignedDirection<degrees,TypeParam>(val2);

                    // UnsignedDirection - UnsignedDirection -> vect1
                    EXPECT_NEAR(0, abs(modulo(val1 - val2 + static_cast<TypeParam>(180), static_cast<TypeParam>(360)) - static_cast<TypeParam>(180)
                        - (p1 - p2).value()), AZIMUTH_TOLERANCE);

                    // UnsignedDirection + vect1 -> UnsignedDirection
                    EXPECT_NEAR(0, abs(modulo(val1 + val2, static_cast<TypeParam>(360))
                        - (p1 + v2).value()), AZIMUTH_TOLERANCE);

                    // UnsignedDirection - vect1 -> UnsignedDirection
                    EXPECT_NEAR(0, abs(modulo(val1 - val2, static_cast<TypeParam>(360))
                        - (p1 - v2).value()), AZIMUTH_TOLERANCE);
                }
            }
        }
    }

    TEST(unitTest, magnitudes)
    {
        ASSERT_STREQ(" rad", radians::id().suffix());
        EXPECT_EQ(1, radians::id().ratio());
        EXPECT_EQ(0, radians::id().offset());

        ASSERT_STREQ(" rev", turns::id().suffix());
        EXPECT_NEAR(2 * pi, turns::id().ratio(), AZIMUTH_TOLERANCE);
        EXPECT_EQ(0, turns::id().offset());

        ASSERT_STREQ(" m", metres::id().suffix());
        EXPECT_EQ(1, metres::id().ratio());
        EXPECT_EQ(0, metres::id().offset());

        ASSERT_STREQ("^", degrees::id().suffix());
        EXPECT_NEAR(pi / 180, degrees::id().ratio(), AZIMUTH_TOLERANCE);
        EXPECT_EQ(0, degrees::id().offset());

        ASSERT_STREQ(" Km", km::id().suffix());
        EXPECT_EQ(1000, km::id().ratio());
        EXPECT_EQ(0, km::id().offset());

        ASSERT_STREQ("\"", inches::id().suffix());
        EXPECT_EQ(0.0254, inches::id().ratio());
        EXPECT_EQ(0, inches::id().offset());

        ASSERT_STREQ(" s", seconds::id().suffix());
        EXPECT_EQ(1, metres::id().ratio());
        EXPECT_EQ(0, metres::id().offset());

        ASSERT_STREQ(" h", hours::id().suffix());
        EXPECT_EQ(3600, hours::id().ratio());
        EXPECT_EQ(0, hours::id().offset());

        ASSERT_STREQ(" d", days::id().suffix());
        EXPECT_EQ(86400, days::id().ratio());
        EXPECT_EQ(0, days::id().offset());

        ASSERT_STREQ(" m/s", metres_per_second::id().suffix());
        EXPECT_EQ(1, metres_per_second::id().ratio());
        EXPECT_EQ(0, metres_per_second::id().offset());

        ASSERT_STREQ(" Km/h", km_per_hour::id().suffix());
        EXPECT_FLOAT_EQ(1 / 3.6, km_per_hour::id().ratio());
        EXPECT_EQ(0, km_per_hour::id().offset());

        ASSERT_STREQ("\"/day", inches_per_day::id().suffix());
        EXPECT_EQ(86400 / 0.0254, inches_per_day::id().ratio());
        EXPECT_EQ(0, inches_per_day::id().offset());

        ASSERT_STREQ(" m2", square_metres::id().suffix());
        EXPECT_EQ(1, square_metres::id().ratio());
        EXPECT_EQ(0, square_metres::id().offset());

        ASSERT_STREQ(" Km2", square_km::id().suffix());
        EXPECT_EQ(1000000, square_km::id().ratio());
        EXPECT_EQ(0, square_km::id().offset());

        ASSERT_STREQ("\"2", square_inches::id().suffix());
        EXPECT_EQ(0.0254 * 0.0254, square_inches::id().ratio());
        EXPECT_EQ(0, square_inches::id().offset());

        ASSERT_STREQ("^K", kelvin::id().suffix());
        EXPECT_EQ(1, kelvin::id().ratio());
        EXPECT_EQ(0, kelvin::id().offset());

        ASSERT_STREQ("^C", celsius::id().suffix());
        EXPECT_EQ(1, celsius::id().ratio());
        EXPECT_EQ(273.15, celsius::id().offset());

        ASSERT_STREQ("^F", fahrenheit::id().suffix());
        EXPECT_EQ(5. / 9., fahrenheit::id().ratio());
        EXPECT_EQ(273.15 - 32. * 5. / 9., fahrenheit::id().offset());
    }

    TEST(unitTest, units)
    {
        ASSERT_STREQ(" rad", radians::suffix());
        EXPECT_EQ(1, radians::ratio());
        EXPECT_EQ(0, radians::offset());

        ASSERT_STREQ(" rev", turns::suffix());
        EXPECT_NEAR(2 * pi, turns::ratio(), AZIMUTH_TOLERANCE);
        EXPECT_EQ(0, turns::offset());

        ASSERT_STREQ(" m", metres::suffix());
        EXPECT_EQ(1, metres::ratio());
        EXPECT_EQ(0, metres::offset());

        ASSERT_STREQ("^", degrees::suffix());
        EXPECT_NEAR(pi / 180, degrees::ratio(), AZIMUTH_TOLERANCE);
        EXPECT_EQ(0, degrees::offset());

        ASSERT_STREQ(" Km", km::suffix());
        EXPECT_EQ(1000, km::ratio());
        EXPECT_EQ(0, km::offset());

        ASSERT_STREQ("\"", inches::suffix());
        EXPECT_EQ(0.0254, inches::ratio());
        EXPECT_EQ(0, inches::offset());

        ASSERT_STREQ(" s", seconds::suffix());
        EXPECT_EQ(1, metres::ratio());
        EXPECT_EQ(0, metres::offset());

        ASSERT_STREQ(" h", hours::suffix());
        EXPECT_EQ(3600, hours::ratio());
        EXPECT_EQ(0, hours::offset());

        ASSERT_STREQ(" d", days::suffix());
        EXPECT_EQ(86400, days::ratio());
        EXPECT_EQ(0, days::offset());

        ASSERT_STREQ(" m/s", metres_per_second::suffix());
        EXPECT_EQ(1, metres_per_second::ratio());
        EXPECT_EQ(0, metres_per_second::offset());

        ASSERT_STREQ(" Km/h", km_per_hour::suffix());
        EXPECT_FLOAT_EQ(1 / 3.6, km_per_hour::ratio());
        EXPECT_EQ(0, km_per_hour::offset());

        ASSERT_STREQ("\"/day", inches_per_day::suffix());
        EXPECT_EQ(86400 / 0.0254, inches_per_day::ratio());
        EXPECT_EQ(0, inches_per_day::offset());

        ASSERT_STREQ(" m2", square_metres::suffix());
        EXPECT_EQ(1, square_metres::ratio());
        EXPECT_EQ(0, square_metres::offset());

        ASSERT_STREQ(" Km2", square_km::suffix());
        EXPECT_EQ(1000000, square_km::ratio());
        EXPECT_EQ(0, square_km::offset());

        ASSERT_STREQ("\"2", square_inches::suffix());
        EXPECT_EQ(0.0254 * 0.0254, square_inches::ratio());
        EXPECT_EQ(0, square_inches::offset());

        ASSERT_STREQ("^K", kelvin::suffix());
        EXPECT_EQ(1, kelvin::ratio());
        EXPECT_EQ(0, kelvin::offset());

        ASSERT_STREQ("^C", celsius::suffix());
        EXPECT_EQ(1, celsius::ratio());
        EXPECT_EQ(273.15, celsius::offset());

        ASSERT_STREQ("^F", fahrenheit::suffix());
        EXPECT_EQ(5. / 9., fahrenheit::ratio());
        EXPECT_EQ(273.15 - 32. * 5. / 9., fahrenheit::offset());
    }

    TEST(unitTest, vect_conversions)
    {
        EXPECT_FLOAT_EQ(101. / 2 / pi, convert<turns>(Measure<radians>(101)).value());
        EXPECT_FLOAT_EQ(101. * 2 * pi, convert<radians>(Measure<turns>(101)).value());
        EXPECT_FLOAT_EQ(101. / 2 / pi * 360, convert<degrees>(Measure<radians>(101)).value());
        EXPECT_FLOAT_EQ(101. * 2 * pi / 360, convert<radians>(Measure<degrees>(101)).value());
        EXPECT_FLOAT_EQ(101. / 360, convert<turns>(Measure<degrees>(101)).value());
        EXPECT_FLOAT_EQ(101. * 360, convert<degrees>(Measure<turns>(101)).value());

        EXPECT_FLOAT_EQ(101. * 1000 / 0.0254, convert<inches>(Measure<km>(101)).value());
        EXPECT_FLOAT_EQ(101. / 1000 * 0.0254, convert<km>(Measure<inches>(101)).value());

        EXPECT_FLOAT_EQ(101., convert<celsius>(Measure<kelvin>(101)).value());
        EXPECT_FLOAT_EQ(101., convert<kelvin>(Measure<celsius>(101)).value());
        EXPECT_FLOAT_EQ(101. * 9 / 5, convert<fahrenheit>(Measure<kelvin>(101)).value());
        EXPECT_FLOAT_EQ(101. * 5 / 9, convert<kelvin>(Measure<fahrenheit>(101)).value());
        EXPECT_FLOAT_EQ(101. / 9 * 5, convert<celsius>(Measure<fahrenheit>(101)).value());
        EXPECT_FLOAT_EQ(101. / 5 * 9, convert<fahrenheit>(Measure<celsius>(101)).value());

        auto a1 = convert<inches>(Measure2d<metres>(101, 102));
        EXPECT_FLOAT_EQ(101. / 0.0254, a1.x().value());
        EXPECT_FLOAT_EQ(102. / 0.0254, a1.y().value());

        auto a2 = convert<metres>(Measure2d<inches>(101, 102));
        EXPECT_FLOAT_EQ(101. * 0.0254, a2.x().value());
        EXPECT_FLOAT_EQ(102. * 0.0254, a2.y().value());

        auto a3 = convert<inches>(Measure3d<metres>(101, 102, 103));
        EXPECT_FLOAT_EQ(101. / 0.0254, a3.x().value());
        EXPECT_FLOAT_EQ(102. / 0.0254, a3.y().value());
        EXPECT_FLOAT_EQ(103. / 0.0254, a3.z().value());

        auto a4 = convert<metres>(Measure3d<inches>(101, 102, 103));
        EXPECT_FLOAT_EQ(101. * 0.0254, a4.x().value());
        EXPECT_FLOAT_EQ(102. * 0.0254, a4.y().value());
        EXPECT_FLOAT_EQ(103. * 0.0254, a4.z().value());
    }

    TEST(unitTest, point_conversions)
    {
        EXPECT_FLOAT_EQ(101. / 2 / pi, convert<turns>(MeasurePoint<radians>(101)).value());
        EXPECT_FLOAT_EQ(101. * 2 * pi, convert<radians>(MeasurePoint<turns>(101)).value());
        EXPECT_FLOAT_EQ(101. / 2 / pi * 360, convert<degrees>(MeasurePoint<radians>(101)).value());
        EXPECT_FLOAT_EQ(101. * 2 * pi / 360, convert<radians>(MeasurePoint<degrees>(101)).value());
        EXPECT_FLOAT_EQ(101. / 360, convert<turns>(MeasurePoint<degrees>(101)).value());
        EXPECT_FLOAT_EQ(101. * 360, convert<degrees>(MeasurePoint<turns>(101)).value());

        EXPECT_FLOAT_EQ(101. * 1000 / 0.0254, convert<inches>(MeasurePoint<km>(101)).value());
        EXPECT_FLOAT_EQ(101. / 1000 * 0.0254, convert<km>(MeasurePoint<inches>(101)).value());

        EXPECT_FLOAT_EQ(101. - 273.15, convert<celsius>(MeasurePoint<kelvin>(101)).value());
        EXPECT_FLOAT_EQ(101. + 273.15, convert<kelvin>(MeasurePoint<celsius>(101)).value());
        EXPECT_FLOAT_EQ((101. - 273.15) * 9 / 5 + 32, convert<fahrenheit>(MeasurePoint<kelvin>(101)).value());
        EXPECT_FLOAT_EQ((101. - 32) * 5 / 9 + 273.15, convert<kelvin>(MeasurePoint<fahrenheit>(101)).value());
        EXPECT_FLOAT_EQ((101. - 32) / 9 * 5, convert<celsius>(MeasurePoint<fahrenheit>(101)).value());
        EXPECT_FLOAT_EQ(101. / 5 * 9 + 32, convert<fahrenheit>(MeasurePoint<celsius>(101)).value());

        auto a1 = convert<inches>(MeasurePoint2d<metres>(101, 102));
        EXPECT_FLOAT_EQ(101. / 0.0254, a1.x().value());
        EXPECT_FLOAT_EQ(102. / 0.0254, a1.y().value());

        auto a2 = convert<metres>(MeasurePoint2d<inches>(101, 102));
        EXPECT_FLOAT_EQ(101. * 0.0254, a2.x().value());
        EXPECT_FLOAT_EQ(102. * 0.0254, a2.y().value());

        auto a3 = convert<inches>(MeasurePoint3d<metres>(101, 102, 103));
        EXPECT_FLOAT_EQ(101. / 0.0254, a3.x().value());
        EXPECT_FLOAT_EQ(102. / 0.0254, a3.y().value());
        EXPECT_FLOAT_EQ(103. / 0.0254, a3.z().value());

        auto a4 = convert<metres>(MeasurePoint3d<inches>(101, 102, 103));
        EXPECT_FLOAT_EQ(101. * 0.0254, a4.x().value());
        EXPECT_FLOAT_EQ(102. * 0.0254, a4.y().value());
        EXPECT_FLOAT_EQ(103. * 0.0254, a4.z().value());
    }

    TEST(unitTest, direction_conversions)
    {
        EXPECT_FLOAT_EQ(fmod(1010.3 / 2 / pi + 0.5, 1.) - 0.5, convert<turns>(SignedDirection<radians>(1010.3)).value());
        EXPECT_FLOAT_EQ(fmod(1010.3 * 2 * pi + pi, 2. * pi) - pi, convert<radians>(SignedDirection<turns>(1010.3)).value());
        EXPECT_FLOAT_EQ(fmod(1010.3 / 2 / pi * 360 + 180, 360.) - 180, convert<degrees>(SignedDirection<radians>(1010.3)).value());
        EXPECT_FLOAT_EQ(fmod(1010.3 * 2 * pi / 360 + pi, 2. * pi) - pi, convert<radians>(SignedDirection<degrees>(1010.3)).value());
        EXPECT_FLOAT_EQ(fmod(1010.3 / 360 + 0.5, 1.) - 0.5, convert<turns>(SignedDirection<degrees>(1010.3)).value());
        EXPECT_FLOAT_EQ(fmod(1010.3 * 360 + 180, 360.) - 180, convert<degrees>(SignedDirection<turns>(1010.3)).value());

        EXPECT_FLOAT_EQ(fmod(1010.3 / 2 / pi, 1.), convert<turns>(UnsignedDirection<radians>(1010.3)).value());
        EXPECT_FLOAT_EQ(fmod(1010.3 * 2 * pi, 2. * pi), convert<radians>(UnsignedDirection<turns>(1010.3)).value());
        EXPECT_FLOAT_EQ(fmod(1010.3 / 2 / pi * 360, 360.), convert<degrees>(UnsignedDirection<radians>(1010.3)).value());
        EXPECT_FLOAT_EQ(fmod(1010.3 * 2 * pi / 360, 2. * pi), convert<radians>(UnsignedDirection<degrees>(1010.3)).value());
        EXPECT_FLOAT_EQ(fmod(1010.3 / 360, 1.), convert<turns>(UnsignedDirection<degrees>(1010.3)).value());
        EXPECT_FLOAT_EQ(fmod(1010.3 * 360, 360.), convert<degrees>(UnsignedDirection<turns>(1010.3)).value());
    }

    TEST(unitTest, derived_operations)
    {
        EXPECT_FLOAT_EQ(12.3 * 23.47, (Measure<hours>(12.3) * Measure<km_per_hour>(23.47)).value());
        EXPECT_FLOAT_EQ(12.3 * 23.47, (Measure<km_per_hour>(12.3) * Measure<hours>(23.47)).value());
        EXPECT_FLOAT_EQ(12.3 / 23.47, (Measure<km>(12.3) / Measure<hours>(23.47)).value());
        EXPECT_FLOAT_EQ(12.3 / 23.47, (Measure<km>(12.3) / Measure<km_per_hour>(23.47)).value());
        EXPECT_FLOAT_EQ(12.3 * 23.47, (Measure<inches>(12.3) * Measure<inches>(23.47)).value());
        EXPECT_FLOAT_EQ(12.3 / 23.47, (Measure<square_inches>(12.3) / Measure<inches>(23.47)).value());
        EXPECT_FLOAT_EQ(std::sqrt(23.47), sqrt(Measure<square_inches>(23.47)).value());

        auto a1 = Measure<hours>(12.3) * Measure2d<km_per_hour>(23.47, 34.51);
        EXPECT_FLOAT_EQ(12.3 * 23.47, a1.x().value());
        EXPECT_FLOAT_EQ(12.3 * 34.51, a1.y().value());

        auto a2 = Measure2d<km_per_hour>(23.47, 34.51) * Measure<hours>(12.3);
        EXPECT_FLOAT_EQ(23.47 * 12.3, a2.x().value());
        EXPECT_FLOAT_EQ(34.51 * 12.3, a2.y().value());

        auto a3 = Measure2d<km>(23.47, 34.51) / Measure<hours>(12.3);
        EXPECT_FLOAT_EQ(23.47 / 12.3, a3.x().value());
        EXPECT_FLOAT_EQ(34.51 / 12.3, a3.y().value());

        auto a4 = Measure<hours>(12.3) * Measure3d<km_per_hour>(23.47, 34.51, 45.19);
        EXPECT_FLOAT_EQ(12.3 * 23.47, a4.x().value());
        EXPECT_FLOAT_EQ(12.3 * 34.51, a4.y().value());
        EXPECT_FLOAT_EQ(12.3 * 45.19, a4.z().value());

        auto a5 = Measure3d<km_per_hour>(23.47, 34.51, 45.19) * Measure<hours>(12.3);
        EXPECT_FLOAT_EQ(23.47 * 12.3, a5.x().value());
        EXPECT_FLOAT_EQ(34.51 * 12.3, a5.y().value());
        EXPECT_FLOAT_EQ(45.19 * 12.3, a5.z().value());

        auto a6 = Measure3d<km>(23.47, 34.51, 45.19) / Measure<hours>(12.3);
        EXPECT_FLOAT_EQ(23.47 / 12.3, a6.x().value());
        EXPECT_FLOAT_EQ(34.51 / 12.3, a6.y().value());
        EXPECT_FLOAT_EQ(45.19 / 12.3, a6.z().value());

        EXPECT_FLOAT_EQ(12.3 * 34.5 + 23.4 * 45.6,
            (Measure2d<newtons>(12.3, 23.4) * Measure2d<metres>(34.5, 45.6)).value());
        EXPECT_FLOAT_EQ(12.3 * 34.5 + 23.4 * 45.6,
            (Measure2d<metres>(12.3, 23.4) * Measure2d<newtons>(34.5, 45.6)).value());

        EXPECT_FLOAT_EQ(12.3 * 45.6 - 23.4 * 34.5,
            cross_product(Measure2d<metres_per_second>(12.3, 23.4), Measure2d<tesla>(34.5, 45.6)).value());
        EXPECT_FLOAT_EQ(12.3 * 45.6 - 23.4 * 34.5,
            cross_product(Measure2d<tesla>(12.3, 23.4), Measure2d<metres_per_second>(34.5, 45.6)).value());

        EXPECT_FLOAT_EQ(12.3 * 45.6 + 23.4 * 56.7 + 34.5 * 67.8,
            (Measure3d<newtons>(12.3, 23.4, 34.5) * Measure3d<metres>(45.6, 56.7, 67.8)).value());
        EXPECT_FLOAT_EQ(12.3 * 45.6 + 23.4 * 56.7 + 34.5 * 67.8,
            (Measure3d<metres>(12.3, 23.4, 34.5) * Measure3d<newtons>(45.6, 56.7, 67.8)).value());

        auto a7 = cross_product(
            Measure3d<metres_per_second>(12.3, 23.4, 34.5),
            Measure3d<tesla>(45.6, 56.7, 67.8));
        EXPECT_FLOAT_EQ(23.4 * 67.8 - 34.5 * 56.7, a7.x().value());
        EXPECT_FLOAT_EQ(34.5 * 45.6 - 12.3 * 67.8, a7.y().value());
        EXPECT_FLOAT_EQ(12.3 * 56.7 - 23.4 * 45.6, a7.z().value());

        auto a8 = cross_product(
            Measure3d<tesla>(12.3, 23.4, 34.5),
            Measure3d<metres_per_second>(45.6, 56.7, 67.8));
        EXPECT_FLOAT_EQ(23.4 * 67.8 - 34.5 * 56.7, a8.x().value());
        EXPECT_FLOAT_EQ(34.5 * 45.6 - 12.3 * 67.8, a8.y().value());
        EXPECT_FLOAT_EQ(12.3 * 56.7 - 23.4 * 45.6, a8.z().value());

        EXPECT_FLOAT_EQ(12.3 * 34.5 + 23.4 * 45.6,
            (Measure2d<units>(12.3, 23.4) * Measure2d<units>(34.5, 45.6)).value());

        EXPECT_FLOAT_EQ(12.3 * 45.6 - 23.4 * 34.5,
            (cross_product(Measure2d<units>(12.3, 23.4), Measure2d<units>(34.5, 45.6))).value());

        EXPECT_FLOAT_EQ(12.3 * 45.6 + 23.4 * 56.7 + 34.5 * 67.8,
            (Measure3d<units>(12.3, 23.4, 34.5) * Measure3d<units>(45.6, 56.7, 67.8)).value());

        auto a9 = cross_product(
            Measure3d<units>(12.3, 23.4, 34.5),
            Measure3d<units>(45.6, 56.7, 67.8));
        EXPECT_FLOAT_EQ(23.4 * 67.8 - 34.5 * 56.7, a9.x().value());
        EXPECT_FLOAT_EQ(34.5 * 45.6 - 12.3 * 67.8, a9.y().value());
        EXPECT_FLOAT_EQ(12.3 * 56.7 - 23.4 * 45.6, a9.z().value());
    }

    /*
    operations to test:
        trigonometriche
        direction
    */

    /*
    //// STATIC-UNIT 1-DIMENSION MEASURES

    //EXPECT_EQ(m5.value(), m4.value());

    //// STATIC-UNIT 2-DIMENSIONS MEASURES
    //// STATIC-UNIT 3-DIMENSIONS MEASURES
    //// STATIC-UNIT AZIMUTHS

    */
    */
}
