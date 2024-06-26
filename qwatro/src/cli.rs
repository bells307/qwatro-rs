use clap::Parser;
use qwatro_port_scanner::range::PortRange;
use std::{net::IpAddr, num::NonZeroU16};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct AppArgs {
    /// Scanning IP-address
    #[arg(default_value = "127.0.0.1")]
    pub ip: IpAddr,
    /// Port range
    #[arg(short, long, default_value = "1:65535", value_parser = port_range_parser)]
    pub port_range: PortRange,
    /// TCP response timeout (ms)
    #[arg(long, default_value_t = 300, requires = "tcp-scan")]
    pub resp_timeout: u64,
    /// Maximum parallel scan tasks
    #[arg(long, default_value_t = 500)]
    pub max_tasks: usize,
}

fn port_range_parser(s: &str) -> Result<PortRange, String> {
    let splitted = s.split(':').collect::<Vec<_>>();
    if splitted.len() == 2 {
        let min = splitted[0]
            .parse::<NonZeroU16>()
            .map_err(|e| e.to_string())?;

        let max = splitted[1]
            .parse::<NonZeroU16>()
            .map_err(|e| e.to_string())?;

        PortRange::ordered(min, max).map_err(|e| e.to_string())
    } else if splitted.len() == 1 {
        let port = splitted[0]
            .parse::<NonZeroU16>()
            .map_err(|e| e.to_string())?;

        Ok(PortRange::specific(vec![port]).map_err(|e| e.to_string())?)
    } else {
        Err("port range can't contain more than 2 values".into())
    }
}
