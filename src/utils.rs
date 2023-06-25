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

pub trait Runner {
    fn run_command(&self, args: &[String]) -> Result<(), Box<dyn Error>>;
}

pub enum CommandRunner {
    Rustup,
    Cargo,
    Tar,
}

impl Runner for CommandRunner {
    fn run_command(&self, args: &[String]) -> Result<(), Box<dyn Error>> {
        let (command, label) = match self {
            CommandRunner::Rustup => ("rustup", "rustup"),
            CommandRunner::Cargo => ("cargo", "cargo"),
            CommandRunner::Tar => ("tar", "package"),
        };

        let output = Command::new(command)
        .args(args)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output()?;

        if output.status.success() {
            println!("Execute {} command succeeded", label);
            Ok(())
        } else {
            eprintln!("Execute {} command failed", label);
            Err("Execute command failed".into())
        }
    }
}