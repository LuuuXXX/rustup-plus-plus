use std::{io::Error, path::PathBuf};

use crate::{Config, TargetSelection};

pub static DEFAULT_DIST_ROOT: &str = "https://static.rust-lang.org/dist";

pub fn run_package(config: &Config, output_dir: &PathBuf) -> Result<(), Error>{
    let binding = String::from(DEFAULT_DIST_ROOT);
    let dist_root = match &config.rustup_dist_server {
        Some(rustup_dist_server) => {
            rustup_dist_server
        },
        None => &binding,
    };

    for target_selction in config.target_selections.iter() {

    }

    Ok(())
}

fn download() -> Result<(), Error> {
    
    Ok(())
}