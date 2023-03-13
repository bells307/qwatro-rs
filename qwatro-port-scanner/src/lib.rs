/// Ошибки сканера портов
pub mod error;
/// Диапазон сканирования портов
pub mod range;
/// Реализация сканирования портов
pub mod scanner;

mod task_queue;
mod worker;

use std::net::IpAddr;

/// Тип сканирования
#[derive(Debug)]
pub enum ScanType {
    TCP,
    UDP,
}

/// Результат сканирования
#[derive(Debug)]
pub struct ScanResult {
    pub ip: IpAddr,
    pub port: u16,
    pub ty: ScanType,
}
