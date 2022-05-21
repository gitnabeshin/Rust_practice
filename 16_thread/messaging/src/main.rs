use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn basic() {
    println!("==== Start basic ====");

    // tx: a transmitter
    // rx: a receiver
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hi");
        println!("Send {}", val); //don't do this after send()
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    });

    let recieved = rx.recv().unwrap();
    println!("Got {}", recieved);
}

fn message() {
    println!("==== Start message ====");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            println!("Send {}", val); //don't do this after send()
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recieved in rx {
        println!("  Got {}", recieved);
    }
}

fn multiple_message() {
    println!("==== Start message multiple ====");

    let (tx, rx) = mpsc::channel();

    // send multiple sender
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("message"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            println!("Send th1 {}", val); //don't do this after send()
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            println!("Send th2 {}", val); //don't do this after send()
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recieved in rx {
        println!("  Got {}", recieved);
    }
}

fn main() {
    basic();
    message();
    multiple_message();
}
