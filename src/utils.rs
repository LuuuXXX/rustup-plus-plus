use std::{process::Command, error::Error};

use std::path::Path;
use std::path::PathBuf;

pub fn canonicalize_path(path: &Path) -> Option<PathBuf> {
    match std::fs::canonicalize(path) {
        Ok(path) => Some(path),
        Err(_) => None,
    }
}

pub fn run_rustup(args: &Vec<String>) {
    let command = "rustup".to_string();
    if let Err(err) = run_command(&command, &args) {
        println!("Failed to install toolchain, cause: {}", err);
    }
}

pub fn run_cargo(args: &Vec<String>) {
    let command = "cargo".to_string();
    if let Err(err) = run_command(&command, &args) {
        println!("Failed to install toolchain, cause: {}", err);
    }
}

fn run_command(cmd: &String, args: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let output = Command::new(cmd)
        .args(args)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output()?;

    if output.status.success() {
        println!("Execute {} command succeeded", cmd);
        Ok(())
    } else {
        eprintln!("Execute {} command failed", cmd);
        Err("Execute command failed".into())
    }
}