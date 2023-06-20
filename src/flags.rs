use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Subcommand)]
pub enum SubCommand {
    #[clap(
        name = "install",
        long_about = "Arguments:
        This subcommand optionally accepts `<PATH>` which succeeds if formatting is correct and
        fails if it is not. For example:
            rustup_plus_plus install -s /home/toolchain.yaml"
    )]
    Install {
        #[arg(short, long, required = true)]
        source_dir: Option<PathBuf>,
    },
    #[clap(
        name = "package",
        long_about = "Arguments:
    This subcommand optionally accepts `<PATH>` which succeeds if formatting is correct and
        fails if it is not. For example:
            rustup_plus_plus package -s /home/toolchain.yaml -o /home/package/"
    )]
    Package {
        #[arg(short, long, required = true)]
        source_dir: Option<PathBuf>,
        #[arg(short, long, required = true)]
        output_dir: Option<PathBuf>,
    },
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
pub struct Options {
    #[command(subcommand)]
    pub subcommand: SubCommand,
}