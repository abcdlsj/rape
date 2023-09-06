use crate::packet::{Message, Packet};
use anyhow::{Ok, Result};
use tokio::net::{TcpListener};


use uuid::Uuid;

#[derive(Debug, Clone)]
struct Server {
    port: u16,
}

pub async fn start(port: u16) -> Result<()> {
    let server = Server { port };

    server.start().await;

    Ok(())
}

impl Server {
    async fn start(&self) {
        let addr = format!("127.0.0.1:{}", self.port);
        let listener = TcpListener::bind(&addr).await.unwrap();
        info!("Server listening on {}", addr);

        loop {
            let (stream, addr) = listener.accept().await.unwrap();
            info!("Accepting from {}", addr);

            let mut packet = Packet { t: stream };
            match packet.read().await.unwrap() {
                Message::TunnelReq { port, sub_domain } => {
                    info!("Port: {}, SubDomain: {}", port, sub_domain);

                    packet
                        .write(Message::TunnelResp {
                            result: "ok".to_string(),
                            domain: "test".to_string(),
                        })
                        .await
                        .unwrap();

                    self.listening(port).await;
                }
                _ => {
                    panic!("Error");
                }
            }
        }
    }

    async fn listening(&self, port: u16) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
            .await
            .unwrap();

        loop {
            let (stream, addr) = listener.accept().await.unwrap();
            info!("Accepting from {}", addr);

            tokio::spawn(async move {
                let mut packet = Packet { t: stream };
                packet
                    .write(Message::ProxyReq {
                        uuid: Uuid::new_v4().to_string(),
                    })
                    .await
                    .unwrap();
            });
        }
    }
}
