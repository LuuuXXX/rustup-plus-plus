use std::error::Error;
use std::path::{PathBuf, Path};
use std::io::{Write, self};
use std::collections::VecDeque;
use std::env;
use std::fs::{self, File};

use anyhow::{Result};

use crate::{Config, DownloadCfg, ExtendTool, CommandRunner, Runner, utils, TargetSelection};

pub fn run_package(config: &Config, output_dir: &PathBuf) -> Result<(), Box<dyn Error>>{
    env::set_var("RUSTUP_USE_CURL", "true");

    // FIXME: this only support one target, but we want to support multiple targets
    // ADDITIONAL: CARGO only supports one target to install: cargo install --root XXX
    for target_selection in &config.target_selections {
        // parse to tagert package download path
        let root_url = target_selection.package_dir(&config.rustup_dist_server);

        let target_file_name = get_package_full_name(&target_selection);

        // TODO: Figure out how to handle different dist urls
        let dist_root = format!("{}/{}", root_url, target_file_name);

        let download_cfg = DownloadCfg {
            dist_root: dist_root,
            download_dir: output_dir.to_path_buf(),
        };

        if let Ok(file) = download_cfg.extract_after_download(&target_file_name) {
            // move file to destination
            for extra_tool in &config.extra_tools {
                let tool_path =  package_extra_tools(extra_tool, output_dir);
                if let Err(err) = move_folder(&tool_path, &file.path) {
                    panic!("Couldn't move file: {}", err);
                };
                update_components(&file, &get_tool_folder_name(extra_tool))?;
            }

            try_package(&target_file_name, &output_dir, &file).unwrap();
        }
    }

    Ok(())
}

fn try_package(target_file_name: &String, output_dir: &PathBuf, file: &Path) -> Result<()> {
    let targe_file_path = output_dir.join(target_file_name);
    
    println!("{}", targe_file_path.to_owned().to_string_lossy().to_string());
    println!("{}", file.to_owned().to_string_lossy().to_string());

    let mut args = Vec::new();
    args.push("-czvf".to_string());
    args.push(targe_file_path.to_owned().to_string_lossy().to_string());
    args.push("-C".to_string());
    args.push(file.to_owned().to_string_lossy().to_string());
    args.push(".".to_string());

    CommandRunner::Tar.run_command(&args).unwrap();
    fs::remove_dir(file).expect("Failed to remove directory after tar command successfully");

    Ok(())
}

fn get_package_full_name(target_selection: &TargetSelection) -> String {
    let mut target_file_name = String::new();
    // FIXME: this don't cover all possible channel names
    if ["nightly", "beta", "stable"].contains(&&*target_selection.channel) {
        target_file_name = format!(
            "rust-{}-{}.tar.gz",
            target_selection.channel, target_selection.target
        );
    }
    target_file_name
}

fn get_tool_folder_name(tool: &ExtendTool) -> String {
    let tool_dir: String;
    if let Some(tool_version) = &tool.version {
        tool_dir = format!("{}-{}", tool.name, tool_version);
    } else {
        tool_dir = format!("{}", tool.name);
    }
    tool_dir
}

fn package_extra_tools(tool: &ExtendTool, output_dir: &PathBuf) -> PathBuf {
    // init tool download dir
    let package_dir = init_package_dir(tool, output_dir);
    // install tool locally
    install_tool_local_directory(tool, &PathBuf::from(&package_dir));
    // init manifest.in file
    if let Err(err) = init_manifest_for_tool(&PathBuf::from(&package_dir)) {
        panic!("Couldn't init manifest for tool '{}'", err);
    };

    PathBuf::from(package_dir)
}

fn init_package_dir(tool: &ExtendTool, output_dir: &PathBuf) -> String {
    let tool_dir = get_tool_folder_name(tool);
    let download_path = output_dir.join(&tool_dir);
    utils::ensure_dir_exists(&"Tools download path".to_string(), &download_path).unwrap();

    download_path.to_owned().to_string_lossy().to_string()
}

fn install_tool_local_directory(tool: &ExtendTool, package_dir: &PathBuf) {
    let mut args = Vec::new();
    args.push("install".to_string());
    args.push(tool.crate_name());
    args.push("--root".to_string());
    args.push(package_dir.to_string_lossy().into_owned());
    
    if let Err(e) = CommandRunner::Cargo.run_command(&args){
        panic!("CommandRunner failed {}", e);
    };
}

fn init_manifest_for_tool(package_dir: &PathBuf) -> io::Result<()> {
    let manifest_path = package_dir.join("manifest.in");
    let mut manifest_file = File::create(manifest_path)?;

    write_directory_entries(&mut manifest_file, package_dir)?;
    Ok(())
}

const MANIFEST_FILENAME: &str = "manifest.in";
const CRATE_JSON_FILENAME: &str = ".crates2.json";
const CRATE_TOML_FILENAME: &str = ".crates.toml";

fn write_directory_entries(file: &mut File, dir_path: &Path) -> io::Result<()> {
    let mut queue = VecDeque::new();
    queue.push_back(dir_path.to_path_buf());

    while let Some(path) = queue.pop_front() {
        let entries = fs::read_dir(&path)?;
        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_file() {
                let rel_path = entry_path.strip_prefix(dir_path).expect("Invalid directory path");
                let format_path = rel_path.display().to_string().replace("\\", "/");
                let file_name = entry_path.file_name().unwrap();
                if file_name != MANIFEST_FILENAME && file_name != CRATE_JSON_FILENAME && file_name != CRATE_TOML_FILENAME {
                    writeln!(file,  "file:{}", format_path)?;
                }
            } else {
                queue.push_back(entry_path);
            }
        }
    }

    Ok(())
}

fn move_folder(from: &PathBuf, to: &PathBuf) -> io::Result<()> {
    let destination_path = to.join(from.file_name().unwrap());

    fs::rename(from, destination_path)?;

    Ok(())
}

fn update_components(components_folder: &Path, target_file_name: &String) -> Result<()> {
    let components_path = components_folder.join("components");
    let context = format!("{}\n", target_file_name);
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(&components_path)?;
    file.write_all(context.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    pub fn test_init_package_dir() {
        let tool = ExtendTool{
            name: "grcov".to_string(),
            version: Some("0.8.18".to_string()),
        };
        let path = PathBuf::from(r"D:\Normal\projects\rustup-plus-plus\");

        init_package_dir(&tool, &path);
    }

    #[test]
    #[ignore]
    pub fn test_install_tool_local_directory() {
        let tool = ExtendTool{
            name: "grcov".to_string(),
            version: Some("0.8.18".to_string()),
        };
        let path = PathBuf::from(r"D:\Normal\projects\rustup-plus-plus\grcov_0.8.18");

        install_tool_local_directory(&tool, &path);
    }

    #[test]
    #[ignore]
    pub fn test_init_manifest_file() {
        let path = PathBuf::from(r"D:\Normal\projects\rustup-plus-plus\grcov_0.8.18");
        if let Err(err) = init_manifest_for_tool(&path) {
            panic!("Couldn't init manifest for tool '{}'", err);
        };
    }

    #[test]
    #[ignore]
    pub fn test_move_file() {
        let from = PathBuf::from(r"D:\Normal\projects\rustup-plus-plus\grcov_0.8.18");
        let to = PathBuf::from(r"D:\Normal\projects\rustup-plus-plus\tmp");
        let _ = move_folder(&from, &to);
    }

    #[test]
    #[ignore]
    pub fn test_modify_components() {
        let path = PathBuf::from(r"D:\Normal\projects\rustup-plus-plus\rust-nightly-x86_64-pc-windows-msvc");
        let target_file_name = "grcov-0.8.18".to_string();
        update_components(&path, &target_file_name).unwrap();
    }
    
    #[test]
    #[ignore]
    pub fn test_try_package() {
        let target_file_name = "rust-nightly-x86_64-pc-windows-msvc.tar.gz";
        let output_dir = PathBuf::from(r"D:\Normal\projects\rustup-plus-plus");
        let file = PathBuf::from(r"D:\Normal\projects\rustup-plus-plus\rust-nightly-x86_64-pc-windows-msvc");
        try_package(&target_file_name.to_string(), &output_dir, &file).unwrap();
    }
}