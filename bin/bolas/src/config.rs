use clap::Parser;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io;
use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct BolasArgs {
    /// Interval in milliseconds at which the bolas state is
    /// updated and sent to the websocket client
    #[arg(default_value = "32", env, short, long)]
    pub(crate) bolas_refresh_rate_ms: u64,

    /// Path to folder containing static files to be served
    #[arg(env, short, long)]
    pub(crate) static_file_path: PathBuf,

    /// List of Systemd file descriptor names to listen on
    /// for the application server
    #[arg(env, long)]
    pub(crate) systemd_names: Vec<String>,

    /// List of TCP addresses to listen on for the application server
    #[arg(env, long)]
    pub(crate) tcp_addrs: Vec<SocketAddr>,

    /// List of Unix socket addresses to listen on for the application
    /// server
    #[arg(env, long)]
    pub(crate) unix_addrs: Vec<String>,

    /// List of Systemd file descriptor names to listen on
    /// for the management server
    #[arg(env, long)]
    pub(crate) management_systemd_names: Vec<String>,

    /// List of TCP addresses to listen on for the management server
    #[arg(env, long)]
    pub(crate) management_tcp_addrs: Vec<SocketAddr>,

    /// List of Unix socket addresses to listen on for the management
    /// server
    #[arg(env, long)]
    pub(crate) management_unix_addrs: Vec<String>,
}

#[derive(Clone)]
pub(crate) struct BolasConfig {
    /// Interval in milliseconds at which the bolas state is
    /// updated and sent to the websocket client
    pub(crate) bolas_refresh_rate_ms: u64,

    /// Path to folder containing static files to be served
    pub(crate) static_file_path: PathBuf,

    /// Factor by which the velocity users release balls with
    /// (length of pull line in pixels) is divided to get the
    /// actual velocity used
    pub(crate) velocity_scaling_factor: i32,
}

impl TryFrom<&BolasArgs> for BolasConfig {
    type Error = io::Error;

    fn try_from(args: &BolasArgs) -> Result<Self, Self::Error> {
        // I've found that this is a good factor to divide the velocity
        // at which balls are released (number of pixels the user dragged
        // before releasing) to make the experience look reasonable
        let velocity_scaling_factor = args
            .bolas_refresh_rate_ms
            .try_into()
            .map(|r: i32| 256 / r)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(Self {
            bolas_refresh_rate_ms: args.bolas_refresh_rate_ms,
            static_file_path: args.static_file_path.clone(),
            velocity_scaling_factor,
        })
    }
}
