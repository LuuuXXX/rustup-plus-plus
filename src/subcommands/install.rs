use std::{io::{Error}, env};

use crate::{Config, TargetSelection, Channel, run_command, run_command_without_output};

pub fn run_install(config: &Config) -> Result<(), Error>{
    if let Some(rustup_dist_server) = &config.rustup_dist_server {
        env::set_var("RUSTUP_DIST_SERVER", rustup_dist_server);
    }
    if let Some(rustup_update_root) = &config.rustup_update_root {
        env::set_var("RUSTUP_UPDATE_ROOT", rustup_update_root);
    }

    let target_selections = &config.target_selections;

    for target_selection in target_selections.iter() {
        // let _result = install_toolchain(&target_selection);
        if let Err(err) = install_toolchain(&target_selection) {
            panic!("Installing Rust from the yaml file failed, cause: {}", err);
        }
        if let Some(toolchain) = get_toolchain_full_name(&target_selection) {
            if let Err(err) = checkout_toolchain(&toolchain) {
                panic!("Checkout toolchain failed, cause: {}", err);
            }
            if let Err(err) = install_extra_tools(&target_selection.extra_tools) {
                panic!("Failed to install extra tools, cause: {}", err);
            } 
        }
    }

    Ok(())
}

fn install_toolchain(target_selection: &TargetSelection) -> Result<(), Error> {
    let toolchain = get_toolchain_full_name(target_selection);

    let mut args: Vec<String> = Vec::new();
    if let Some(toolchain) = &toolchain {
        args.push("toolchain".to_string());
        args.push("install".to_string());
        args.push(toolchain.to_lowercase());
        args.push("--profile".to_string());
        if let Some(profile) = &target_selection.profile {
            match profile {
                crate::Profile::Default => args.push("default".to_string()),
                crate::Profile::Complete => args.push("complete".to_string()),
                crate::Profile::Minimal => args.push("minimal".to_string()),
            }
        }
    }

    let command = "rustup".to_string();
    if let Err(err) = run_command(&command, &args) {
        println!("Failed to install toolchain, cause: {}", err);
    }

    Ok(())
}

fn install_extra_tools(extra_tools: &Option<Vec<String>>) -> Result<(), Error> {
    if let Some(extra_tools) = extra_tools {
        for tool in extra_tools.iter() {
            let command = "cargo".to_string();
            let mut args = Vec::new();
            args.push("install".to_string());
            args.push(tool.to_string());
            if let Err(err) = run_command(&command, &args) {
                panic!("Failure to install tool: {}", err);
            }
        }
    }

    Ok(())
}

fn get_toolchain_full_name(target_selection: &TargetSelection) -> Option<String> {
    let target = &target_selection.target;
    let channel = &target_selection.channel;

    let mut toolchain_date = String::new();
    if let Some(date) = &target_selection.date {
        toolchain_date = date.to_string();
    }

    let toolchain = target.as_ref().clone().and_then(|toolchain_target| {
        channel.as_ref().cloned().map(|toolchain_channel| {
            match toolchain_channel {
                Channel::Stable => {
                    "stable-".to_owned() + &toolchain_date + "-" + &toolchain_target.to_lowercase()
                },
                Channel::Beta => {
                    "beta-".to_owned() + &toolchain_date + "-" + &toolchain_target.to_lowercase()
                },
                Channel::Nightly => {
                    "nightly-".to_owned() + &toolchain_date + "-" + &toolchain_target.to_lowercase()
                },
            }
        })
    });

    toolchain
}

fn checkout_toolchain(toolchain: &String) -> Result<(), Box<dyn std::error::Error>> {
    let command = "rustup".to_string();
    let mut args = Vec::new();
    args.push("default".to_string());
    args.push(toolchain.to_string());
    let output = run_command_without_output(&command, &args);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_toolchain_full_name() {
        let mut target_selection = TargetSelection::default();
        target_selection.target = Some("x86_64-pc-windows-msvc".to_string());
        target_selection.channel = Some(Channel::Nightly);
        target_selection.date = Some("2023-06-14".to_string());
        if let Some(toolchain) = get_toolchain_full_name(&target_selection) {
            assert_eq!("nightly-2023-06-14-x86_64-pc-windows-msvc", &toolchain);
        }
    }
}