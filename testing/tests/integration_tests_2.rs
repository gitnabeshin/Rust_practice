//extern crate testing;
use testing::*;

mod common;

fn adds_three(result_of_two:i32) -> i32 {
    testing::add_two(2) + 1
}

#[test]
fn it_adds_two_plus1() {
    common::setup();
    assert_eq!(5, adds_three(2));
}