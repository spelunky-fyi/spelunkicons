use std::convert::Infallible;
use std::net::SocketAddr;

use clap::Parser;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use spelunkicons::service::spelunkicon;

/// Server for generating spelunkicons
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// The port to listen on
    #[clap(short, long, default_value_t = 3000)]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(spelunkicon)) });
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
