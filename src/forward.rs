use anyhow::{Ok, Result};
use core::panic;
use std::result::Result::Ok as StdOK;
use tokio::{
    io::{copy, split, AsyncRead, AsyncWrite},
    net::{TcpListener, TcpStream, UnixListener, UnixStream},
};

#[derive(Debug, PartialEq)]
pub enum Schema {
    Tcp,
    Unix,
}

fn parse_schema(schema: String) -> Result<Schema> {
    match schema.as_str() {
        "tcp" => Ok(Schema::Tcp),
        "unix" => Ok(Schema::Unix),
        _ => Err(anyhow::anyhow!("Invalid schema: {}", schema)),
    }
}

fn split_addr(addr: String) -> Result<(Schema, String)> {
    let mut parts = addr.split("://");
    let schema = parts.next().unwrap();
    let addr = parts.next().unwrap();
    let schema = parse_schema(schema.to_string())?;
    Ok((schema, addr.to_string()))
}

pub async fn run(src: String, dst: String) -> Result<()> {
    let (src_schema, src_addr) = split_addr(src)?;
    let (dst_schema, dst_addr) = split_addr(dst)?;

    info!("Source: [{:?}] {}", src_schema, src_addr);
    info!("Destination: [{:?}] {}", dst_schema, dst_addr);

    match src_schema {
        Schema::Tcp => {
            let listener = TcpListener::bind(src_addr).await?;
            loop {
                match dst_schema {
                    Schema::Tcp => match listener.accept().await {
                        StdOK((stream, _addr)) => {
                            info!("New connection: {:?}", stream);
                            let dst_stream = TcpStream::connect(dst_addr.clone()).await?;
                            proxy(stream, dst_stream).await?;
                        }
                        Err(e) => {
                            panic!("accept failed = {:?}", e);
                        }
                    },
                    Schema::Unix => match listener.accept().await {
                        StdOK((stream, _addr)) => {
                            info!("New connection: {:?}", stream);
                            let dst_stream = UnixStream::connect(dst_addr.clone()).await?;
                            proxy(stream, dst_stream).await?;
                        }
                        Err(e) => {
                            panic!("accept failed = {:?}", e);
                        }
                    },
                }
            }
        }
        Schema::Unix => {
            let listener = UnixListener::bind(src_addr)?;
            loop {
                match dst_schema {
                    Schema::Tcp => match listener.accept().await {
                        StdOK((stream, _addr)) => {
                            info!("New connection: {:?}", stream);
                            let dst_stream = TcpStream::connect(dst_addr.clone()).await?;
                            proxy(stream, dst_stream).await?;
                        }
                        Err(e) => {
                            panic!("accept failed = {:?}", e);
                        }
                    },
                    Schema::Unix => match listener.accept().await {
                        StdOK((stream, _addr)) => {
                            info!("New connection: {:?}", stream);
                            let dst_stream = UnixStream::connect(dst_addr.clone()).await?;
                            proxy(stream, dst_stream).await?;
                        }
                        Err(e) => {
                            panic!("accept failed = {:?}", e);
                        }
                    },
                }
            }
        }
    }
}

pub async fn proxy<S1, S2>(stream1: S1, stream2: S2) -> Result<()>
where
    S1: AsyncRead + AsyncWrite + Unpin,
    S2: AsyncRead + AsyncWrite + Unpin,
{
    let (mut s1_read, mut s1_write) = split(stream1);
    let (mut s2_read, mut s2_write) = split(stream2);
    tokio::select! {
        res = copy(&mut s1_read, &mut s2_write) => res,
        res = copy(&mut s2_read, &mut s1_write) => res,
    }?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_schema() {
        assert_eq!(parse_schema("tcp".to_string()).unwrap(), Schema::Tcp);
        assert_eq!(parse_schema("unix".to_string()).unwrap(), Schema::Unix);
        assert!(parse_schema("invalid".to_string()).is_err());
    }

    #[test]
    fn test_split_addr() {
        assert_eq!(
            split_addr("tcp://127.0.0.1:9299".to_string()).unwrap(),
            (Schema::Tcp, "127.0.0.1:9299".to_string())
        );
    }
}

#[tokio::test]
async fn test_run() {
    assert!(run(
        "tcp://127.0.0.1:8081".to_string(),
        "tcp://127.0.0.1:8080".to_string()
    )
    .await
    .is_ok());
}
