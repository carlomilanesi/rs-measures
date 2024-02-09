#[macro_export]
macro_rules! define_1d_2d_3d {
    {} => {
        rs_measures::define_1d_2d!();
        rs_measures::inner_define_measure_3d!();
        rs_measures::inner_define_measure_point_3d!();
        rs_measures::inner_define_linear_map_3d!();
        rs_measures::inner_define_affine_map_3d!();
    };
}
