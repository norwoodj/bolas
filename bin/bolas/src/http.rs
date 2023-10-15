use actix_http::{Request, Response};
use actix_service::ServiceFactory;
use actix_web::{body::MessageBody, dev::AppConfig, HttpServer};
use libsystemd::{activation, activation::IsType};
use std::collections::HashMap;
use std::fmt::Debug;
use std::io;
use std::net::{SocketAddr, TcpListener};
use std::os::fd::{FromRawFd, IntoRawFd};
use std::os::unix::net::UnixListener;

fn get_systemd_listeners(
    systemd_names: &[String],
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

pub(crate) async fn run_http_server<F, I, S, B>(
    mut server: HttpServer<F, I, S, B>,
    server_name: &'static str,
    tcp_addrs: &[SocketAddr],
    unix_addrs: &[String],
    systemd_names: &[String],
) -> io::Result<()>
where
    F: Fn() -> I + Send + Clone + 'static,
    I: actix_service::IntoServiceFactory<S, Request>,
    S: ServiceFactory<Request, Config = AppConfig, Error = actix_web::Error> + 'static,
    <S as ServiceFactory<Request>>::Error: 'static,
    S::InitError: Debug,
    S::Response: Into<Response<B>>,
    B: MessageBody + 'static,
{
    for t in tcp_addrs {
        log::info!("Starting {server_name} server on tcp listener {}", t);
        server = server.bind(t)?;
    }

    for u in unix_addrs {
        log::info!("Starting {server_name} server on unix listener {}", u);
        server = server.bind_uds(u)?;
    }

    if !systemd_names.is_empty() {
        let (tcp_listeners, unix_listeners) = get_systemd_listeners(systemd_names)?;

        for (name, t) in tcp_listeners {
            log::info!(
                "Starting {server_name} server on inherited systemd tcp listener {}",
                name
            );
            server = server.listen(t)?;
        }

        for (name, u) in unix_listeners {
            log::info!(
                "Starting {server_name} server on inherited systemd unix listener {}",
                name
            );
            server = server.listen_uds(u)?;
        }
    }

    server.run().await
}
