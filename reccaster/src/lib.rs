use std::{io, net::{SocketAddr, IpAddr, Ipv4Addr}};
use tokio::{net::{UdpSocket, TcpStream}, io::Interest}; 
use tokio_util::codec::Framed;
use wire::{Announcement, MSG_ID, MessageCodec, Message};
use tokio_stream::StreamExt;

pub struct Reccaster {
    udpsock: UdpSocket,
    buf: [u8; 1024],
    pvs: Vec<String>,
}

impl Reccaster {
    pub async fn new() -> Reccaster{
        let sock = UdpSocket::bind(format!("0.0.0.0:{}", wire::SERVER_ANNOUNCEMENT_UDP_PORT)).await.unwrap();
        Self { udpsock: sock, buf: [0; 1024], pvs: Vec::new() } 
    }

    pub async fn run(&mut self) {
        loop {
            let ready = self.udpsock.ready(Interest::READABLE).await.unwrap();

            if ready.is_readable() {
                match self.udpsock.try_recv_from(&mut self.buf) {
                    Ok((len, addr)) => {
                        if len >= 16 {
                            let msg = Self::parse_announcement_message(&self.buf[..len], addr).unwrap();
                            println!("Received announcement message: {:?}:{:?} with key:{:?} from: {:?}", msg.server_addr, msg.server_port, msg.server_key, addr);
                            Self::handle_handshake(msg).await;
                        }
                    },
                    Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => { continue; },
                    Err(err) => { println!("{:?}", err) }
                };
            }
        }
    }

    async fn handle_handshake(msg: Announcement) {
        let addr = msg.server_addr;
        let stream = TcpStream::connect(addr.to_string()).await.unwrap();
        let codec = MessageCodec;
        let mut framed = Framed::new(stream, codec);
        while let Some(msg) = framed.next().await {
            
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
