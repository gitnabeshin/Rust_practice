fn adapter() {
    println!("adapter -----------");
    let source = vec![1, 2, 3, 4, 5, 6, 7, 8];
    for element in &source {
        println!("source={}", element);
    }

    println!("++++++ result %2");
    let result = source
        .into_iter()
        .filter(|x| x % 2 == 0)
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    for element in &result {
        println!("result for:{}", element);
    }
    for s in result.into_iter() {
        println!("result ite:{}", s);
    }
}

fn pattern_match(opt_sample: Option<i32>) {
    println!("pattern matching -----------");
    // let opt_sample : Option<i32> = Some(1);
    match opt_sample {
        Some(x) if x % 2 == 0 => println!("Guusuu: {}", x),
        Some(x) => println!("Kisuu: {}", x),
        None => println!("No value."),
    }
}

fn basic_code() {
    println!("basic ----------------");

    // value is constraint in Rust
    let x = 5;
    println!("x = {}", x);

    // mut: value can be changed
    let mut y = 9;
    println!("y = {}", y);
    y = 100;
    println!("y = {}", y);

    let v1 = vec![1, 2, 3, 4, 5];
    println!("++++++ for");
    for element in &v1 {
        println!("Vector {}", element);
    }
    println!("Vector v1[0] {}", v1[0]);
    println!("Vector v1[1] {}", v1[1]);

    println!("++++++ iterator");
    let v = vec!["a".to_string(), "b".to_string()];
    for s in v.into_iter() {
        // s has type String, not &String
        println!("into_iter:{}", s);
    }
}

fn main() {
    println!("Hello, world!");

    basic_code();
    adapter();
    pattern_match(Some(1));
    pattern_match(Some(2));
    pattern_match(Some(3));
    pattern_match(Some(4));
    pattern_match(None);
}
