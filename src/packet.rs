use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::{
    io::AsyncRead,
    io::{AsyncReadExt, AsyncWrite, AsyncWriteExt},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Message {
    Ping,
    Pong,
    TunnelReq { port: u16, sub_domain: String },
    TunnelResp { result: String, domain: String },
    TunnelClose { port: u16 },
    ProxyReq { uuid: String },
}

pub struct Packet<T> {
    pub t: T,
}

impl<T: AsyncRead + AsyncWrite + Unpin> Packet<T> {
    pub async fn write(&mut self, msg: Message) -> Result<()> {
        let msg = bincode::serialize(&msg).unwrap();
        let len = msg.len() as u32;
        let len = len.to_be_bytes();
        self.t.write_all(&len).await?;
        self.t.write_all(&msg).await?;
        Ok(())
    }

    pub async fn read(&mut self) -> Result<Message> {
        let mut len = [0u8; 4];
        self.t.read_exact(&mut len).await?;
        let len = u32::from_be_bytes(len) as usize;
        let mut msg = vec![0u8; len];
        self.t.read_exact(&mut msg).await?;
        let msg = bincode::deserialize(&msg).unwrap();
        Ok(msg)
    }
}
