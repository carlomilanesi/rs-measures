use trybuild::TestCases;

fn pass(tester: &TestCases, case: &str) {
    tester.pass(format!("tests/define-units-relationship/{}.rs", case));
}

fn compile_fail(tester: &TestCases, case: &str) {
    tester.compile_fail(format!("tests/define-units-relationship/{}.rs", case));
}

#[test]
fn tests() {
    let t = TestCases::new();
    pass(&t, "1_is_1_mul_1"); // id1:1 == id2:1 * id3:1  =>  expand_1_1(id2, id3, id1)
    pass(&t, "1_is_1_mul_itself"); // id1:1 == id2:1 * =:1  =>  expand_1_1(id2, id1)
    pass(&t, "2_is_1_mul_2"); // id1:2 == id2:1 * id3:2  =>  expand_1_2(id2, id3, id1)
    pass(&t, "2_is_2_mul_1"); // id1:2 == id2:2 * id3:1  =>  expand_1_2(id3, id2, id1)
    pass(&t, "3_is_1_mul_3"); // id1:3 == id2:1 * id3:3  =>  expand_1_3(id2, id3, id1)
    pass(&t, "3_is_3_mul_1"); // id1:3 == id2:3 * id3:1  =>  expand_1_3(id3, id2, id1)
    pass(&t, "1_is_2_mul_2"); // id1:1 == id2:2 * id3:2  =>  expand_2_2(id2, id3, id1)
    pass(&t, "1_is_2_mul_itself"); // id1:1 == id2:2 * =:2  =>  expand_2_2(id2, id1)
    pass(&t, "1_is_3_mul_3"); // id1:1 == id2:3 * id3:3  =>  expand_3_3(id2, id3, id1)
    pass(&t, "1_is_3_mul_itself"); // id1:1 == id2:3 * =:3  =>  expand_3_3(id2, id1)
    pass(&t, "1_is_0_div_1"); // id1:1 == 1 / id3:1      =>  expand_inverse(id1, id3)
    pass(&t, "1_is_2_cross_2"); // id1:1 == id2:2 X id3:2  =>  expand_cross_2(id2, id3, id1)
    pass(&t, "1_is_2_cross_itself"); // id1:1 == id2:2 X =:2  =>  expand_cross_2(id2, id1)
    pass(&t, "3_is_3_cross_3"); // id1:3 == id2:3 X id3:3  =>  expand_cross_3(id2, id3, id1)
    pass(&t, "3_is_3_cross_itself"); // id1:3 == id2:3 X =:3  =>  expand_cross_3(id2, id1)
    compile_fail(&t, "fail_1_is_1_mul_2"); // id1:1 == id2:1 * id3:2
    compile_fail(&t, "fail_1_is_bad_literal_div_1"); // id1:1 == 2 / id2:1
    compile_fail(&t, "fail_1_mul_1_mul_1"); // id1:1 * id2:1 * id3:1
    compile_fail(&t, "fail_4_is_1_mul_1"); // id1:4 == id2:1 * id3:1
}
