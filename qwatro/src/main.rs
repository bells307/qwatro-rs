mod cli;

use crate::cli::AppArgs;
use clap::Parser;
use futures::StreamExt;
use qwatro_port_scanner::builder::PortScannerBuilder;
use std::env;
use tokio::signal;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    let args = AppArgs::parse();

    // Глобальный `CancellationToken`, который будет передаваться в компоненты приложения
    let ct = CancellationToken::new();
    tokio::spawn(shutdown(ct.clone()));

    scan(ct, args).await
}

/// Запуск сканирования портов
async fn scan(ct: CancellationToken, args: AppArgs) {
    let builder = PortScannerBuilder::new()
        .ip(args.ip)
        .port_range(args.port_range)
        .max_tasks(args.max_tasks);

    let scanner = builder.build();

    // Запускаем сканер
    let mut stream = scanner.run(ct);

    // Выводим элементы потока результата сканирования в stdout
    while let Some(res) = stream.next().await {
        println!("{}/{:#?}", res.addr, res.ty);
    }
}

/// Future, которая будет ожидать сигнала завершения приложения, после чего завершать `CancellationToken`
async fn shutdown(ct: CancellationToken) {
    signal::ctrl_c().await.unwrap();
    log::info!("got shutdown signal");
    ct.cancel();
}
