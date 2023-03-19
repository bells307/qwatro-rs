use clap::Subcommand;
use std::net::SocketAddr;

#[derive(Debug, Subcommand)]
pub enum ProxyArgs {
    /// TCP
    TCP {
        listen: SocketAddr,
        server: SocketAddr,
    },
    /// UDP
    UDP {
        listen: SocketAddr,
        server: SocketAddr,
    },
}
