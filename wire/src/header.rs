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
