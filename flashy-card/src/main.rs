use clap::Parser;
use crate::server::Server;

mod templates;
mod server;

/// Arguments for application initialization
#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, default_value_t = 5)]
    max_db_connections: u32,

    #[clap(short, long, default_value = "static")]
    static_dir: String,

    #[clap(short, long, default_value_t = 3000)]
    port: u32
}

#[tokio::main()]
async fn main() {

    let args = Args::parse();
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", args.port))
        .await.expect("Failed to bind listener!");

    log::info!("Listening on port {}", args.port);
    let server = Server::new(args).await;
    server.serve(listener).await;

}
