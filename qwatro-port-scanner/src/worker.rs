use crate::{ScanResult, ScanType};
use std::net::{IpAddr, SocketAddr};
use tokio::net::TcpStream;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::oneshot;

pub fn spawn(
    task_queue_tx: UnboundedSender<oneshot::Sender<Option<(IpAddr, u16)>>>,
    scan_res_tx: UnboundedSender<ScanResult>,
) {
    tokio::spawn(async move {
        loop {
            let (tx, rx) = oneshot::channel();
            if task_queue_tx.send(tx).is_err() {
                // Канал очереди задач закрылся - скорее всего, был завершен предварительно.
                // В таком случае, воркеру делать больше нечего, завершаем работу
                break;
            };

            match rx.await.unwrap() {
                Some((ip, port)) => {
                    if TcpStream::connect(SocketAddr::new(ip, port)).await.is_ok() {
                        scan_res_tx
                            .send(ScanResult {
                                ip,
                                port,
                                ty: ScanType::TCP,
                            })
                            .unwrap();
                    };
                }
                None => {
                    // В очереди кончились задачи, завершаем работу
                    break;
                }
            };
        }
    });
}
