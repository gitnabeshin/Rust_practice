use std::thread;
use std::time::Duration;

use std::thread::JoinHandle;

fn test_move() -> JoinHandle<()> {

    let v = vec![1, 2, 3];

    // needs move for setting param to thread(because it can't be known when thread ends)
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("    [thread] number {}. vector={:?}", i, v);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle
}

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("[thread] number {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let handle2 = test_move();
    handle2.join().unwrap();

    for i in 1..5 {
        println!("[main] num {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
