use tests_dir_sample::add;

#[test]
fn test_add() {
    assert_eq!(0, add(0, 0));
    assert_eq!(1, add(1, 0));
    assert_eq!(1, add(0, 1));
}

#[test]
fn test_add2() {
    assert_eq!(2, add(1, 1));
}

#[test]
#[ignore]
fn test_add3() {
    assert_eq!(4, add(2, 2));
}

#[test]
#[should_panic]
fn test_paniced() {
    assert_eq!(4, add(2, 2));
    panic!("Expected Panic!!");
}
