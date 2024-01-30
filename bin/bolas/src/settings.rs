use foundations::settings::net::SocketAddr;
use foundations::settings::settings;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io;
use std::path::PathBuf;

#[settings]
pub(crate) struct BolasSettings {
    /// Interval in milliseconds at which the bolas state is
    /// updated and sent to the websocket client
    #[serde(default = "default_bolas_refresh_rate_ms")]
    pub(crate) bolas_refresh_rate_ms: u64,

    /// Path to folder containing static files to be served
    pub(crate) static_file_path: PathBuf,

    /// Listener configuration for the application http server
    pub(crate) application_http_server: ServerListenerSettings,

    /// Listener configuration for the management http server
    pub(crate) management_http_server: ServerListenerSettings,
}

#[settings]
pub(crate) struct ServerListenerSettings {
    /// List of Systemd file descriptor names to listen on
    pub(crate) systemd_names: Vec<String>,

    /// List of socket addresses to listen on
    pub(crate) socket_addrs: Vec<SocketAddr>,

    /// List of Unix socket addresses to listen on
    pub(crate) unix_addrs: Vec<String>,
}

impl ServerListenerSettings {
    pub(crate) fn validate(&self, server_name: &str) -> io::Result<()> {
        if self.systemd_names.len() + self.socket_addrs.len() + self.unix_addrs.len() == 0 {
            return Err(io::Error::new(
                io::ErrorKind::AddrNotAvailable,
                format!("No addresses provided to listen on for {server_name} server"),
            ));
        }

        Ok(())
    }

    pub(crate) fn get_socket_addrs(&self) -> Vec<std::net::SocketAddr> {
        self.socket_addrs.iter().copied().map(Into::into).collect()
    }
}

fn default_bolas_refresh_rate_ms() -> u64 {
    32
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

impl TryFrom<&BolasSettings> for BolasConfig {
    type Error = io::Error;

    fn try_from(args: &BolasSettings) -> Result<Self, Self::Error> {
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
