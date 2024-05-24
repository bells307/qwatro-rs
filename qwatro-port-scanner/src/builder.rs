use crate::range::PortRange;
use crate::scanner::PortScanner;
use std::net::{IpAddr, Ipv4Addr};
use std::num::NonZeroU16;
use std::time::Duration;

/// Builder сканера портов
///
/// # Example
/// ```
/// use qwatro_port_scanner::builder::PortScannerBuilder;
///
/// let scanner = PortScannerBuilder::new()
///     .ip("192.168.100.1".parse().unwrap())
///     .tcp(None)
///     .build();
/// ```
pub struct PortScannerBuilder {
    /// IP-адрес
    ip: IpAddr,
    /// Диапазон портов
    port_range: PortRange,
    /// Максимальное количество параллельно запущенных задач сканирования
    max_tasks: usize,
    /// Максимальное время ответа при ожидании на установление соединения
    resp_timeout: Option<Duration>,
}

impl Default for PortScannerBuilder {
    fn default() -> Self {
        Self {
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port_range: PortRange::ordered(1.try_into().unwrap(), NonZeroU16::MAX).unwrap(),
            max_tasks: 500,
            resp_timeout: None,
        }
    }
}

impl PortScannerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// IP-адрес сканируемого хоста
    pub fn ip(mut self, ip: IpAddr) -> Self {
        self.ip = ip;
        self
    }

    /// Диапазон портов
    pub fn port_range(mut self, port_range: PortRange) -> Self {
        self.port_range = port_range;
        self
    }

    /// Максимальное количество параллельно запущенных задач сканирования
    pub fn max_tasks(mut self, max_tasks: usize) -> Self {
        self.max_tasks = max_tasks;
        self
    }

    /// Максимальное количество параллельно запущенных задач сканирования
    pub fn resp_timeout(mut self, resp_timeout: Duration) -> Self {
        self.resp_timeout = Some(resp_timeout);
        self
    }

    /// Создать `PortScanner`
    pub fn build(self) -> PortScanner {
        PortScanner::new(self.ip, self.port_range, self.max_tasks, self.resp_timeout)
    }
}
