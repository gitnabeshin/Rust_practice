/******************************************
* 
*
*******************************************/
fn main() {
    let s = String::from("hello");
    takes_ownership(s); // s is moved into func
//    println!("{}", s); //ERROR: s is feed. 

    let x = 5;
    makes_copy(x);
    println!("{}", x); //NO ERROR: i32 is copied.

    let s1 = gives_ownership();
    println!("S1: {}", s1);

    let s2 = String::from("hello S2");
    println!("S2: {}", s2);
    let s3 = takes_and_gives_back(s2); 
    println!("S3: {}", s3);

    let s4 = String::from("hello S4");
    println!("S4: {}", s4);
    let (s5, len) = calculate_length(s4);
    println!("The length of S5: '{}' is {}.", s5, len)
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} //string is freed here

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} //i32 has copy trait

fn gives_ownership() -> String {
    let some_string = String::from("hello S1");
    some_string //move back
} //return ownership

fn takes_and_gives_back(a_string: String) -> String {
    a_string //move back
} //return ownership

//return tuple
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
} //return ownership