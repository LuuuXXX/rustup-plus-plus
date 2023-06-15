use std::path::Path;
use std::path::PathBuf;

mod flags;
pub use crate::flags::*;

mod utils;
pub use crate::utils::*;

mod config;
pub use crate::config::*;

mod subcommands;
pub use crate::subcommands::*;

pub fn canonicalize_path(path: &Path) -> Option<PathBuf> {
    match std::fs::canonicalize(path) {
        Ok(path) => Some(path),
        Err(_) => None,
    }
}