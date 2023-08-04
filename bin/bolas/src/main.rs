use actix_web::{middleware::Logger, web, App, HttpServer};
use clap::Parser;
use std::io;
use std::net::SocketAddr;
use std::path::PathBuf;

mod bolas;
mod static_files;
mod websocket;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct BolasArgs {
    /// Path to folder containing static files to be served
    #[arg(short, long)]
    static_file_path: PathBuf,

    /// List of TCP addresses to listen for http on
    #[arg(short, long)]
    tcp_addrs: Vec<SocketAddr>,

    /// List of Unix socket addresses to listen for http on
    #[arg(short, long)]
    unix_addrs: Vec<String>,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let args = BolasArgs::parse();
    let static_file_path = args.static_file_path.clone();

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(static_file_path.clone()))
            .wrap(Logger::default())
            .route("/ws", web::get().to(websocket::serve_websockets))
            .route("/", web::get().to(static_files::serve_index_html))
            .route(
                "/{filename}",
                web::get().to(static_files::serve_static_file),
            )
    });

    for t in args.tcp_addrs {
        log::info!("Starting HTTP server on {}", t);
        server = server.bind(t)?;
    }

    for u in args.unix_addrs {
        log::info!("Starting HTTP server on {}", u);
        server = server.bind_uds(u)?;
    }

    server.run().await
}
