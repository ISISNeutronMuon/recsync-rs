
use std::{io, net::{SocketAddr, IpAddr, Ipv4Addr}};
use tokio::{net::UdpSocket, io::Interest};
use wire::{Announcement, ANNOUNCEMENT_MSG_ID}; 

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
                            let msg = Self::parse_announcement_message(&self.buf[..len], addr);
                            println!("Received announcement message from {:?}\n{:?}", addr, msg);
                        }

                    },
                    Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => { println!("would block here"); continue; },
                    Err(err) => { println!("{:?}", err) }
                };
            }
        }
    }

    fn parse_announcement_message(data: &[u8], src_addr: SocketAddr) -> Result<Announcement, &'static str> {
        // @TODO check for zero flag in announcement message
        
        let id = u16::from_be_bytes([data[0], data[1]]);
        // Checking if the ID is 'RC'
        if id != ANNOUNCEMENT_MSG_ID {             
            return Err("Invalid ID");
        }

        let version = data[0];
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
