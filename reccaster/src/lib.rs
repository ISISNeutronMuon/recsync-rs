use std::{io, net::{IpAddr, Ipv4Addr, SocketAddr}};
use tokio::{net::{UdpSocket, TcpStream}, io::Interest}; 
use tokio_util::codec::Framed;
use wire::{Announcement, MSG_ID, MessageCodec, Message};
use tokio_stream::StreamExt;
use futures::SinkExt;

pub struct Reccaster {
    udpsock: UdpSocket,
    framed: Option<Framed<TcpStream, MessageCodec>>,
    buf: [u8; 1024],
    pvs: Vec<String>,
    state: CasterState,
}

enum CasterState {
    Announcement,
    Handshake(Announcement),
    Upload,
    PingPong,
}

impl Reccaster {
    pub async fn new() -> Reccaster{
        let sock = UdpSocket::bind(format!("0.0.0.0:{}", wire::SERVER_ANNOUNCEMENT_UDP_PORT)).await.unwrap();
        let pvs: Vec<String> = vec!["DEV:JEM".to_string()];
        Self { udpsock: sock, framed: None, buf: [0; 1024], pvs, state: CasterState::Announcement } 
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
        println!("ANNOUNCEMENT_STATE");
        let ready = self.udpsock.ready(Interest::READABLE).await.unwrap();
        if ready.is_readable() {
            match self.udpsock.try_recv_from(&mut self.buf) {
                Ok((len, addr)) => {
                    if len >= 16 {
                        let msg = Self::parse_announcement_message(&self.buf[..len], addr).unwrap();
                        println!("Received announcement message: {:?}:{:?} with key:{:?} from: {:?}", msg.server_addr, msg.server_port, msg.server_key, addr);
                        self.state = CasterState::Handshake(msg);
                    }
                },
                Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => { return; },
                Err(err) => { println!("{:?}", err) }
            };
        }
    }

    async fn handle_handshake(&mut self) {
        println!("HANDSHAKE_STATE");
        if let CasterState::Handshake(msg) = &mut self.state {
            let addr = msg.server_addr;
            let port = msg.server_port;
            let key = msg.server_key;
            let stream = TcpStream::connect(format!("{}:{}", addr, port)).await.unwrap();
            let codec = MessageCodec;
            let framed = Framed::new(stream, codec);
            self.framed = Some(framed);

            if let Some(framed) = &mut self.framed {    
                while let Some(msg) = framed.next().await {
                    match msg.unwrap() {
                        Message::ServerGreet(_) => {
                            println!("Server is Greeting 👋");
                            framed.send(Message::ClientGreet(wire::ClientGreet { serv_key: key })).await;

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
        println!("UPLOAD_STATE");
        if let CasterState::Upload = &mut self.state {
            if let Some(framed) = &mut self.framed {
                // @TODO upload data using add record
                println!("Sending AddRecord Message 📧");
                let atype_add_record = 1;
                let record_name = "DEV:JEM";
                let record_type = "PERSON";
                let msg = Message::AddRecord(wire::AddRecord { recid: 0 as u32, atype: atype_add_record, rtlen: record_type.len() as u8, rnlen: record_name.len() as u16, 
                    rtype: record_type.to_string(), rname: record_name.to_string() });
                framed.send(msg.clone()).await.unwrap();
                println!("Sending AddRecord Message 📧.\n{:?}", msg);
                framed.send(Message::UploadDone(wire::UploadDone)).await.unwrap();
                println!("Sending UploadDone Message 🆗");
                self.state = CasterState::PingPong;
            }
        }
    }

    async fn handle_pingpong(&mut self) {
        println!("PINGPONG_STATE");
        if let CasterState::PingPong = &mut self.state {
            if let Some(framed) = &mut self.framed {
                while let Some(msg_result) = dbg!(framed.next().await) {
                    match msg_result {
                        Ok(msg) => {
                            match msg {
                                Message::Ping(ping_msg) => {
                                    println!("Server is Pinging 🏓");
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
        if id != MSG_ID {             
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
                IpAddr::V6(_) => { unimplemented!("IPv6 is not supported")},
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
