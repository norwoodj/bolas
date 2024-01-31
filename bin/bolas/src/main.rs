use actix_web::{middleware::Logger, web, App, HttpServer};
use foundations::cli::Cli;
use foundations::telemetry::init_with_server;
use foundations::telemetry::log;
use foundations::telemetry::settings::TelemetrySettings;
use foundations::ServiceInfo;
use std::convert::TryInto;
use std::io;
use tokio::select;
use tokio::signal::unix::{signal, SignalKind};

mod bolas;
mod http;
mod metrics;
mod settings;
mod static_files;
mod utils;
mod version;
mod websocket;

use self::http::run_http_server;
use self::settings::{BolasConfig, BolasSettings, ServerListenerSettings};
use self::utils::bootstrap_to_io_error;
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
    telemetry_settings: &TelemetrySettings,
    service_info: &ServiceInfo,
) -> io::Result<()> {
    let mut int_signal_receiver = signal(SignalKind::interrupt())?;
    let mut term_signal_receiver = signal(SignalKind::terminate())?;
    let telemetry_server_fut =
        init_with_server(service_info, telemetry_settings, Default::default())
            .map_err(bootstrap_to_io_error)?;

    log::info!(
        "Starting telemetry server";
        "address" => %telemetry_settings.server.addr
    );

    select! {
        _ = int_signal_receiver.recv() => {
            log::info!("Shutting down telemetry server on signal"; "signal" => "SIGINT");
            Ok(())
        },
        _ = term_signal_receiver.recv() => {
            log::info!("Shutting down telemetry server on signal"; "signal" => "SIGTERM");
            Ok(())
        }
        r = telemetry_server_fut => r.map_err(bootstrap_to_io_error),
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let mut service_info = foundations::service_info!();
    let version_info = VersionInfo::default();

    // Vergen generates a version using `git describe` which is more detailed
    service_info.version = version_info.version;

    let cli = Cli::<BolasSettings>::new(&service_info, Default::default())
        .map_err(bootstrap_to_io_error)?;

    let bolas_config: BolasConfig = match (&cli.settings).try_into() {
        Ok(c) => c,
        Err(e) => {
            log::error!("Failed to convert arguments to runtime configuration"; "error" => ?e);
            return Err(e);
        }
    };

    let application_server = run_application_server(
        &cli.settings.application_http_server,
        bolas_config,
        version_info,
    );

    let management_server = run_management_server(&cli.settings.telemetry, &service_info);

    futures::try_join!(application_server, management_server)?;
    Ok(())
}
