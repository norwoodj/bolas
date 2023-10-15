use actix_web::{middleware::Logger, web, App, HttpServer};
use clap::Parser;
use libsystemd::{activation, activation::IsType};
use std::collections::HashMap;
use std::convert::TryInto;
use std::io;
use std::net::TcpListener;
use std::os::fd::{FromRawFd, IntoRawFd};
use std::os::unix::net::UnixListener;

mod bolas;
mod config;
mod static_files;
mod version;
mod websocket;

use self::config::{BolasArgs, BolasConfig};

fn get_systemd_listeners(
    systemd_names: Vec<String>,
) -> io::Result<(HashMap<String, TcpListener>, HashMap<String, UnixListener>)> {
    let file_descriptors_with_names = activation::receive_descriptors_with_names(true)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let inet_listeners = file_descriptors_with_names
        .iter()
        .filter(|(fd, name)| systemd_names.contains(name) && fd.is_inet())
        .map(|(fd, name)| unsafe {
            (
                name.clone(),
                TcpListener::from_raw_fd(fd.clone().into_raw_fd()),
            )
        })
        .collect();

    let unix_listeners = file_descriptors_with_names
        .into_iter()
        .filter(|(fd, name)| systemd_names.contains(name) && fd.is_unix())
        .map(|(fd, name)| unsafe { (name, UnixListener::from_raw_fd(fd.into_raw_fd())) })
        .collect();

    Ok((inet_listeners, unix_listeners))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let args = BolasArgs::parse();
    let version_info = version::VersionInfo::default();
    let bolas_config: BolasConfig = match (&args).try_into() {
        Ok(c) => c,
        Err(e) => {
            log::error!("Failed to convert arguments to runtime configuration {e:?}");
            return Err(e);
        }
    };

    let mut server = HttpServer::new(move || {
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

    for t in args.tcp_addrs {
        log::info!("Starting HTTP server on {}", t);
        server = server.bind(t)?;
    }

    for u in args.unix_addrs {
        log::info!("Starting HTTP server on {}", u);
        server = server.bind_uds(u)?;
    }

    if !args.systemd_names.is_empty() {
        let (tcp_listeners, unix_listeners) = get_systemd_listeners(args.systemd_names)?;

        for (name, t) in tcp_listeners {
            log::info!(
                "Starting HTTP server on inherited systemd tcp listener {}",
                name
            );
            server = server.listen(t)?;
        }

        for (name, u) in unix_listeners {
            log::info!("Starting HTTP server on {}", name);
            server = server.listen_uds(u)?;
        }
    }

    server.run().await
}
