pub mod tcp;

mod strategy;
mod udp;

use crate::strategy::ProxyStrategy;
use std::net::SocketAddr;
use tokio::io;
use tokio_util::sync::CancellationToken;

/// Запуск проксирования
/// * `ct`: `CancellationToken`, по завершении которого задача проксирования будет остановлена
/// * `strategy`: стратегия проксирования
/// * `listen`: адрес, на котором будет открыт порт входящих соединений
/// * `server`: адрес, на который будет происходить проксирование
pub async fn run_proxy(
    ct: CancellationToken,
    strategy: impl ProxyStrategy,
    listen: SocketAddr,
    server: SocketAddr,
) -> io::Result<()> {
    strategy.run(ct, listen, server).await
}
