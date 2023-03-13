mod cli;

use crate::cli::{Cli, Commands};
use clap::Parser;
use futures::StreamExt;
use qwatro_port_scanner::builder::PortScannerBuilder;
use qwatro_port_scanner::range::PortRange;
use std::net::IpAddr;
use std::time::Duration;
use tokio::signal;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::PS {
            ip,
            port_range,
            tcp,
            tcp_resp_timeout,
            max_tasks,
        } => scan(ip, port_range, tcp, tcp_resp_timeout, max_tasks).await,
    };
}

async fn scan(
    ip: IpAddr,
    port_range: PortRange,
    tcp: bool,
    tcp_resp_timeout: u64,
    max_tasks: usize,
) {
    let mut builder = PortScannerBuilder::new()
        .ip(ip)
        .port_range(port_range)
        .max_tasks(max_tasks);

    if tcp {
        builder = builder.tcp(Some(Duration::from_millis(tcp_resp_timeout)));
    }

    let scanner = builder.build();

    let ct = CancellationToken::new();
    tokio::spawn(shutdown(ct.clone()));

    let mut stream = scanner.run(ct);

    while let Some(res) = stream.next().await {
        println!("{}/{:#?}", res.addr, res.ty);
    }
}

async fn shutdown(ct: CancellationToken) {
    signal::ctrl_c().await.unwrap();
    println!("got ctrl + c signal");
    ct.cancel();
}
