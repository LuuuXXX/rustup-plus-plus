use clap::Parser;
use rustup_distribution::{Options, Config, canonicalize_path, install::run_install, package::run_package};

fn main() {
    let opts = Options::parse();
    
    match opts.subcommand {
        rustup_distribution::SubCommand::Install { source_dir } => {
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
        rustup_distribution::SubCommand::Package { source_dir, output_dir } => {
            match (source_dir, output_dir) {
                (Some(source_dir), Some(output_dir)) => {
                    let source_dir = canonicalize_path(&source_dir).unwrap_or_else(|| {
                        println!("Invalid or non-existent source directory: {:?}", source_dir);
                        std::process::exit(1);
                    });
                    
                    let output_dir = canonicalize_path(&output_dir).unwrap_or_else(|| {
                        println!("Invalid or non-existent output directory: {:?}", output_dir);
                        std::process::exit(1);
                    });

                    let config = Config::parse(&source_dir);

                    if let Err(err) = run_package(&config, &output_dir) {
                        panic!("Failed to package configuration toolchain : {:?}", err);
                    }
                },
                _ => panic!("Eorror: source directory or output directory not provided")
            }
        },

    }
}