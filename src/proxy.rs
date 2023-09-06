use anyhow::Result;
use tokio::io::{copy, split, AsyncRead, AsyncWrite};

async fn proxy<S>(s1: S, s2: S) -> Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let (mut r1, mut w1) = split(s1);
    let (mut r2, mut w2) = split(s2);

    tokio::select! {
        _ = copy(&mut r1, &mut w2) => {},
        _ = copy(&mut r2, &mut w1) => {},
    }

    Ok(())
}
