// This file is part of Recsync-rs.
// Copyright (c) 2024 UK Research and Innovation, Science and Technology Facilities Council
//
// This project is licensed under both the MIT License and the BSD 3-Clause License.
// You must comply with both licenses to use, modify, or distribute this software.
// See the LICENSE file for details.

use std::net::Ipv4Addr;

/// AddRecord message type
pub enum AddRecordType {
    Record = 0,
    Alias = 1,
}

/// UDP Announcement message structure
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
            _ => unimplemented!("Unknown Message ID"),
        }
    }
}

impl From<MessageID> for u16 {
    fn from(msg_id: MessageID) -> u16 {
        match msg_id {
            MessageID::ServerGreet => 0x8001,
            MessageID::ClientGreet => 0x0001,
            MessageID::Ping => 0x8002,
            MessageID::Pong => 0x0002,
            MessageID::AddRecord => 0x0003,
            MessageID::DelRecord => 0x0004,
            MessageID::UploadDone => 0x0005,
            MessageID::AddInfo => 0x0006,
        }
    }
}

// Define all the message structs and enums here

#[derive(Debug, Clone, PartialEq)]
pub struct ServerGreet;

#[derive(Debug, Clone, PartialEq)]
pub struct Ping {
    pub nonce: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClientGreet {
    pub serv_key: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pong {
    pub nonce: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AddRecord {
    pub recid: u32,
    pub atype: u8,
    pub rtlen: u8,
    pub rnlen: u16,
    pub rtype: String,
    pub rname: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DelRecord {
    pub recid: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UploadDone;

#[derive(Debug, Clone, PartialEq)]
pub struct AddInfo {
    pub recid: u32,
    pub keylen: u8,
    pub valen: u16,
    pub key: String,
    pub value: String,
}

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
