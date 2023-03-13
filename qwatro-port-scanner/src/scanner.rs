use crate::range::PortRange;
use crate::{task_queue, worker, ScanResult};
use futures::stream::BoxStream;
use futures::StreamExt;
use std::net::IpAddr;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_util::sync::CancellationToken;

/// Сканер портов
pub struct PortScanner {
    /// IP-адрес
    ip: IpAddr,
    /// Диапазон партов
    port_range: PortRange,
    /// Максимальное количество параллельно запущенных задач сканирования
    max_tasks: usize,
}

impl PortScanner {
    pub fn new(ip: IpAddr, port_range: PortRange, max_tasks: usize) -> Self {
        Self {
            ip,
            port_range,
            max_tasks,
        }
    }

    pub fn run<'a>(self, ct: CancellationToken) -> BoxStream<'a, ScanResult> {
        let (task_queue_tx, task_queue_rx) = mpsc::unbounded_channel();
        task_queue::run(ct, task_queue_rx, self.ip, self.port_range);

        let (scan_res_tx, scan_res_rx) = mpsc::unbounded_channel();

        let stream = UnboundedReceiverStream::new(scan_res_rx);

        for _ in 0..self.max_tasks {
            worker::spawn(task_queue_tx.clone(), scan_res_tx.clone());
        }

        stream.boxed()
    }
}

#[cfg(test)]
mod tests {
    use crate::range::PortRange;
    use crate::scanner::PortScanner;
    use tokio_stream::StreamExt;
    use tokio_util::sync::CancellationToken;

    #[tokio::test]
    async fn mytest() {
        let s = PortScanner::new(
            "192.168.65.82".parse().unwrap(),
            PortRange::ordered(8000, 10000).unwrap(),
            500,
        );

        let mut stream = s.run(CancellationToken::new());
        while let Some(v) = stream.next().await {
            println!("{:#?}", v);
        }
    }
}
