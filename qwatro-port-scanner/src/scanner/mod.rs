/// Очередь задач на сканирование
mod task_queue;
/// Воркер, выполняющий задачи сканирования
mod worker;

use crate::range::PortRange;
use crate::strategy::{self, ScanStrategy};
use crate::ScanResult;
use futures::stream::BoxStream;
use futures::StreamExt;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_util::sync::CancellationToken;

/// Сканер портов
pub struct PortScanner {
    /// IP-адрес
    ip: IpAddr,
    /// Диапазон портов
    port_range: PortRange,
    /// Максимальное количество параллельно запущенных задач сканирования
    max_tasks: usize,
    /// Стратегии сканирования
    strategies: Arc<[Box<dyn ScanStrategy>]>,
}

impl PortScanner {
    pub fn new(
        ip: IpAddr,
        port_range: PortRange,
        max_tasks: usize,
        resp_timeout: Option<Duration>,
    ) -> Self {
        Self {
            ip,
            port_range,
            max_tasks,
            strategies: Arc::new(strategy::strategies(resp_timeout)),
        }
    }

    /// Запуск сканирования портов. Возвращает `Stream` успешных результатов сканирования
    /// * `ct`: `CancellationToken`, при отмене которого сканирование будет остановлено
    pub fn run<'a>(self, ct: CancellationToken) -> BoxStream<'a, ScanResult> {
        log::debug!(
            "start port scanning on {}, port range: {}",
            self.ip,
            self.port_range
        );

        let (task_queue_tx, task_queue_rx) = mpsc::unbounded_channel();
        task_queue::run(ct, task_queue_rx, self.ip, self.port_range);

        let (scan_res_tx, scan_res_rx) = mpsc::unbounded_channel();

        (0..self.max_tasks).for_each(|_| {
            worker::spawn(
                task_queue_tx.clone(),
                scan_res_tx.clone(),
                Arc::clone(&self.strategies),
            );
        });

        UnboundedReceiverStream::new(scan_res_rx).boxed()
    }
}
