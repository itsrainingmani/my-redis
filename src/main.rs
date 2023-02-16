use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    // Bind listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // the second item contains the IP and port of the new connection
        let (socket, _) = listener.accept().await.unwrap();
        // This processes inbound requests one at a time. When a connection is accepted,
        // the server stays inside the accept loop until the response is fully written to the socket.
        // process(socket).await;

        // a new task is spawned for each inbound socket. the socket is moved to the new task
        // and processed there
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    // The Connection lets us read/write redis **frames** instead of byte streams
    // The Connection type is defined by mini-redis

    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        // respond with an error
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}
