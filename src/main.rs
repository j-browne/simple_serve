use axum::Router;
use clap::Parser;
use std::{
    error::Error,
    net::{IpAddr, SocketAddr},
    path::PathBuf,
};
use tower_http::services::ServeDir;

/// Serve a directory.
#[derive(Parser, Debug)]
struct Args {
    /// Directory to serve.
    #[arg(default_value = ".")]
    path: PathBuf,
    /// IP address to bind to.
    #[arg(short, long, default_value = "::")]
    address: IpAddr,
    /// Port to bind to. If 0, the OS will assign a port.
    #[arg(short, long, default_value_t = 0)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let app = Router::new().fallback_service(ServeDir::new(args.path.clone()));
    let addr = SocketAddr::new(args.address, args.port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Serving {:?} on {:?}", args.path, listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
