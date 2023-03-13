use crate::range::PortRange;
use std::collections::VecDeque;
use std::net::IpAddr;
use tokio::sync::{mpsc, oneshot};
use tokio_util::sync::CancellationToken;

pub fn run(
    ct: CancellationToken,
    mut task_queue_rx: mpsc::UnboundedReceiver<oneshot::Sender<Option<(IpAddr, u16)>>>,
    ip: IpAddr,
    port_range: PortRange,
) {
    tokio::spawn(async move {
        // Очередь задач сканирования на выполнение
        let mut task_queue = port_range
            .into_iter()
            .map(|p| (ip, p))
            .collect::<VecDeque<_>>();

        loop {
            tokio::select! {
                // В случае отмены CancellationToken, выходим из функции и закрываем канал очереди.
                // Воркеры увидят, что канал закрыт и завершат работу
                _ = ct.cancelled() => break,
                Some(tx) = task_queue_rx.recv() => tx.send(task_queue.pop_front()).unwrap()
            }
        }
    });
}
