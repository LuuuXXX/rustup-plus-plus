use std::fs;
use std::{process::Command, error::Error};

use std::path::Path;
use std::path::PathBuf;

use anyhow::{Result, Context};
use url::Url;

pub fn ensure_dir_exists(name: &String, path: &PathBuf) -> Result<bool> {
    if !path.exists() {
        fs::create_dir_all(path)?;
        println!("Created directory '{}' at path '{:?}'", name, path);
        Ok(true)
    } else {
        println!("Directory '{}' already exists at path {:?}", name, path);
        Ok(false)
    }
}

#[cfg(target_os = "windows")]
fn normalize_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        if let std::path::Component::Normal(name) = component {
            normalized.push(name.to_string_lossy().replace("/", "\\"));
        } else {
            normalized.push(component.as_os_str());
        }
    }
    // Remove the \\?\ prefix from the normalized path
    if let Some(stripped_path) = normalized.to_str().map(|s| s.trim_start_matches("\\\\?\\")) {
        return PathBuf::from(stripped_path);
    }
    normalized
}

#[cfg(not(target_os = "windows"))]
fn normalize_path(path: &Path) -> PathBuf {
    path.components().collect()
}

pub fn canonicalize_path(path: &Path) -> Option<PathBuf> {
    match std::fs::canonicalize(path) {
        Ok(path) => Some(normalize_path(&path)),
        Err(_) => None,
    }
}

pub fn parse_url(url: &String) -> Result<Url> {
    Url::parse(url).with_context(|| format!("failed to parse url: {}", url))
}

pub fn run_rustup(args: &Vec<String>) {
    let command = "rustup".to_string();
    if let Err(err) = run_command(&command, &args) {
        println!("Failed to run rustup, cause: {}", err);
    }
}

pub fn run_cargo(args: &Vec<String>) {
    let command = "cargo".to_string();
    if let Err(err) = run_command(&command, &args) {
        println!("Failed to run cargo, cause: {}", err);
    }
}

pub fn run_tar(args: &Vec<String>) {
    println!("{:?}", &args);
    let command = "tar".to_string();
    if let Err(err) = run_command(&command, &args) {
        println!("Failed to package, cause: {}", err);
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