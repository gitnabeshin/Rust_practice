// mutex (mutual exclusion)

// use std::rc::Rc;
use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

fn basic() {
    let m = Mutex::new(5);

    {
        // lock() brocks current thread until it can run
        let mut num = m.lock().unwrap();
        *num = 6;
    } // m is unlocked here.

    // m can be accessed
    println!("m = {:?}", m);
}

fn mutex_from_thread() {
    let counter = Arc::new( Mutex::new(0) );
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("@thread {} set number = {}", i, *num);
            thread::sleep(Duration::from_millis(2));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // finally m can be accessed
    println!("m = {}", *counter.lock().unwrap());
}

fn rustlings_arc1_test() {
    println!("rustlings test...");
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers);
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|n| *n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}

fn main() {
    basic();
    mutex_from_thread();
    rustlings_arc1_test();
}