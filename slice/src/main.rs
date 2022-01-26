fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // println!("{}: {}", i, &item);
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

//useful return type
fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // println!("{}: {}", i, &item);
        if item == b' ' {
            return &s[..i];
        }
    }

    &s
}

//more common input param style
fn first_word3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // println!("{}: {}", i, &item);
        if item == b' ' {
            return &s[..i];
        }
    }

    &s
}

fn other_string_code() {
    let mut s = String::from("Hello world");

    // string slice
    // index means in UTF-8
    let hello = &s[0..5]; //0th pointer, 5 length(5-0)
    let world = &s[6..11]; //6th pointer, 5 length(11-6)
    println!("2.{}", hello);
    println!("3.{}", world);
    let hello2 = &s[..5]; //0th pointer, 5 length(5-0)
    let world2 = &s[6..]; //0th pointer, 5 length(5-0)
    println!("4.{}", hello2);
    println!("5.{}", world2);

    s = String::from("0123456789");
    let index = first_word(&s);
    println!("6.{}: first space index={}", s, index);

    // This will be compile error
    // println!("{}", hello);
    // println!("{}", world);
}

fn other_slice() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8];
    let slice = &a[..3];
    println!("8.{:?}", a);
    println!("8.{:?}", slice);
    println!("8.{:?}", &a[3..]);
}

fn call_better_func() {
    let s = String::from("Hello world");
    let word = first_word3(&s); //imutable borrow
    println!("7-1.{}: first word=[{}]", s, word);

    let my_string_literal = "hello world";
    let word = first_word3(&my_string_literal[0..6]);
    println!("7-2.{}: first word=[{}]", s, word);

    let my_string_literal = "hello world";
    let word = first_word3(my_string_literal);
    println!("7-3.{}: first word=[{}]", s, word);
}

fn main() {
    //s, word has same memory area.
    let s = String::from("Hello world");
    let index = first_word(&s);
    let word = first_word2(&s); //imutable borrow
    println!("1-1.{}: first space index=[{}]", s, index);

    //Rust don't allow mutable borrow during imutable borrow is active
    // s.clear(); //this will be compile error
    println!("1-2.{}: first word=[{}]", s, word); //imutable borrow

    other_string_code();
    call_better_func();
    other_slice();
}
