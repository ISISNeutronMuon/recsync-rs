pub mod record;
pub use self::record::Record;

use std::{io, net::{IpAddr, Ipv4Addr, SocketAddr}};
use tokio::{net::{UdpSocket, TcpStream}, io::Interest}; 
use tokio_util::codec::Framed;
use tracing::{debug, error, info};
use wire::{Announcement, Message, MessageCodec, MSG_MAGIC_ID};
use tokio_stream::StreamExt;
use futures::SinkExt;

pub struct Reccaster {
    udpsock: UdpSocket,
    framed: Option<Framed<TcpStream, MessageCodec>>,
    buf: [u8; 1024],
    pvs: Vec<Record>,
    state: CasterState,
}

enum CasterState {
    Announcement,
    Handshake(Announcement),
    Upload,
    PingPong,
}

impl Reccaster {

    pub async fn new(records: Vec<Record>) -> Reccaster {
        let sock = UdpSocket::bind(format!("0.0.0.0:{}", wire::SERVER_ANNOUNCEMENT_UDP_PORT)).await.unwrap();
        debug!("listening for announcement messages at {}", wire::SERVER_ANNOUNCEMENT_UDP_PORT);
        Self { udpsock: sock, framed: None, buf: [0; 1024], pvs: records, state: CasterState::Announcement } 
    }

    pub async fn run(&mut self) {
        loop {
            match self.state {
                CasterState::Announcement => self.handle_announcement().await,
                CasterState::Handshake(_) => self.handle_handshake().await,
                CasterState::Upload => self.handle_upload().await,
                CasterState::PingPong => self.handle_pingpong().await,
            }
        }
    }

    async fn handle_announcement(&mut self) {
        let ready = self.udpsock.ready(Interest::READABLE).await.unwrap();
        if ready.is_readable() {
            match self.udpsock.try_recv_from(&mut self.buf) {
                Ok((len, addr)) => {
                    if len >= 16 {
                        let msg = Self::parse_announcement_message(&self.buf[..len], addr).unwrap();
                        info!("Received announcement message: {:?}:{:?} with key:{:?} from: {:?}", msg.server_addr, msg.server_port, msg.server_key, addr);
                        self.state = CasterState::Handshake(msg);
                    }
                },
                Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => { return; },
                Err(err) => { error!("{:?}", err) }
            };
        }
    }

    async fn handle_handshake(&mut self) {
        if let CasterState::Handshake(msg) = &mut self.state {
            let addr = msg.server_addr;
            let port = msg.server_port;
            let key = msg.server_key;
            // @TODO handle connection errors 
            let stream = TcpStream::connect(format!("{}:{}", addr, port)).await.map_err(|err| error!("{:?}",err)).unwrap();
            info!("connect to {:?}:{}", addr, port);
            let codec = MessageCodec;
            let framed = Framed::new(stream, codec);
            self.framed = Some(framed);

            if let Some(framed) = &mut self.framed {    
                while let Some(msg) = framed.next().await {
                    match msg.unwrap() {
                        Message::ServerGreet(_) => {
                            let _ = framed.send(Message::ClientGreet(wire::ClientGreet { serv_key: key })).await;
                            debug!("Greet Message with server key: {}", key);
                            self.state = CasterState::Upload;
                            return;
                        },
                        _ => {
                            self.state = CasterState::Announcement;
                            return;
                        },
                    }
                }
            }
        }
    }

    async fn handle_upload(&mut self) {
        if let CasterState::Upload = &mut self.state {
            if let Some(framed) = &mut self.framed {
                for (i, record) in self.pvs.iter().enumerate() {
                    let recid: u32 = i as u32 + 100; 
                    // AddRecord Message
                    let record_name = &record.name;
                    let record_type = &record.r#type;
                    let msg = Message::AddRecord(wire::AddRecord { recid: recid, atype: wire::AddRecordType::Record as u8, rtlen: record_type.len() as u8, rnlen: record_name.len() as u16, 
                        rtype: record_type.to_string(), rname: record_name.to_string() });
                    let _ = framed.send(msg.clone()).await;
                    debug!("Sending AddRecord Message: {:?}", msg);
                    // AddRecord alias Message if avaliable
                    if let Some(record_alias) = &record.alias {
                        let msg = Message::AddRecord(wire::AddRecord { recid: recid, atype: wire::AddRecordType::Alias as u8, rtlen: record_type.len() as u8, rnlen: record_alias.len() as u16, 
                            rtype: record_type.to_string(), rname: record_alias.to_string() });
                        let _ = framed.send(msg.clone()).await;
                    };
                    // AddInfo Message
                    for (key, value) in &record.properties {
                        let msg = Message::AddInfo(wire::AddInfo { recid: recid, keylen: key.len() as u8, valen: value.len() as u16, key: key.to_string(), value: value.to_string() });
                        let _ = framed.send(msg.clone()).await;
                        debug!("Sending AddInfo Message: {:?}", msg.clone());
                    }
                }
                let _ = framed.send(Message::UploadDone(wire::UploadDone)).await;
                debug!("Sending UploadDone Message");
                self.state = CasterState::PingPong;
            }
        }
    }

    async fn handle_pingpong(&mut self) {
        if let CasterState::PingPong = &mut self.state {
            if let Some(framed) = &mut self.framed {
                while let Some(msg_result) = dbg!(framed.next().await) {
                    match msg_result {
                        Ok(msg) => {
                            match msg {
                                Message::Ping(ping_msg) => {
                                    if let Err(_) = framed.send(Message::Pong(wire::Pong { nonce: ping_msg.nonce })).await {
                                        self.state = CasterState::Announcement;
                                        return;
                                    }
                                },
                                _ => {
                                    self.state = CasterState::Announcement;
                                    return;
                                },
                            }
                        },
                        Err(_) => {
                            self.state = CasterState::Announcement;
                            return;
                        }
                    }
                } 
                self.state = CasterState::Announcement;
                return;
            }
        }
    }

    fn parse_announcement_message(data: &[u8], src_addr: SocketAddr) -> Result<Announcement, &'static str> {
        let id = u16::from_be_bytes([data[0], data[1]]);
        // Checking if the ID is 'RC'
        if id != MSG_MAGIC_ID {             
            return Err("Invalid ID");
        }

        let version = data[2];
        if version != 0 {
            return Err("Invalid version");
        }

        // Extracting the server address (IPv4, 4 bytes)
        let server_addr_bytes = &data[4..8];
        let mut server_addr = Ipv4Addr::new(
            server_addr_bytes[0],
            server_addr_bytes[1],
            server_addr_bytes[2],
            server_addr_bytes[3],
        );

        if server_addr.is_broadcast() {
            match src_addr.ip() {
                IpAddr::V4(addr) => { server_addr = addr; },
                IpAddr::V6(_) => { unimplemented!("IPv6 is not supported") },
            }
        }

        let server_port = u16::from_be_bytes([data[8], data[9]]);

        let server_key = u32::from_be_bytes([data[12], data[13], data[14], data[15]]);

        Ok(Announcement {
            id,
            server_addr,
            server_port,
            server_key,
        })
    }
}
