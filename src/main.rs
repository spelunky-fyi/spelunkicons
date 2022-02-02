use std::net::SocketAddr;
use std::sync::Arc;

use clap::Parser;
use hyper::Server;
use spelunkicons::generator::Generator;
use spelunkicons::service::MakeIconService;

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

    let generator = Arc::new(Generator::new());
    let server = Server::bind(&addr).serve(MakeIconService { generator });

    println!("Service is now ready...");

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
