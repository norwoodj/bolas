use actix_web::{middleware::Logger, web, App, HttpServer};
use clap::Parser;
use std::convert::TryInto;
use std::io;

mod bolas;
mod config;
mod http;
mod static_files;
mod version;
mod websocket;

use self::config::{BolasArgs, BolasConfig};
use self::http::run_http_server;
use self::version::VersionInfo;

async fn run_application_server(
    bolas_args: BolasArgs,
    bolas_config: BolasConfig,
    version_info: VersionInfo,
) -> io::Result<()> {
    let app_server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(bolas_config.clone()))
            .app_data(web::Data::new(version_info.clone()))
            .wrap(Logger::default())
            .route("/ws", web::get().to(websocket::serve_websockets))
            .route("/", web::get().to(static_files::serve_index_html))
            .route(
                "/server-version.json",
                web::get().to(version::serve_version_info),
            )
            .route(
                "/{filename}",
                web::get().to(static_files::serve_static_file),
            )
    });

    run_http_server(
        app_server,
        &bolas_args.tcp_addrs,
        &bolas_args.unix_addrs,
        &bolas_args.systemd_names,
    )
    .await
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let bolas_args = BolasArgs::parse();
    let version_info = VersionInfo::default();
    let bolas_config: BolasConfig = match (&bolas_args).try_into() {
        Ok(c) => c,
        Err(e) => {
            log::error!("Failed to convert arguments to runtime configuration {e:?}");
            return Err(e);
        }
    };

    run_application_server(bolas_args, bolas_config, version_info).await
}
