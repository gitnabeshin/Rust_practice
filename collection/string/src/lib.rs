
pub fn string_edit() {
    let mut _s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    println!("s: {}", s);

    let s = "initial contents2".to_string();
    println!("s: {}", s);

    let hello = String::from("Hello");
    println!("hello: {}", hello);
    let hello = String::from("こんにちは");
    println!("hello: {}", hello);

    let mut s = String::from("foo");
    s.push_str(" bar");
    let s2 = " baz ";
    s.push_str(s2);
    s.push('w');

    println!("s: {}", s);
    println!("s2: {}", s2);
}

pub fn string_connect() {
    let s1 = String::from("Hello ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; //s1 is moved
    // [+] --> fn add(self, s: &str) -> String {

    // this will be error(s1 is moved)
    // println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!("s3: {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s: {}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s: {}", s);
}

pub fn internal_data(){
    let len = String::from("Hello").len();
    println!("len: {}", len);

    let len2 = String::from("こんにちは").len();
    println!("len2: {}", len2);

    let hello = "hello";
    // this is not allowed
    // let ans = &hello[0];
    println!("str: {}", hello);
    println!("str: {:?}", hello.chars());
    println!("[0]th char {:?}", hello.chars().nth(0));
    println!("str: {:?}", hello.bytes());
    println!("[1]th char {:?}", hello.bytes().nth(1));

    let hello = "こんにちは";
    // let ans = &hello[0];
    println!("str: {}", hello);
    println!("str: {:?}", hello.chars());
    println!("[0]th char {:?}", hello.chars().nth(0));
    println!("str: {:?}", hello.bytes());
    println!("[1]th char {:?}", hello.bytes().nth(1));

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        string_edit();
    }

    #[test]
    fn it_works2() {
        string_connect();
    }

    #[test]
    fn it_works3() {
        internal_data();
    }
}
