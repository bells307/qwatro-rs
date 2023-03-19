use crate::strategy::ProxyStrategy;
use async_trait::async_trait;
use std::net::SocketAddr;
use tokio::io::{self, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_util::sync::CancellationToken;

/// TCP-проксирование
pub struct TcpProxy;

#[async_trait]
impl ProxyStrategy for TcpProxy {
    async fn run(
        &self,
        ct: CancellationToken,
        listen: SocketAddr,
        server: SocketAddr,
    ) -> io::Result<()> {
        log::info!(
            "starting tcp proxy, listening port: {}, remote server: {}",
            listen,
            server
        );

        let ln = TcpListener::bind(listen).await?;

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Ok((mut in_stream, addr)) = ln.accept() => {
                        log::debug!("incoming tcp connection: {}", addr);

                        let mut out_stream = match TcpStream::connect(server).await {
                            Ok(s) => s,
                            Err(err) => {
                                log::error!("can't connect to server {}: {}", server, err);
                                continue;
                            }
                        };

                        let (mut ri, mut wi) = in_stream.split();
                        let (mut ro, mut wo) = out_stream.split();

                        let client_to_server = async {
                            io::copy(&mut ri, &mut wo).await?;
                            wo.shutdown().await
                        };

                        let server_to_client = async {
                            io::copy(&mut ro, &mut wi).await?;
                            wi.shutdown().await
                        };

                        if let Err(err) = tokio::try_join!(client_to_server, server_to_client) {
                            log::error!("{}", err);
                        };

                        log::debug!("closed tcp connection: {}", addr);
                    }
                    _ = ct.cancelled() => break
                }
            }
        });

        Ok(())
    }
}
