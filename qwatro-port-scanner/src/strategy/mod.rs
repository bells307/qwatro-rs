pub mod tcp;

use crate::{strategy::tcp::TcpScanning, ScanType};
use async_trait::async_trait;
use std::{net::SocketAddr, time::Duration};

/// Стратегия сканирования адреса
#[async_trait]
pub trait ScanStrategy: Send + Sync {
    /// Просканировать адрес
    async fn scan(&self, addr: SocketAddr) -> bool;
    /// Тип сканирования
    fn scan_type(&self) -> ScanType;
}

pub(crate) fn strategies(resp_timeout: Option<Duration>) -> [Box<dyn ScanStrategy>; 1] {
    [Box::new(TcpScanning::new(resp_timeout))]
}
