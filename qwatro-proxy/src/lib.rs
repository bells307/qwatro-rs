pub mod tcp;

mod proxier;

use crate::proxier::ProxyStrategy;
use std::net::SocketAddr;
use tokio::io;
use tokio_util::sync::CancellationToken;

/// Запуск проксирования
pub async fn run_proxy(
    ct: CancellationToken,
    strategy: impl ProxyStrategy,
    listen: SocketAddr,
    server: SocketAddr,
) -> io::Result<()> {
    strategy.run(ct, listen, server).await
}

#[cfg(test)]
mod tests {
    use crate::run_proxy;
    use crate::tcp::TcpProxy;
    use tokio_util::sync::CancellationToken;

    #[tokio::test]
    async fn it_works() {
        let ct = CancellationToken::new();

        run_proxy(
            ct.clone(),
            TcpProxy,
            "127.0.0.1:9998".parse().unwrap(),
            "127.0.0.1:9999".parse().unwrap(),
        )
        .await
        .unwrap();

        ct.cancelled().await;
    }
}
