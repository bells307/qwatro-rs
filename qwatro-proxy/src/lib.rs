use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::{
    io,
    net::{TcpListener, TcpStream},
    time,
};
use tokio_util::sync::CancellationToken;

pub enum ProxyType {
    TCP,
    UDP,
}

pub struct ProxyTask {
    listen: SocketAddr,
    server: SocketAddr,
    ty: ProxyType,
}

async fn tcp_bind_listener(addr: SocketAddr, timeout: Duration) -> TcpListener {
    loop {
        match TcpListener::bind(addr).await {
            Ok(ln) => {
                println!("binded {addr} successfully");
                break ln;
            }
            Err(err) => {
                println!("error binding tcp listener for {}: {}", addr, err);
                time::sleep(timeout).await;
                continue;
            }
        };
    }
}

async fn tcp_connect(addr: SocketAddr, timeout: Duration) -> TcpStream {
    loop {
        match TcpStream::connect(addr).await {
            Ok(s) => {
                println!("connected to server {addr}");
                break s;
            }
            Err(err) => {
                println!("error connecting to {}: {}", addr, err);
                time::sleep(timeout).await;
                continue;
            }
        };
    }
}

pub fn start_proxy(ct: CancellationToken, tasks: Vec<ProxyTask>) {
    for task in tasks {
        tokio::spawn(async move {
            loop {
                let ln = tcp_bind_listener(task.listen, Duration::from_millis(1000)).await;
                let mut out_stream = tcp_connect(task.server, Duration::from_millis(1000)).await;
                while let Ok((mut in_stream, _)) = ln.accept().await {
                    let (mut ri, mut wi) = in_stream.split();
                    let (mut ro, mut wo) = out_stream.split();

                    let client_to_server = async {
                        io::copy(&mut ri, &mut wo).await?;
                        wo.shutdown().await
                    };

                    let server_to_client = async {
                        io::copy(&mut ro, &mut wi).await?;
                        wi.shutdown().await
                    };

                    if let Err(err) = tokio::try_join!(client_to_server, server_to_client) {
                        println!("{}", err);
                    };
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::{start_proxy, ProxyTask, ProxyType};
    use std::time::Duration;
    use tokio::time;
    use tokio_util::sync::CancellationToken;

    #[tokio::test]
    async fn it_works() {
        let tasks = vec![ProxyTask {
            listen: "127.0.0.1:9998".parse().unwrap(),
            server: "127.0.0.1:9999".parse().unwrap(),
            ty: ProxyType::TCP,
        }];

        let ct = CancellationToken::new();
        start_proxy(ct.clone(), tasks);
        ct.cancelled().await;
    }
}
