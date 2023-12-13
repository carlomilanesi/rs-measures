use trybuild::TestCases;

fn pass(tester: &TestCases, case: &str) {
    tester.pass(format!("tests/define-units-relation/{}.rs", case));
}

fn compile_fail(tester: &TestCases, case: &str) {
    tester.compile_fail(format!("tests/define-units-relation/{}.rs", case));
}

#[test]
fn tests() {
    let t = TestCases::new();
    pass(&t, "1_mul_1_is_1"); // id1:1 * id2:1 == id3:1  =>  expand_1_1(id1, id2, id3)
    pass(&t, "1_mul_itself_is_1"); // id1:1 * id1:1 == id3:1  =>  expand_1_1(id1, id1, id3)
    pass(&t, "1_mul_2_is_2"); // id1:1 * id2:2 == id3:2  =>  expand_1_2(id1, id2, id3)
    pass(&t, "2_mul_1_is_2"); // id1:2 * id2:1 == id3:2  =>  expand_1_2(id2, id1, id3)
    pass(&t, "1_mul_3_is_3"); // id1:1 * id2:3 == id3:3  =>  expand_1_3(id1, id2, id3)
    pass(&t, "3_mul_1_is_3"); // id1:3 * id2:1 == id3:3  =>  expand_1_3(id2, id1, id3)
    pass(&t, "1_mul_1_is_0"); // id1:1 * id2:1 == 1  =>  expand_inverse(id1, id2)
    pass(&t, "2_mul_2_is_1"); // id1:2 * id2:2 == id3:1  =>  expand_2_2(id1, id2, id3)
    pass(&t, "2_mul_itself_is_1"); // id1:2 * id1:2 == id3:1  =>  expand_2_2(id1, id1, id3)
    pass(&t, "3_mul_3_is_1"); // id1:3 * id2:3 == id3:1  =>  expand_3_3(id1, id2, id3)
    pass(&t, "3_mul_itself_is_1"); // id1:3 * id1:3 == id3:1  =>  expand_3_3(id1, id1, id3)
    pass(&t, "1_div_1_is_1"); // id1:1 / id2:1 == id3:1  =>  expand_1_1(id2, id3, id1)
    pass(&t, "2_div_1_is_2"); // id1:2 / id2:1 == id3:2  =>  expand_1_2(id2, id3, id1)
    pass(&t, "3_div_1_is_3"); // id1:3 / id2:1 == id3:3  =>  expand_1_3(id2, id3, id1)
    pass(&t, "0_div_1_is_1"); // 1 / id2:1 == id3:1  =>  expand_inverse(id2, id3)
    pass(&t, "1_is_1_mul_1"); // id1:1 == id2:1 * id3:1  =>  expand_1_1(id2, id3, id1)
    pass(&t, "2_is_1_mul_2"); // id1:2 == id2:1 * id3:2  =>  expand_1_2(id2, id3, id1)
    pass(&t, "2_is_2_mul_1"); // id1:2 == id2:2 * id3:1  =>  expand_1_2(id3, id2, id1)
    pass(&t, "3_is_1_mul_3"); // id1:3 == id2:1 * id3:3  =>  expand_1_3(id2, id3, id1)
    pass(&t, "3_is_3_mul_1"); // id1:3 == id2:3 * id3:1  =>  expand_1_3(id3, id2, id1)
    pass(&t, "0_is_1_mul_1"); // 1 == id2:1 * id3:1  =>  expand_inverse(id2, id3)
    pass(&t, "1_is_2_mul_2"); // id1:1 == id2:2 * id3:2  =>  expand_2_2(id2, id3, id1)
    pass(&t, "1_is_2_mul_itself"); // id1:1 == id2:2 * id2:2  =>  expand_2_2(id2, id2, id1)
    pass(&t, "1_is_3_mul_3"); // id1:1 == id2:3 * id3:3  =>  expand_3_3(id2, id3, id1)
    pass(&t, "1_is_3_mul_itself"); // id1:1 == id2:3 * id2:3  =>  expand_3_3(id2, id2, id1)
    pass(&t, "1_is_1_div_1"); // id1:1 == id2:1 / id3:1  =>  expand_1_1(id3, id1, id2)
    pass(&t, "2_is_2_div_1"); // id1:2 == id2:2 / id3:1  =>  expand_1_2(id3, id1, id2)
    pass(&t, "3_is_3_div_1"); // id1:3 == id2:3 / id3:1  =>  expand_1_3(id3, id1, id2)
    pass(&t, "1_is_0_div_1"); // id1:1 == 1 / id3:1      =>  expand_inverse(id1, id3)
    pass(&t, "1_is_2_cross_2"); // id1:1 == id2:2 X id3:2  =>  expand_cross_2(id2, id3, id1)
    pass(&t, "1_is_2_cross_itself"); // id1:1 == id2:2 X id2:2  =>  expand_cross_2(id2, id2, id1)
    pass(&t, "3_is_3_cross_3"); // id1:3 == id2:3 X id3:3  =>  expand_cross_3(id2, id3, id1)
    pass(&t, "3_is_3_cross_itself"); // id1:3 == id2:3 X id2:3  =>  expand_cross_3(id2, id2, id1)
    pass(&t, "2_cross_2_is_1"); // id1:2 X id2:2 == id3:1  =>  expand_cross_2(id1, id2, id3)
    pass(&t, "2_cross_itself_is_1"); // id1:2 X id1:2 == id3:1  =>  expand_cross_2(id1, id1, id3)
    pass(&t, "3_cross_3_is_3"); // id1:3 X id2:3 == id3:3  =>  expand_cross_3(id1, id2, id3)
    pass(&t, "3_cross_itself_is_3"); // id1:3 X id1:3 == id3:3  =>  expand_cross_3(id1, id1, id3)
    compile_fail(&t, "1_mul_1_is_not_1");
}
