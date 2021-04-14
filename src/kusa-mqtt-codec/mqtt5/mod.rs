/// Protocol type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolVersion {
    V5,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Packet {
    Reserved = 0,
    Connect(VariableHeader, Payload) = 1,
    CONNACK,
    PUBLISH,
    PUBACK,
    PUBREC,
    PUBREL,
    PUBCOMP,
    SUBSCRIBE,
    SUBACK,
    UNSUBSCRIBE,
    UNSUBACK,
    PINGREQ,
    PINGRESP,
    DISCONNECT,
    AUTH
}

impl Packet {
    pub fn new() -> Packet {
        Packet::Reserved
    }

    pub fn decode(&self) -> Result<Packet, Error> {

    }

    /// Returns the size of full packet (fixed header + variable header + payload)
    /// Fixed header is enough to get the size of a frame in the stream
    pub fn frame_length(&self) -> usize {
        self.fixed_header_len + self.remaining_len
    }
}