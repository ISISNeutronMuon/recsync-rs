// This file is part of Recsync-rs.
// Copyright (c) 2024 UK Research and Innovation, Science and Technology Facilities Council
//
// This project is licensed under both the MIT License and the BSD 3-Clause License.
// You must comply with both licenses to use, modify, or distribute this software.
// See the LICENSE file for details.

use bytes::{BufMut, BytesMut};
use std::mem::size_of;
use crate::MSG_MAGIC_ID;

#[derive(Debug, Clone, PartialEq)]
pub struct MessageHeader {
    pub id: u16,
    pub msg_id: u16,
    pub len: u32,
}

impl MessageHeader {
    pub fn new(msg_id: u16, len: u32) -> MessageHeader {
        MessageHeader { id: MSG_MAGIC_ID, msg_id, len }
    }

    /// Return Header as BytesMut
    pub fn as_bytes(&self) -> BytesMut {
        let mut buf = BytesMut::with_capacity(size_of::<MessageHeader>());
        buf.put_u16(self.id);
        buf.put_u16(self.msg_id);
        buf.put_u32(self.len);
        buf
    }
}
