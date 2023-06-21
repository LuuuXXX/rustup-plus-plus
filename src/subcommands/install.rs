use std::{io::{Error}, env};

use crate::{Config, TargetSelection, run_rustup, ExtendTool, run_cargo};

pub fn run_install(config: &Config) -> Result<(), Error>{
    env::set_var("RUSTUP_DIST_SERVER", &config.rustup_dist_server);
    env::set_var("RUSTUP_UPDATE_ROOT", &config.rustup_update_root);

    for target_selection in config.target_selections.iter() {
        install_toolchain(&target_selection);
    }

    for extra_tool in config.extra_tools.iter() {
        install_extra_tools(&extra_tool);
    }

    Ok(())
}

fn install_toolchain(target_selection: &TargetSelection) {
    let toolchain = target_selection.toolchain_name();

    let mut args: Vec<String> = Vec::new();
    args.push("toolchain".to_string());
    args.push("install".to_string());
    args.push(toolchain.to_lowercase());
    if let Some(profile) = &target_selection.profile {
        // Default profile selection is : default
        args.push("--profile".to_string());
        args.push(profile.to_string());
    }

    run_rustup(&args);
}

fn install_extra_tools(tool: &ExtendTool) {
    let tool = tool.crate_name();
    
    let mut args: Vec<String> = Vec::new();
    args.push("install".to_string());
    args.push(tool.to_lowercase());

    run_cargo(&args);
}