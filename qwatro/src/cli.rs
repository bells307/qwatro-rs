use clap::{Parser, Subcommand};
use qwatro_port_scanner::range::PortRange;
use std::net::IpAddr;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Port scan
    PS {
        /// Scanning IP-address
        #[arg(default_value = "127.0.0.1")]
        ip: IpAddr,
        /// Port range
        #[arg(short, long, default_value = "1-65535", value_parser = port_range_parser)]
        port_range: PortRange,
        /// Enable tcp scanning
        #[arg(long, group = "tcp-scan")]
        tcp: bool,
        /// TCP response timeout (ms)
        #[arg(long, default_value_t = 300, requires = "tcp-scan")]
        tcp_resp_timeout: u64,
        /// Maximum parallel scan tasks
        #[arg(long, default_value_t = 500)]
        max_tasks: usize,
    },
}

fn port_range_parser(s: &str) -> Result<PortRange, String> {
    let splitted = s.split('-').collect::<Vec<_>>();
    if splitted.len() == 2 {
        let min = splitted[0].parse::<u16>().map_err(|e| e.to_string())?;
        let max = splitted[1].parse::<u16>().map_err(|e| e.to_string())?;
        PortRange::ordered(min, max).map_err(|e| e.to_string())
    } else if splitted.len() == 1 {
        let port = splitted[0].parse::<u16>().map_err(|e| e.to_string())?;
        Ok(PortRange::specific(vec![port]))
    } else {
        Err("port range can't contain more than 2 values".into())
    }
}
