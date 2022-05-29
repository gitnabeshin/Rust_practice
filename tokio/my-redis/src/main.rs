use bytes::Bytes;
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

async fn server_process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    // let mut db = HashMap::new();

    let mut conn = Connection::new(socket);

    while let Some(frame) = conn.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                println!("SET: {:?}", cmd);

                // needs mut here for insert
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                println!("GET: {:?}", cmd);

                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        conn.write_frame(&response).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    // bind listener to this address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        // pub async fn accept(&self) -> Result<(TcpStream, SocketAddr)>
        let (socket, _) = listener.accept().await.unwrap();

        println!("accepted");

        let db = db.clone();

        // Spawns a new asynchronous task
        tokio::spawn(async move {
            server_process(socket, db).await;
        });
    }
}
