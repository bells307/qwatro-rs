use crate::join_streams;
use crate::strategy::{HostToServerMap, ProxyStrategy};
use async_trait::async_trait;
use std::net::SocketAddr;
use tokio::io::{self, AsyncRead, AsyncWrite, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_util::sync::CancellationToken;

/// TCP-проксирование
pub struct TcpProxy;

#[async_trait]
impl ProxyStrategy for TcpProxy {
    async fn run(&self, ct: CancellationToken, hs_map: HostToServerMap) -> io::Result<()> {
        for (host, server) in hs_map {
            log::info!("starting tcp proxy, host: {}, server: {}", host, server);

            let ln = TcpListener::bind(host).await?;
            tokio::spawn(incoming_connections_loop(ct.clone(), ln, server));
        }

        Ok(())
    }
}

/// Цикл приема входящих TCP-подключений
async fn incoming_connections_loop(ct: CancellationToken, ln: TcpListener, server: SocketAddr) {
    loop {
        tokio::select! {
            Ok((mut in_stream, in_addr)) = ln.accept() => {
                // На каждое подключение создаем таск обработки подключения
                let ct = ct.clone();
                match TcpStream::connect(server).await {
                    Ok(mut out_stream) => {
                        tokio::spawn(async move {
                            log::debug!("incoming tcp connection: {}", in_addr);
                            join_streams(ct, in_stream.split(), out_stream.split()).await;
                            log::debug!("closed tcp connection: {}", in_addr);
                        });
                    },
                    Err(err) => {
                        log::error!("can't connect to server {}: {}", server, err);
                    }
                };
            }
            // `CancellationToken` отменен, больше не нужно принимать входящие соединения
            _ = ct.cancelled() => break
        }
    }
}
