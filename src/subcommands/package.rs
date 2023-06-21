use std::{io::Error, path::PathBuf};

use crate::{Config, TargetSelection};

pub static DEFAULT_DIST_ROOT: &str = "https://static.rust-lang.org/dist";

pub fn run_package(config: &Config, output_dir: &PathBuf) -> Result<(), Error>{
    todo!()
}