#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater,
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("  Lucky Penny!!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater => 25,
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // etc.
}

#[derive(Debug)]
enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quater(UsState),
}

fn value_in_cents2(coin: &Coin2) -> u32 {
    match coin {
        Coin2::Penny => {
            println!("  Lucky Penny!!");
            1
        }
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quater(state) => {
            println!("  State quater from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn some_value(val: u32) {
    match val {
        1 => println!("  one"),
        2 => println!("  two"),
        3 => println!("  three"),
        4 => println!("  four"),
        _ => println!("  other value."),
    }
}

fn some_value2(val: u32) {
    match val {
        3 => println!("  three"),
        _ => println!("  other value."),
    }

    if let 3 = val {
        println!("  THREE");
    } else {
        println!("  OTHER VALUE.");
    }
}

fn main() {
    let coin = Coin::Dime;
    println!("1:{:?}, {}", coin, value_in_cents(&coin));
    let coin = Coin::Penny;
    println!("2:{:?}, {}", coin, value_in_cents(&coin));
    let coin = Coin::Nickel;
    println!("3:{:?}, {}", coin, value_in_cents(&coin));
    let coin = Coin::Quater;
    println!("4:{:?}, {}", coin, value_in_cents(&coin));

    println!("");

    let coin = Coin2::Dime;
    println!("5:{:?}, {}", coin, value_in_cents2(&coin));
    let coin = Coin2::Penny;
    println!("6:{:?}, {}", coin, value_in_cents2(&coin));
    let coin = Coin2::Nickel;
    println!("7:{:?}, {}", coin, value_in_cents2(&coin));
    let coin = Coin2::Quater(UsState::Alabama);
    println!("8:{:?}, {}", coin, value_in_cents2(&coin));
    let coin = Coin2::Quater(UsState::Alaska);
    println!("9:{:?}, {}", coin, value_in_cents2(&coin));

    let five = Some(5);
    assert_eq!(Some(6), plus_one(five));
    assert_eq!(Some(-9), plus_one(Some(-10)));
    assert_eq!(None, plus_one(None));

    println!("");

    some_value(1);
    some_value(2);
    some_value(3);
    some_value(4);
    some_value(5);
    some_value(22);

    println!("");

    some_value2(2);
    some_value2(3);
    some_value2(4);
}
