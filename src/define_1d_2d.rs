#[macro_export]
macro_rules! define_1d_2d {
    {} => {
        rs_measures::define_1d_and_directions!();
        rs_measures::inner_define_measure_2d!();
        rs_measures::inner_define_measure_point_2d!();
        rs_measures::inner_define_linear_map_2d!();
        rs_measures::inner_define_affine_map_2d!();
    };
}
