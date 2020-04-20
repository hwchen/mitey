use futures_core::task::{Context, Poll};
use std::io;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, Tokio02AsyncReadCompatExt};

/// Needed because async-std tcpstream impl Clone, but tokio tcpstream doesn't?
#[derive(Clone)]
pub struct TokioCompatStream(Arc<Mutex<Compat<TcpStream>>>);

impl TokioCompatStream {
    pub fn wrap(stream: TcpStream) -> Self {
        Self(Arc::new(Mutex::new(stream.compat())))
    }
}

impl futures_io::AsyncRead for TokioCompatStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut *(&*self.0).lock().unwrap()).poll_read(cx, buf)
    }
}

impl futures_io::AsyncWrite for TokioCompatStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut *(&*self.0).lock().unwrap()).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut *(&*self.0).lock().unwrap()).poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut *(&*self.0).lock().unwrap()).poll_close(cx)
    }
}
