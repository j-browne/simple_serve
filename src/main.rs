use axum::Router;
use clap::Parser;
use std::{
    error::Error,
    future::IntoFuture,
    net::{IpAddr, Ipv6Addr, SocketAddr},
    path::PathBuf,
};
use tokio::task::JoinSet;
use tower_http::services::ServeDir;

/// Serve a directory.
#[derive(Parser, Debug)]
struct Args {
    /// Directory to serve.
    #[arg(default_value = ".")]
    path: PathBuf,
    /// Socket address to bind to.
    ///
    /// This option may be specified multiple times.
    ///
    /// If the port is 0, the OS will assign a port.
    ///
    /// If this option is not specified, it will default to
    /// listening on all ipv6 addresses, with an OS-supplied port.
    /// Depending on the OS, this may listen on all ipv4 addresses, as well.
    /// [default: [::]:0]
    #[arg(short, long = "address", id = "ADDR")]
    addrs: Vec<SocketAddr>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut args = Args::parse();
    let app = Router::new().fallback_service(ServeDir::new(args.path.clone()));

    // Set default addr
    if args.addrs.is_empty() {
        args.addrs
            .push(SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 0));
    }

    // Spawn all the tasks. If any finishes, return the error
    let mut join_set = JoinSet::new();
    for addr in args.addrs {
        let listener = tokio::net::TcpListener::bind(addr).await?;
        println!("Serving {:?} on {:?}", args.path, listener.local_addr()?);
        join_set.spawn(axum::serve(listener, app.clone()).into_future());
    }
    while let Some(res) = join_set.join_next().await {
        let _ = res?;
    }

    Ok(())
}
