use clap::Parser;
use rustup_plus_plus::{Options, Config, canonicalize_path};

fn main() {
    let opts = Options::parse();
    
    match opts.subcommand {
        rustup_plus_plus::SubCommand::Install { path } => {
            if let Some(source_dir) = &path {
                let source_dir = canonicalize_path(source_dir).unwrap_or_else(|| {
                    println!("Invalid or non-existent source directory: {:?}", source_dir);
                    std::process::exit(1);
                });
                let config = Config::parse(&source_dir);
                print!("{:?}", config);
            } else {
                println!("Source directory not provided");
                std::process::exit(1);
            }
        },
        rustup_plus_plus::SubCommand::Package { path: _ } => todo!(),
    }
}