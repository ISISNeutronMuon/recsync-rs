use bytes::{Buf, BufMut, BytesMut};
use std::{io, mem::size_of, net::Ipv4Addr};
use tokio_util::codec::{Decoder, Encoder};

/// UDP broadcast port
pub const SERVER_ANNOUNCEMENT_UDP_PORT: u16 = 5049;

/// Message ID Magic number (ascii "RC")
pub const MSG_MAGIC_ID: u16 = 0x5243;

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

impl Message {
    fn id(&self) -> MessageID {
        match self {
            Message::ServerGreet(_) => MessageID::ServerGreet,
            Message::Ping(_) => MessageID::Ping,
            Message::ClientGreet(_) => MessageID::ClientGreet,
            Message::Pong(_) => MessageID::Pong,
            Message::AddRecord(_) => MessageID::AddRecord,
            Message::DelRecord(_) => MessageID::DelRecord,
            Message::UploadDone(_) => MessageID::UploadDone,
            Message::AddInfo(_) => MessageID::AddInfo,
        }
    }
}

impl MessageHeader {
    pub fn new(msg_id: u16, len: u32) -> MessageHeader {
        MessageHeader { id: MSG_MAGIC_ID, msg_id, len }
    }

    /// return Header as BytesMut
    pub fn as_bytes(&self) -> BytesMut {
        let mut buf = BytesMut::with_capacity(size_of::<MessageHeader>());
        buf.put_u16(self.id);
        buf.put_u16(self.msg_id);
        buf.put_u32(self.len);
        buf
    }
}

/// Encoders and Decoders for Messages
pub struct MessageCodec;

impl Encoder<Message> for MessageCodec {
    type Error = io::Error;

    fn encode(&mut self, msg: Message, dst: &mut BytesMut) -> Result<(), Self::Error> {
        match msg {
            Message::ClientGreet(msg) => {
                let header = MessageHeader::new(MessageID::ClientGreet as u16, (size_of::<u32>() + size_of::<ClientGreet>())as u32);
                dst.put(header.as_bytes());
                dst.put_u32(0); // Padding
                dst.put_u32(msg.serv_key);
                Ok(())
            },
            Message::Pong(msg) => {
                let header = MessageHeader::new(MessageID::Pong as u16, size_of::<Pong>() as u32);
                dst.put(header.as_bytes());
                dst.put_u32(msg.nonce);
                Ok(())
            },
            Message::AddRecord(msg) => {
                let header = MessageHeader::new(MessageID::AddRecord as u16, (size_of::<AddRecord>() as u32) + (msg.rtlen as u32 + msg.rnlen as u32));
                dst.put(header.as_bytes());
                dst.put_u32(msg.recid);
                dst.put_u8(msg.atype);
                dst.put_u8(msg.rtlen);
                dst.put_u16(msg.rnlen);
                dst.put(msg.rtype.as_bytes());
                dst.put(msg.rname.as_bytes());
                Ok(())
            },
            Message::DelRecord(_) => todo!(),
            Message::AddInfo(msg) => {
                let header = MessageHeader::new(MessageID::AddInfo as u16,
                    (size_of::<u32>() + size_of::<u8>() + size_of::<u8>() + size_of::<u16>()) as u32 + (msg.keylen as u32 + msg.valen as u32) );
                println!("Header size {:?}", header.len);
                dst.put(header.as_bytes());
                dst.put_u32(msg.recid);
                dst.put_u8(msg.keylen);
                dst.put_u16(msg.valen);
                dst.put(msg.key.as_bytes());
                dst.put(msg.value.as_bytes());
                Ok(())
            },
            Message::UploadDone(_) => {
                let header = MessageHeader::new(MessageID::UploadDone as u16, size_of::<u32>() as u32);
                dst.put(header.as_bytes());
                dst.put_u32(0);
                Ok(())
            },
            Message::Ping(_) => unimplemented!("Recceiver related messages are not implemented yet."),
            Message::ServerGreet(_) => unimplemented!("Recceiver related messages are not implemented yet.")
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
        if id != MSG_MAGIC_ID {
            return Ok(None);
        }

        if src.len() < len {
            // Not enough data to read the body
            return Ok(None);
        }

        // Match based on `msg_id` and parse accordingly
        match msg_id.into() {
            MessageID::ServerGreet => {
                let _placeholder = src.get_u8();
                Ok(Some(Message::ServerGreet(ServerGreet)))
            }
            MessageID::Ping => {
                let nonce = src.get_u32();
                Ok(Some(Message::Ping(Ping { nonce })))
            },
            MessageID::ClientGreet => unimplemented!("Recceiver related messages are not implemented yet."),
            MessageID::Pong => unimplemented!("Recceiver related messages are not implemented yet."),
            MessageID::AddRecord => unimplemented!("Recceiver related messages are not implemented yet."),
            MessageID::DelRecord => unimplemented!("Recceiver related messages are not implemented yet."),
            MessageID::UploadDone => unimplemented!("Recceiver related messages are not implemented yet."),
            MessageID::AddInfo => unimplemented!("Recceiver related messages are not implemented yet."),
        }
    }
}

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
