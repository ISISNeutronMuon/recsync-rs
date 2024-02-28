use bytes::{Buf, BufMut, BytesMut};
use std::{io, net::Ipv4Addr};
use tokio_util::codec::{Decoder, Encoder};

/// UDP broadcast port
pub const SERVER_ANNOUNCEMENT_UDP_PORT: u16 = 5049;

/// Message ID Magic number (ascii 'RC')
pub const MSG_ID: u16 = 0x5243;

/// UDP Announcement message strcut
#[derive(Debug)]
pub struct Announcement {
    pub id: u16,
    pub server_addr: Ipv4Addr,
    pub server_port: u16,
    pub server_key: u32,
}

/// Messages ID
#[derive(Copy, Clone)]
#[repr(u16)]
pub enum MessageID {
    ServerGreet = 0x8001,
    ClientGreet = 0x0001,
    Ping = 0x8002,
    Pong = 0x0002,
    AddRecord = 0x0003,
    DelRecord = 0x0004,
    UploadDone = 0x0005,
    AddInfo = 0x0006,
}

/// Message Header
#[derive(Debug, Clone, PartialEq)]
pub struct MessageHeader {
    pub id: u16,
    pub msg_id: u16,
    pub len: u32,
}

/// Server Greet Message
#[derive(Debug, Clone, PartialEq)]
pub struct ServerGreet;

/// Ping Message
#[derive(Debug, Clone, PartialEq)]
pub struct Ping {
    pub nonce: u32,
}

/// Client Greet Message
#[derive(Debug, Clone, PartialEq)]
pub struct ClientGreet {
    pub serv_key: u32,
}

/// Pong Message
#[derive(Debug, Clone, PartialEq)]
pub struct Pong {
    pub nonce: u32,
}

/// Add Record Message
#[derive(Debug, Clone, PartialEq)]
pub struct AddRecord {
    pub recid: u32,
    pub atype: u8,
    pub rtlen: u8,
    pub rnlen: u16,
    pub rtype: String,
    pub rname: String,
}

/// Del Record Message
#[derive(Debug, Clone, PartialEq)]
pub struct DelRecord {
    pub recid: u32,
}

/// Upload Done Message
#[derive(Debug, Clone, PartialEq)]
pub struct UploadDone;

/// Add Info Message
#[derive(Debug, Clone, PartialEq)]
pub struct AddInfo {
    pub recid: u32,
    pub keylen: u8,
    pub valen: u16,
    pub key: String,
    pub value: String,
}

/// Message Types
#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    ServerGreet(ServerGreet),
    Ping(Ping),
    ClientGreet(ClientGreet),
    Pong(Pong),
    AddRecord(AddRecord),
    DelRecord(DelRecord),
    UploadDone(UploadDone),
    AddInfo(AddInfo),
}

impl From<u16> for MessageID {
    fn from(value: u16) -> Self {
        match value {
            0x8001 => MessageID::ServerGreet,
            0x0001 => MessageID::ClientGreet,
            0x8002 => MessageID::Ping,
            0x0002 => MessageID::Pong,
            0x0003 => MessageID::AddRecord,
            0x0004 => MessageID::DelRecord,
            0x0005 => MessageID::UploadDone,
            0x0006 => MessageID::AddInfo,
            _ => unimplemented!("Unknown Message ID")
        }
    }
}

/// Encoders and Decoders for Messages
pub struct MessageCodec;

impl Encoder<Message> for MessageCodec {
    type Error = io::Error;

    fn encode(&mut self, msg: Message, dst: &mut BytesMut) -> Result<(), Self::Error> {
        match msg {
            Message::ServerGreet(_) => {
                dst.put_u16(MessageID::ServerGreet as u16);
                dst.put_u32(1); // Length is 1 for Server Greet
                dst.put_u8(0); // Placeholder
                Ok(())
            },
            // ... handle other message types similarly
            _ => unimplemented!(),
        }
    }
}

impl Decoder for MessageCodec {
    type Item = Message;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 8 {
            // Not enough data to read header
            return Ok(None);
        }

        // Read header
        let id = src.get_u16();
        let msg_id = src.get_u16();
        let len = src.get_u32() as usize;
        
        // Checking if the ID is 'RC'
        if id != MSG_ID {
            return Ok(None);
        }


        if src.len() < len {
            // Not enough data to read the body
            return Ok(None);
        }

        // Match based on `msg_id` and parse accordingly
        match msg_id.into() {
            MessageID::ServerGreet => {
                // Parse Server Greet (example)
                let _placeholder = src.get_u8();
                Ok(Some(Message::ServerGreet(ServerGreet)))
            }
            MessageID::ClientGreet => todo!(),
            MessageID::Ping => todo!(),
            MessageID::Pong => todo!(),
            MessageID::AddRecord => todo!(),
            MessageID::DelRecord => todo!(),
            MessageID::UploadDone => todo!(),
            MessageID::AddInfo => todo!(),
        }
    }
}

//@TODO You would continue to implement the `encode` and `decode` methods for each message type.
//@TODO Make sure to handle buffer underflow (not enough data) and any potential parsing errors properly.

//@TODO This is just the foundation for your protocol's codec. You will need to flesh out each message's
//@TODO details according to your application's protocol specifications. The codec can then be integrated
//@TODO with a Tokio-based TCP/UDP server to encode outgoing messages and decode incoming messages.

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn it_works() {
//        let result = add(2, 2);
//        assert_eq!(result, 4);
//    }
//}
