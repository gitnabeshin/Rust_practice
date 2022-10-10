use std::thread;
use std::time::Duration;
use tokio::time;

fn th_id() -> String {
    format!("{:?}", thread::current().id())
}

async fn thread_test() {
    println!("============ thread_test ================");
    let mut joinhandles = Vec::new();

    for offset in 0..10 {
        joinhandles.push(thread::spawn(move || {
            for i in 0..3 {
                thread::sleep(Duration::from_millis(offset * 10));
                println!("  THREAD:[{}](No.{}) cnt={}", th_id(), offset, i);
            }
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}

async fn tokio_spawn_test() {
    println!("============ tokio_spawn_test ================");

    let mut joinhandles = Vec::new();

    for offset in 0..10 {
        joinhandles.push(tokio::spawn(async move {
            for i in 0..3 {
                time::sleep(Duration::from_millis(offset * 10)).await;
                println!("   TOKIO:[{}](No.{}) cnt={}", th_id(), offset, i);
            }
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    thread_test().await;
    tokio_spawn_test().await;
}
