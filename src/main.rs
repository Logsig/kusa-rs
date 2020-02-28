use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::{TcpStream, TcpListener};
use tokio::prelude::*;
use tokio::stream::StreamExt;
use std::{env, error::Error, fmt, io};
use tokio_util::codec::{ LinesCodec, FramedRead, FramedWrite };

#[tokio::main]
async fn main() {

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6142);
    let mut listener = TcpListener::bind(addr).await.unwrap();

    println!("Binding port 6142");


    // Here we convert the `TcpListener` to a stream of incoming connections
    // with the `incoming` method.
    let server = async move { 
        let mut incoming = listener.incoming();
        while let Some(socket_res) = incoming.next().await {    // Blocking call await for incoming connections
            match socket_res {
                Ok(mut socket) => {
                    println!("Accepted connection from {:?}", socket.peer_addr().unwrap());
                    socket.write_all(b"Welcome to KUSA Server!\n").await;

                    tokio::spawn(async move {   // Spawn async handler(may or may not be a thread)
                        
                        if let Err(e) = process(socket).await {
                            println!("failed to process connection; error = {}", e);
                        }
                        
                      });

                }
                Err(err) => {
                    // Handle error by printing to STDOUT.
                    println!("accept error = {:?}", err);
                }
            }
        }
    };

    println!("Server started and running...");

    // Waiting(blocking) for `server`. Without this, the main entry point will exit immediately.
    server.await;   

    // 1. Spin up broker
}

async fn process(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    
    let (reader, mut writer) = stream.split();

    let mut framedReader = FramedRead::new(reader, LinesCodec::new());
    // let mut framedWriter = FramedWrite::new(writer, LinesCodec::new())

    // We loop while there are messages coming from the Stream `framed`.
    // The stream will return None once the client disconnects.
    while let Some(message) = framedReader.next().await {
        match message {
            Ok(line) => match line.as_str() {
                "ping" => writer.write_all(b"pong").await?,
                "quit" => {
                    writer.write_all(b"byebye").await?;
                    return Ok(())
                }, // Connection closed
                _ => {
                    println!("Unknown command: {:?}", line); 
                    writer.write_all(b"I do no understand").await?;
                },
            }
            Err(err) => println!("Socket closed with error: {:?}", err),
        }
    }
    println!("Socket received FIN packet and closed connection");

    Ok(())
}
