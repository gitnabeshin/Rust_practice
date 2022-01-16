fn main() {
    // Ownership
    let s1 = String::from("hello S1");
    println!("S1: {}", s1);
    let (s2, len) = calculate_length(s1);
    println!("The length of S2: '{}' is {}.", s2, len);

    //Reference ans Borrowing
    let s3 = String::from("hello");
    let len = calculate_length_ref(&s3);
    println!("The length of '{}' is {}.", s3, len);
    //change reference is not allowd
    change_ref(&s3);

    //mutable reference
    let mut s4 = String::from("hello");
    change_ref2(&mut s4);

    run_mut_ref_error();
    run_mut_ref_valid();
    run_dangle_pattern();
}

//return tuple
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
} //return ownership

// Reference param type "&String"
fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

// ref is not mutable
fn change_ref(s: &String) {
    println!("s: {}", s);
    // s.push_str(", world.");
    // s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
}

// mutable ref
fn change_ref2(s: &mut String) {
    println!("s: {}", s);
    s.push_str(", world.");
    // s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
}

//Rust prevents multiple pointer access to the same variable.
fn run_mut_ref_error() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; //second mutable borrow is not allowed
    // println!("{}, {}", r1, r2);
    println!("{}", r1);
}

fn run_mut_ref_valid() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    println!("{}, {}, {}", r1, r2, r3);
}

fn run_dangle_pattern(){
    // let reference_to_nothing = dangle();
    let _reference_to_nothing = no_dangle();
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// } //s is freed here, so return dangling reference

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}