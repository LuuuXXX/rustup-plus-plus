use std::{env, path::PathBuf, fs};

use serde::Deserialize;

// Deprecated
pub static DEFAULT_RUSTUP_DIST_SERVER: &str = "https://static.rust-lang.org";
pub static DEFAULT_RUSTUP_UPDATE_ROOT: &str = "https://static.rust-lang.org/rustup";

// Fully-resolved toolchain descriptors. These always have full target
// triples attached to them and are used for canonical identification,
// such as naming their installation directory.
//
// as strings they look like stable-x86_64-pc-windows-msvc or
/// 1.55-x86_64-pc-windows-msvc
#[derive(Clone, Deserialize, Debug)]
pub struct TargetSelection {
    pub target: String,
    pub channel: String,
    pub date: Option<String>,
    // [possible values: minimal, default, complete]
    pub profile: Option<String>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct ExtendTool {
    pub name: String,
    pub version: Option<String>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct YamlConfig {
    #[serde(rename = "RUSTUP_DIST_SERVER")]
    pub rustup_dist_server: Option<String>,
    #[serde(rename = "RUSTUP_UPDATE_ROOT")]
    pub rustup_update_root: Option<String>,
    // rust toolchains
    #[serde(rename = "TARGETS")]
    pub targets: Vec<TargetSelection>,
    // extra rust tools
    #[serde(rename = "EXTEND_TOOLS")]
    pub extra_tools: Vec<ExtendTool>,
}

#[derive(Default, Clone, Debug)]
pub struct Config {
    pub rustup_dist_server: String,
    pub rustup_update_root: String,

    pub target_selections: Vec<TargetSelection>,

    pub extra_tools: Vec<ExtendTool>,
}

impl Config {
    pub fn parse(path: &PathBuf) -> Self {
        let mut config = Config::default();

        let yaml_config = YamlConfig::parse_yaml(&path);

        if let Some(rustup_dist_server) = yaml_config.rustup_dist_server {
            config.rustup_dist_server = env::var("RUSTUP_DIST_SERVER").unwrap_or(rustup_dist_server);
        } else {
            config.rustup_dist_server = String::from(DEFAULT_RUSTUP_DIST_SERVER);
        }

        if let Some(rustup_update_root) = yaml_config.rustup_update_root {
            config.rustup_update_root = env::var("RUSTUP_UPDATE_ROOT").unwrap_or(rustup_update_root);
        } else {
            config.rustup_update_root = String::from(DEFAULT_RUSTUP_UPDATE_ROOT);
        }

        config.target_selections = yaml_config.targets;
        config.extra_tools = yaml_config.extra_tools;

        config
    }
}

impl YamlConfig {
    // Deserialize yaml file data
    pub fn parse_yaml(path: &PathBuf) -> Self {
        let file_content = fs::read_to_string(&path).expect("Failed to read file content");
        let yaml_config = serde_yaml::from_str(&file_content).expect("failed to parse yaml config");
        
        yaml_config
    }
}

impl TargetSelection {
    pub fn mainifest_v1_url(&self, dist_root: &str) -> String {
        let do_mainifest_staging = env::var("RUSTUP_STAGED_MANIFEST").is_ok();
        match (self.date.as_ref(), do_mainifest_staging) {
            (None, false) => format!("{}/channel-rust-{}", dist_root, self.channel),  
            (Some(date), false) => format!("{}/{}/channel-rust-{}", dist_root, date, self.channel),
            (Some(_), true) => format!("{}/staging/channel-rust-{}", dist_root, self.channel),
            (None, true) => panic!("not a real-world case"),
        }
    }

    pub fn mainifest_v2_url(&self, dist_root: &str) -> String {
        format!("{}.toml", self.mainifest_v1_url(dist_root))
    }

    pub fn toolchain_name(&self) -> String {
        match self.date.as_ref() {
            Some(date) => format!("{}-{}-{}", self.channel, date, self.target),
            None => format!("{}-{}", self.channel, self.target),
        }
    }

    pub fn package_dir(&self, dist_root: &String) -> String {
        match self.date {
            None => dist_root.to_string(),
            Some(ref date) => format!("{dist_root}/dist/{date}"),
        }
    }
}

impl ExtendTool {
    pub fn crate_name(&self) -> String {
        match self.version.as_ref() {
            Some(version) => format!("{}@{}", self.name, version),
            None => format!("{}", self.name),
        }
    }
}