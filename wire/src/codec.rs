// This file is part of Recsync-rs.
// Copyright (c) 2024 UK Research and Innovation, Science and Technology Facilities Council
//
// This project is licensed under both the MIT License and the BSD 3-Clause License.
// You must comply with both licenses to use, modify, or distribute this software.
// See the LICENSE file for details.

use bytes::{Buf, BufMut, BytesMut};
use std::{io, mem::size_of};
use tokio_util::codec::{Decoder, Encoder};

use crate::{header::MessageHeader, ClientGreet, Message, MessageID, Ping, Pong, ServerGreet};

/// UDP broadcast port
pub const SERVER_ANNOUNCEMENT_UDP_PORT: u16 = 5049;

/// Message ID Magic number (ascii "RC")
pub const MSG_MAGIC_ID: u16 = 0x5243;

/// Encoders and Decoders for Messages
pub struct MessageCodec;

impl Encoder<Message> for MessageCodec {
    type Error = io::Error;

    fn encode(&mut self, msg: Message, dst: &mut BytesMut) -> Result<(), Self::Error> {
        match msg {
            Message::ClientGreet(msg) => {
                let header = MessageHeader::new(MessageID::ClientGreet.into(), (size_of::<u32>() + size_of::<ClientGreet>())as u32);
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
                let len = (size_of::<u32>() + size_of::<u8>() + size_of::<u8>() + size_of::<u16>() + msg.rtype.len() + msg.rname.len()) as u32;
                let header = MessageHeader::new(MessageID::AddRecord.into(), len);
                dst.put_u16(header.id);
                dst.put_u16(header.msg_id);
                dst.put_u32(header.len);
                dst.put_u32(msg.recid);
                dst.put_u8(msg.atype);
                dst.put_u8(msg.rtlen);
                dst.put_u16(msg.rnlen);
                dst.put_slice(msg.rtype.as_bytes());
                dst.put_slice(msg.rname.as_bytes());
                Ok(())
            },
            Message::DelRecord(_) => todo!(),
            Message::AddInfo(msg) => {
                let len = (size_of::<u32>() + size_of::<u8>() + size_of::<u8>() + size_of::<u16>() + msg.key.len() + msg.value.len()) as u32;
                let header = MessageHeader::new(MessageID::AddInfo.into(), len);
                dst.put_u16(header.id);
                dst.put_u16(header.msg_id);
                dst.put_u32(header.len);
                dst.put_u32(msg.recid);
                dst.put_u8(msg.keylen);
                dst.put_u8(0); // Padding
                dst.put_u16(msg.valen);
                dst.put_slice(msg.key.as_bytes());
                dst.put_slice(msg.value.as_bytes());
                Ok(())
            },
            Message::UploadDone(_) => {
                let header = MessageHeader::new(MessageID::UploadDone.into(), size_of::<u32>() as u32);
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
