use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use tokio::net::{TcpListener};
use std::io::prelude::*;

use std::env;

use rumq_core::mqtt4::MqttWrite;

pub mod server;

#[tokio::main]
async fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "client" {
        println!("Client mode");
        // let mut stream = TcpStream::connect(("127.0.0.1", 6142)).await.unwrap();

        if let Ok(mut stream) = TcpStream::connect("127.0.0.1:6142") {
            println!("Connected to the server!");
            let mut connect = rumq_core::mqtt4::Connect::new("test");
            connect.keep_alive = 3600;
            connect.clean_session = true;
            connect.last_will = None;
            let packet = rumq_core::mqtt4::Packet::Connect(connect);

            let mut buffer = Vec::new();

            buffer.mqtt_write(&packet).unwrap();

            dbg!(&buffer);
            
            if let Err(e) = stream.write_all(&buffer).await {
                println!("failed to write to socket; err = {:?}", e);
                return;
            }
        } else {
            println!("Couldn't connect to server...");
        }

        //framed.get/_mut().write_all(b"\0x32\0x2\0x0\0x0").await?;
        
    } else {

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
}

