use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Subcommand)]
pub enum SubCommand {
    #[clap(
        name = "install",
        long_about = "Arguments:
        This subcommand optionally accepts `<PATH>` which succeeds if formatting is correct and
        fails if it is not. For example:
            rustup_plus_plus install /home/toolchain.yaml"
    )]
    Install {
        #[arg(required = true)]
        path: Option<PathBuf>,
    },
    #[clap(
        name = "package",
        long_about = "Arguments:
    This subcommand optionally accepts `<PATH>` which succeeds if formatting is correct and
        fails if it is not. For example:
            rustup_plus_plus package /home/toolchain.yaml"
    )]
    Package {
        #[arg(required = true)]
        path: Option<PathBuf>,
    },
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
pub struct Options {
    #[command(subcommand)]
    pub subcommand: SubCommand,
}