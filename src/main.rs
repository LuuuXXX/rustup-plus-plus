use clap::Parser;
use rustup_plus_plus::{Options, Config, canonicalize_path, install::run_install, package::run_package};

fn main() {
    let opts = Options::parse();
    
    match opts.subcommand {
        rustup_plus_plus::SubCommand::Install { source_dir } => {
            if let Some(source_dir) = &source_dir {
                let source_dir = canonicalize_path(source_dir).unwrap_or_else(|| {
                    println!("Invalid or non-existent source directory: {:?}", source_dir);
                    std::process::exit(1);
                });
                let config = Config::parse(&source_dir);
                
                if let Err(err) = run_install(&config) {
                    panic!("Failed to install configuration toolchain : {:?}", err);
                }
            } else {
                println!("Source directory not provided");
                std::process::exit(1);
            }
        },
        rustup_plus_plus::SubCommand::Package { source_dir, output_dir } => {
            match (source_dir, output_dir) {
                (Some(source_dir), Some(output_dir)) => {
                    let config = Config::parse(&source_dir);

                    // print!("{:?}", config);
                    if let Err(err) = run_package(&config, &output_dir) {
                        panic!("Failed to package configuration toolchain : {:?}", err);
                    }
                },
                (None, Some(_)) => todo!(),
                (Some(_), None) => todo!(),
                (None, None) => todo!(),
            }
        },

    }
}