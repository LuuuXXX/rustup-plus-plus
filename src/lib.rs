mod flags;
pub use crate::flags::*;

mod utils;
pub use crate::utils::*;

mod config;
pub use crate::config::*;

mod download;
pub use crate::download::*;

mod subcommands;
pub use crate::subcommands::*;

mod backend;
pub use crate::backend::*;

pub enum Backend {
    Curl,
    Reqwest(TlsBackend),
}

pub enum TlsBackend {
    Rustls,
    Default,
}