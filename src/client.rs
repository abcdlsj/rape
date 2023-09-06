use core::panic;

use crate::packet::{Message, Packet};
use anyhow::{Ok, Result};
use log::info;
use tokio::net::{TcpStream};

#[derive(Debug, Clone)]
struct Client {
    port: u16,
    server_addr: String,
}

pub async fn start(port: u16, server_addr: String) -> Result<()> {
    let client = Client { port, server_addr };

    client.handle().await;

    Ok(())
}

impl Client {
    async fn handle(&self) {
        let stream = TcpStream::connect(&self.server_addr).await.unwrap();
        info!("Connected to {}", self.server_addr);
        let mut packet = Packet { t: stream };
        let msg = Message::TunnelReq {
            port: self.port,
            sub_domain: "test".to_string(),
        };

        packet.write(msg).await.unwrap();

        match packet.read().await.unwrap() {
            Message::TunnelResp { result, domain } => {
                info!("Result: {}, Domain: {}", result, domain);

                loop {
                    match packet.read().await.unwrap() {
                        Message::ProxyReq { uuid } => {
                            info!("Data: {}", uuid);
                        }
                        _ => {
                            panic!("Error in tunnel data");
                        }
                    }
                }
            }
            _ => {
                panic!("Error in tunnel create");
            }
        }
    }
}
