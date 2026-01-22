use clap::Parser;

mod templates;
mod server;

/// Arguments for application initialization
#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, default_value_t = 5)]
    max_db_connections: u32,

    #[clap(short, long, default_value = "static")]
    static_dir: String,

    #[clap(short, long, default_value = "0.0.0.0")]
    bind_address: String,

    #[clap(short, long, default_value_t = 3000)]
    port: u32
}

#[tokio::main()]
async fn main() {
    let args = Args::parse();
    let addr = format!("{}:{}", args.bind_address, args.port);
    let listener = tokio::net::TcpListener::bind(addr)
        .await.expect("Failed to bind listener!");
    log::info!("Listening on port {}", args.port);

    let router = server::create_router(args).await;
    log::info!("Created router for requests");

    axum::serve(listener, router).await.unwrap();

}
