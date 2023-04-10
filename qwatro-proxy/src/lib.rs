pub mod tcp;

mod strategy;
mod udp;

use crate::strategy::ProxyStrategy;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::io;
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};
use tokio_util::sync::CancellationToken;

/// Запуск проксирования
/// * `ct`: `CancellationToken`, по завершении которого задача проксирования будет остановлена
/// * `strategy`: стратегия проксирования
/// * `ls_map`: map соответствий адреса прослушивания в приложении и проксируемого удаленного адреса
pub async fn run_proxy(
    ct: CancellationToken,
    strategy: impl ProxyStrategy,
    ls_map: HashMap<SocketAddr, SocketAddr>,
) -> io::Result<()> {
    strategy.run(ct, ls_map).await
}

// /// Соединение потоков данных (для проксирования)
// async fn join_streams<RW>(ct: CancellationToken, mut in_rw: RW, mut out_rw: RW)
// where
//     RW: AsyncRead + AsyncWrite + Unpin,
// {
//     let client_to_server = async {
//         tokio::select! {
//             res = io::copy(&mut in_rw, &mut out_rw) => res.map(|_| ())?,
//             _ = ct.cancelled() => {}
//         }
//         out_rw.shutdown().await
//     };
//
//     let server_to_client = async {
//         tokio::select! {
//             res = io::copy(&mut out_rw, &mut in_rw) => res.map(|_| ())?,
//             _ = ct.cancelled() => {}
//         }
//         in_rw.shutdown().await
//     };
//
//     if let Err(err) = tokio::try_join!(client_to_server, server_to_client) {
//         log::error!("{}", err);
//     };
// }

/// Соединение потоков данных (для проксирования)
async fn join_streams<R, W>(
    ct: CancellationToken,
    (mut in_read, mut in_write): (R, W),
    (mut out_read, mut out_write): (R, W),
) where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let client_to_server = async {
        tokio::select! {
            res = io::copy(&mut in_read, &mut out_write) => res.map(|_| ())?,
            _ = ct.cancelled() => {}
        }
        out_write.shutdown().await
    };

    let server_to_client = async {
        tokio::select! {
            res = io::copy(&mut out_read, &mut in_write) => res.map(|_| ())?,
            _ = ct.cancelled() => {}
        }
        in_write.shutdown().await
    };

    if let Err(err) = tokio::try_join!(client_to_server, server_to_client) {
        log::error!("{}", err);
    };
}
