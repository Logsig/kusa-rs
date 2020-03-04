use tokio::prelude::*;
use futures_util::{SinkExt};
use std::{error::Error};
use log::info;

use tokio::stream::{StreamExt};
use tokio::net::{TcpStream, TcpListener};
use tokio_util::codec::{ LinesCodec, FramedRead };
use tokio_tungstenite::{accept_async};


pub async fn build_server(mut listener: TcpListener){
        let mut incoming = listener.incoming();
        while let Some(socket_res) = incoming.next().await {    // Blocking call await for incoming connections
            match socket_res {
                Ok(mut socket) => {
                    println!("Accepted connection from {:?}", socket.peer_addr().unwrap());
                    let _ = socket.write_all(b"Welcome to KUSA Server!\n").await;


                    tokio::spawn(async move {   // Spawn async handler(may or may not be a thread)
                        
                        if let Err(e) = handle_connection(socket).await {
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
}


async fn process(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    
    let (reader, mut writer) = stream.split();

    let mut framed_reader = FramedRead::new(reader, LinesCodec::new());
    // let mut framedWriter = FramedWrite::new(writer, LinesCodec::new())

    // We loop while there are messages coming from the Stream `framed`.
    // The stream will return None once the client disconnects.
    while let Some(message) = framed_reader.next().await {
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

// WebSocket
async fn handle_connection(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let peer = stream.peer_addr().expect("connected streams should have a peer address");
    info!("Peer address: {}", peer);
    let mut ws_stream = accept_async(stream).await.expect("Failed to accept");

    info!("New WebSocket connection: {}", peer);

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if msg.is_text() || msg.is_binary() {
            ws_stream.send(msg).await?;
        }
    }

    Ok(())
}