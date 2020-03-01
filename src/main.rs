use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::{TcpStream, TcpListener};
use tokio::prelude::*;


mod server;

#[tokio::main]
async fn main() {

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6142);
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Binding port 6142");


    // Here we convert the `TcpListener` to a stream of incoming connections
    // with the `incoming` method.
    let server = server::build_server(listener);

    println!("Server started and running...");

    // Waiting(blocking) for `server`. Without this, the main entry point will exit immediately.
    server.await;   
}

