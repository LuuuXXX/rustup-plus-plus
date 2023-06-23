use std::{io::Error, path::PathBuf, env, fs};

use crate::{Config, DownloadCfg, ExtendTool, run_cargo, run_tar};

pub fn run_package(config: &Config, output_dir: &PathBuf) -> Result<(), Error>{
    env::set_var("RUSTUP_USE_CURL", "true");

    for target_selection in &config.target_selections {
        // parse to tagert package download path
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

    for extra_tool in &config.extra_tools {
        package_extra_tools(extra_tool, output_dir);
    }

    Ok(())
}

fn package_extra_tools(tool: &ExtendTool, output_dir: &PathBuf) {
    // init tool download dir
    let package_dir = init_package_dir(tool, output_dir);
    // install tool locally
    install_tool_local_directory(tool, &PathBuf::from(&package_dir));
    // package tool
    package_extra_tool(tool, &PathBuf::from(&package_dir));
    
}

fn package_extra_tool(tool: &ExtendTool, package_dir: &PathBuf) {
    let mut args = Vec::new();
    args.push("-czvf".to_string());
    let parent_dir = package_dir.parent().expect("Failed to get parent directory");
    let package_name : String;
    if let Some(tool_version) = &tool.version {
        package_name = format!("{}-{}.tar.gz", tool.name, tool_version);
    } else {
        package_name = format!("../{}.tar.gz", tool.name);
    }
    args.push(format!("{}/{}", parent_dir.to_string_lossy().to_owned(), package_name));
    args.push(package_dir.to_string_lossy().into_owned());

    run_tar(&args);

    fs::remove_dir_all(package_dir).expect("Failed to remove package directory");
}

fn install_tool_local_directory(tool: &ExtendTool, package_dir: &PathBuf) {
    let mut args = Vec::new();
    args.push("install".to_string());
    args.push(tool.crate_name());
    args.push("--root".to_string());
    args.push(package_dir.to_string_lossy().into_owned());
    
    run_cargo(&args);
}

fn init_package_dir(tool: &ExtendTool, output_dir: &PathBuf) -> String {
    let tool_dir: String;
    if let Some(tool_version) = &tool.version {
        tool_dir = format!("{}_{}", tool.name, tool_version);
    } else {
        tool_dir = format!("{}", tool.name);
    }
    let download_root_with_tool = output_dir.join(&tool_dir);
    if let Err(err) = std::fs::create_dir_all(&download_root_with_tool) {
        panic!("Failed to create package directory: {:?}", err);
    }
    let download_root_url = download_root_with_tool.to_string_lossy().into_owned();

    download_root_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_init_package_dir() {
        let tool = ExtendTool{
            name: "grcov".to_string(),
            version: Some("0.8.18".to_string()),
        };
        let path = PathBuf::from(r"D:\Normal\projects\rustup-plus-plus\");

        init_package_dir(&tool, &path);
    }

    #[test]
    pub fn test_install_tool_local_directory() {
        let tool = ExtendTool{
            name: "grcov".to_string(),
            version: Some("0.8.18".to_string()),
        };
        let path = PathBuf::from(r"D:\Normal\projects\rustup-plus-plus\grcov_0.8.18");

        install_tool_local_directory(&tool, &path);
    }

    #[test]
    pub fn test_package_extra_tool() {
        let tool = ExtendTool{
            name: "grcov".to_string(),
            version: Some("0.8.18".to_string()),
        };
        let path = PathBuf::from(r"D:\Normal\projects\rustup-plus-plus\grcov_0.8.18");

        package_extra_tool(&tool, &path);
    }
}