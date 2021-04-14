use std::error::Error;

use bytes::BytesMut;
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::{self as stream, StreamExt, Stream};
use tokio_util::codec::{Encoder, Decoder, Framed};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Packet {
    Reserved = 0,
    Connect = 1
}

/// An error occured while encoding or decoding a line.
#[derive(Debug)]
pub enum MQTTCodecError {
    InvalidPacket,
    /// An IO error occured.
    Io(std::io::Error),
}

// Convert io Error into  MQTTCodecError
impl From<std::io::Error> for MQTTCodecError {
  fn from(e: std::io::Error) -> MQTTCodecError {
    MQTTCodecError::Io(e)
  }
}

pub struct MQTTCodec;

impl MQTTCodec {
  pub fn new() -> MQTTCodec {
    MQTTCodec {}
  }
}

impl Encoder<Packet> for MQTTCodec {
  type Error = MQTTCodecError;

  fn encode(&mut self, packet: Packet, mut buf: &mut BytesMut) -> Result<(), Self::Error> {
    Ok(())
  }
}

impl Decoder for MQTTCodec {
  type Item = Packet;
  type Error = MQTTCodecError;
  fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
    Ok(Some(Packet::Reserved))
  }
}


pub async fn build_server(listener: TcpListener) {
  // Listener loop
  loop {
    let (socket, info) = listener.accept().await.unwrap();
    match handle_mqtt_stream(socket).await {
      Ok(()) => {
        dbg!("Connection accepted: {}", info);
      },
      Err(e) => {
        dbg!(e);
      },
    };
  }
}


async fn handle_mqtt_stream(stream: TcpStream) -> Result<(), Box<dyn Error>> {
  let mut framed = Framed::new(stream, MQTTCodec::new());

  while let Some(packet) = framed.next().await {
    
  }

  Ok(())
}
