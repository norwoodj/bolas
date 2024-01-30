use actix_web::{middleware::Logger, web, App, HttpServer};
use foundations::cli::Cli;
use std::convert::TryInto;
use std::io;

mod bolas;
mod http;
mod metrics;
mod settings;
mod static_files;
mod version;
mod websocket;

use self::http::run_http_server;
use self::settings::{BolasConfig, BolasSettings, ServerListenerSettings};
use self::version::VersionInfo;

async fn run_application_server(
    server_listener_settings: &ServerListenerSettings,
    bolas_config: BolasConfig,
    version_info: VersionInfo,
) -> io::Result<()> {
    server_listener_settings.validate("application")?;

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
        "bolas application",
        &server_listener_settings.get_socket_addrs(),
        &server_listener_settings.unix_addrs,
        &server_listener_settings.systemd_names,
    )
    .await
}

async fn run_management_server(
    server_listener_settings: &ServerListenerSettings,
) -> io::Result<()> {
    server_listener_settings.validate("management")?;

    let app_server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/metrics", web::get().to(metrics::metrics_handler))
    });

    run_http_server(
        app_server,
        "bolas management",
        &server_listener_settings.get_socket_addrs(),
        &server_listener_settings.unix_addrs,
        &server_listener_settings.systemd_names,
    )
    .await
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let service_info = foundations::service_info!();
    let cli = Cli::<BolasSettings>::new(&service_info, Default::default())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let version_info = VersionInfo::default();
    let bolas_config: BolasConfig = match (&cli.settings).try_into() {
        Ok(c) => c,
        Err(e) => {
            log::error!("Failed to convert arguments to runtime configuration {e:?}");
            return Err(e);
        }
    };

    let application_server = run_application_server(
        &cli.settings.application_http_server,
        bolas_config,
        version_info,
    );

    let management_server = run_management_server(&cli.settings.management_http_server);

    futures::try_join!(application_server, management_server)?;
    Ok(())
}
