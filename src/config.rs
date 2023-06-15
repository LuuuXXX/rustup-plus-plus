use std::{env, path::PathBuf, fs};

use serde::Deserialize;

#[derive(Default, Clone, Debug)]
pub enum Channel {
    #[default]
    Stable,
    Beta,
    Nightly
}

#[derive(Default, Clone, Debug)]
pub enum Profile {
    #[default]
    Default,
    Complete,
    Minimal
}

#[derive(Default, Clone, Deserialize, Debug)]
pub struct TargetSelection {
    pub target: Option<String>,
    pub channel: Option<Channel>,
    pub profile: Option<Profile>,
    #[serde(rename = "extended")]
    pub extra_tools: Option<Vec<String>>
}

#[derive(Default, Clone, Deserialize, Debug)]
pub struct YamlConfig {
    #[serde(rename = "RUSTUP_DIST_SERVER")]
    pub rustup_dist_server: Option<String>,
    #[serde(rename = "RUSTUP_UPDATE_ROOT")]
    pub rustup_update_root: Option<String>,
    // rust toolchains
    pub targets: Vec<TargetSelection>,
}

#[derive(Default, Clone, Debug)]
pub struct Config {
    pub rustup_dist_server: Option<String>,
    pub rustup_update_root: Option<String>,

    pub target_selections: Vec<TargetSelection>,
}

impl<'de> Deserialize<'de> for Channel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> 
    {
        let value = String::deserialize(deserializer)?.to_lowercase();

        match value.as_str() {
            "stable" => Ok(Channel::Stable),
            "beta" => Ok(Channel::Beta),
            "nightly" => Ok(Channel::Nightly),
            _ => Err(serde::de::Error::custom("Invalid channel value")),
        }
    }
}

impl<'de> Deserialize<'de> for Profile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> 
    {
        let value = String::deserialize(deserializer)?.to_lowercase();

        match value.as_str() {
            "complete" => Ok(Profile::Complete),
            "default" => Ok(Profile::Default),
            "minimal" => Ok(Profile::Minimal),
            _ => Err(serde::de::Error::custom("Invalid profile value")),
        }
    }
}


impl Config {
    pub fn default_opts() -> Config {
        let mut config = Config::default();

        config.rustup_dist_server = Some(env::var("RUSTUP_DIST_SERVER").unwrap());
        config.rustup_update_root = Some(env::var("RUSTUP_UPDATE_ROOT").unwrap());

        config
    }

    pub fn parse(path: &PathBuf) -> Config {
        let mut config = Config::default_opts();
        let yaml_config = parse_yaml(&path);

        if let Some(rustup_dist_server) = yaml_config.rustup_dist_server {
            config.rustup_dist_server = Some(env::var("RUSTUP_DIST_SERVER").unwrap_or(rustup_dist_server));
        }
        if let Some(rustup_update_root) = yaml_config.rustup_update_root {
            config.rustup_update_root = Some(env::var("RUSTUP_UPDATE_ROOT").unwrap_or(rustup_update_root));
        }

        config.target_selections = yaml_config.targets;

        config
    }
}

#[allow(unused_assignments)]
pub fn parse_yaml(path: &PathBuf) -> YamlConfig {
    let mut yaml_config = YamlConfig::default();

    let file_content = fs::read_to_string(&path).expect("Failed to read file content");
    yaml_config = serde_yaml::from_str(&file_content).expect("failed to parse yaml config");
    
    yaml_config
}