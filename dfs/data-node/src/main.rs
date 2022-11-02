use axum::routing::get;
use axum::Router;
use clap::Parser;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Parser, Debug)]
struct Args {
    /// Port
    #[arg(short, long, default_value_t = 40000)]
    port: u16,

    /// Volume of disk space to use in KB. If not set service will use all disk space
    #[arg(short, long)]
    disk_usage: Option<u64>,
}

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    axum::Server::bind(&SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        args.port,
    ))
    .serve(app.into_make_service())
    .await
    .unwrap()
}
