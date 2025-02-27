//! Get/Set system proxy. Supports Windows, macOS and linux (via gsettings).

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

// #[cfg(feature = "utils")]
pub mod utils;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ProxyType {
    HTTP,
    HTTPS,
    SOCKS,
}

impl ProxyType {
    fn to_target(&self) -> &'static str {
        match self {
            ProxyType::HTTP => "webproxy",
            ProxyType::HTTPS => "securewebproxy",
            ProxyType::SOCKS => "socksfirewallproxy",
        }
    }
}

impl Default for ProxyType {
    fn default() -> Self {
        ProxyType::HTTP
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Sysproxy {
    pub enable: bool,
    pub host: String,
    pub port: u16,
    pub bypass: String,
    pub proxy_type: ProxyType,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to parse string `{0}`")]
    ParseStr(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("failed to get default network interface")]
    NetworkInterface,

    #[cfg(target_os = "windows")]
    #[error("system call failed")]
    SystemCallFailed(#[from] windows::SystemCallFailed),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Sysproxy {
    pub fn is_support() -> bool {
        cfg!(any(
            target_os = "linux",
            target_os = "macos",
            target_os = "windows",
        ))
    }
}
