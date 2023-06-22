use std::{io::Error, path::PathBuf, env};

use crate::{Config, DownloadCfg};

pub fn run_package(config: &Config, output_dir: &PathBuf) -> Result<(), Error>{
    env::set_var("RUSTUP_USE_CURL", "true");

    for target_selection in &config.target_selections {
        let root_url = target_selection.package_dir(&config.rustup_dist_server);

        let mut target_file_name = String::new();
        
        if ["nightly", "beta", "stable"].contains(&&*target_selection.channel) {
            target_file_name = format!(
                "rust-{}-{}.tar.gz",
                target_selection.channel, target_selection.target
            );
        }

        // TODO: Figure out how to handle different dist urls
        let dist_root = format!("{}/{}", root_url, target_file_name);

        let download_cfg = DownloadCfg {
            dist_root: dist_root,
            download_dir: output_dir.to_path_buf(),
        };

        if let Ok(file) = download_cfg.download(&target_file_name) {
            print!("{:?}", file.file_name());
        }
    }

    Ok(())
}