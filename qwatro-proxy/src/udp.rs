use crate::strategy::{HostToServerMap, ProxyStrategy};
use async_trait::async_trait;
use std::pin::{pin, Pin};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, ReadBuf};
use tokio_util::sync::CancellationToken;
use udp_stream::UdpStream;

pub struct UdpProxy;

#[async_trait]
impl ProxyStrategy for UdpProxy {
    async fn run(&self, ct: CancellationToken, hs_map: HostToServerMap) -> std::io::Result<()> {
        todo!()
    }
}

pub struct UdpStreamReadHalf(Arc<Mutex<UdpStream>>);

impl AsyncRead for UdpStreamReadHalf {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut *self.0.lock().unwrap()).poll_read(cx, buf)
    }
}

pub struct UdpStreamWriteHalf(Arc<Mutex<UdpStream>>);

// pub fn split<'a>(mut stream: UdpStream) -> (UdpStreamReadHalf<'a>, UdpStreamWriteHalf<'a>) {
//     let read = UdpStreamReadHalf(pin!(stream));
//     let write = UdpStreamWriteHalf(pin!(stream));
//
//     (read, write)
// }

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use futures::{SinkExt, StreamExt};
    use std::net::SocketAddr;
    use tokio_util::codec::BytesCodec;
    use tokio_util::udp::UdpFramed;
    use udp_stream::UdpStream;

    #[tokio::test]
    async fn udp_test() {
        let s = UdpStream::connect("127.0.0.1:9999".parse::<SocketAddr>().unwrap())
            .await
            .unwrap();

        let listen_sock = tokio::net::UdpSocket::bind("127.0.0.1:9998").await.unwrap();
        let mut framed = UdpFramed::new(listen_sock, BytesCodec::new()).split();
        // let remote = "127.0.0.1:9999".parse::<SocketAddr>().unwrap();
        // framed.send((Bytes::from(vec![1]), remote)).await.unwrap();
        // while let Some(v) = framed.next().await {
        //     dbg!(v);
        // }
    }
}
