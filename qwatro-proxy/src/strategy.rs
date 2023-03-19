use async_trait::async_trait;
use std::net::SocketAddr;
use tokio::io;
use tokio_util::sync::CancellationToken;

/// Стратегия проксирования
#[async_trait]
pub trait ProxyStrategy {
    /// Запуск задачи проксирования
    /// * `ct`: `CancellationToken`, по завершении которого задача проксирования будет остановлена
    /// * `listen`: адрес, на котором будет открыт порт входящих соединений
    /// * `server`: адрес, на который будет происходить проксирование
    async fn run(
        &self,
        ct: CancellationToken,
        listen: SocketAddr,
        server: SocketAddr,
    ) -> io::Result<()>;
}
