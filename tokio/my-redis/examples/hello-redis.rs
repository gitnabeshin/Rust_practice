use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    // set key, value into redis
    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;

    println!("got val from server; result={:?}", result);

    Ok(())
}

// async fn say_world() {
//     println!("WORLD");
// }

// #[tokio::main]
// async fn main() {
//     // Calling `say_world()` does not execute the body of `say_world()`.
//     let op = say_world();

//     // This println! comes first
//     println!("hello");

//     // Calling `.await` on `op` starts executing `say_world`.
//     op.await;
// }
