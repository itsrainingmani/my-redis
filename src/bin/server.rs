use bytes::Bytes;
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    // Bind listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        // the second item contains the IP and port of the new connection
        let (socket, _) = listener.accept().await.unwrap();

        // Clone the handle to the hash map
        let db = db.clone();
        // This processes inbound requests one at a time. When a connection is accepted,
        // the server stays inside the accept loop until the response is fully written to the socket.
        // process(socket).await;

        // a new task is spawned for each inbound socket. the socket is moved to the new task
        // and processed there
        println!("Accepted");
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    // The Connection lets us read/write redis **frames** instead of byte streams
    // The Connection type is defined by mini-redis
    use mini_redis::Command::{self, Get, Set};

    // A hashmap is used to store data
    // let mut db = HashMap::new();

    let mut connection = Connection::new(socket);

    // use read_frame to receive a command from the connection
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                // The value is stored as a Vec<u8>
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }
}
