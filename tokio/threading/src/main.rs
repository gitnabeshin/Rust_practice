use std::thread;
use std::time::Duration;

fn th_id() -> String {
    format!("{:?}", thread::current().id())
}

async fn thread_test() {
    let mut joinhandles = Vec::new();

    for offset in 0..10 {
        joinhandles.push(thread::spawn(move || {
            for i in 0..3 {
                println!("  [{}][No.{}] cnt={}", th_id(), offset, i);
                thread::sleep(Duration::from_millis(offset * 200));
            }
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}

#[tokio::main]
async fn main() {
    thread_test().await;
}
