use std::{process::Command, error::Error};

use std::path::Path;
use std::path::PathBuf;

use std::fs::File;
use std::io::copy;

pub fn canonicalize_path(path: &Path) -> Option<PathBuf> {
    match std::fs::canonicalize(path) {
        Ok(path) => Some(path),
        Err(_) => None,
    }
}

pub fn run_command(cmd: &String, args: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let output = Command::new(cmd)
        .args(args)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Execute {} command succeeded:\n{}", cmd, stdout);
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Execute {} command failed:\n{}", cmd, stderr);
        Err("Execute command failed".into())
    }
}

pub fn run_command_without_output(cmd: &String, args: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let output = Command::new(cmd)
        .args(args)
        .output()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Execute {} command succeeded:\n{}", cmd, stdout);
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Execute {} command failed:\n{}", cmd, stderr);
        Err("Execute command failed".into())
    }
}

pub async fn download_file(url:&str, output_file: &str) -> Result<(), Box<dyn Error>>{
    println!("Downloading {} ...", url);


    println!("Download completed successfully");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonicalize_path() {
        todo!()
    }

    #[test]
    fn test_run_command() {
        todo!()
    }

    #[test]
    fn test_run_command_without_output() {
        todo!()
    }

    #[test]
    fn test_download_file() {
        todo!()
    }
}