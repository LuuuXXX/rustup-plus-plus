use std::env::{temp_dir, self};
use std::process::Command;

use std::fs::{write, remove_file};

use std::io::{Error, ErrorKind};

use crate::{execute_script, Config, TargetSelection};

pub fn run_install(config: &Config) -> Result<(), Error>{
    if let Some(rustup_dist_server) = &config.rustup_dist_server {
        env::set_var("RUSTUP_DIST_SERVER", rustup_dist_server);
    }
    if let Some(rustup_update_root) = &config.rustup_update_root {
        env::set_var("RUSTUP_UPDATE_ROOT", rustup_update_root);
    }

    let target_selections = &config.target_selections;

    for target_selection in target_selections.iter() {
        let result = install_toolchain(&target_selection);
    }

    Ok(())
}

fn install_toolchain(target_selection: &TargetSelection) -> Result<(), Error> {
    // let target = &target_selection.target;
    // let channel = &target_selection.channel;
    // let profile = &target_selection.profile;
    // let extra_tools = &target_selection.extra_tools;

    Ok(())
}

pub fn install_rust_toolchain_official() -> Result<(), Error>{
    let output = Command::new("curl")
        .args(&["--proto", "=https", "--tlsv1.2", "-sSf", "https://sh.rustup.rs"])
        .output()
        .expect("failed to install rustup-init.sh");

    // Check whether susscess to install
    if !output.status.success() {
        return Err(Error::new(ErrorKind::Other, "Failed to install rust official toolchain"));
    }

    // Save the script to the temporary directory
    let script_path = temp_dir().join("rustup-install.sh");
    write(&script_path, &output.stdout)?;

    // Add executable permission
    Command::new("chmod")
        .arg("+x")
        .arg(&script_path)
        .status()?;

    // Execute the installation script
    let _ = execute_install_script(&script_path);

    // Rename the temporary directory
    remove_file(script_path)?;

    Ok(())
}

fn execute_install_script(script_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    execute_script!(script_path, if cfg!(target_os = "windows") { "cmd.exe" } else { "sh" })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_install_rust_toolchain_official() {
        let result = install_rust_toolchain_official();

        match result {
            Ok(()) => {
                println!("Script executed successfully");
            }
            Err(err) => {
                println!("Script execution failed: {}", err);
            }
        }
    }
}