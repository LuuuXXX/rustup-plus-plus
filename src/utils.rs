use std::{process::Command, error::Error};

#[macro_export]
macro_rules! execute_script {
    ($script_path:expr, $platform_cmd:expr) => {
        {
            let output = std::process::Command::new($platform_cmd)
                .arg($script_path)
                .output()
                .map_err(|e| format!("Failed to execute script: {}", e))?;

            if output.status.success() {
                Ok(())
            } else {
                let error_message = String::from_utf8_lossy(&output.stderr);
                Err(format!("Script execution failed: {}", error_message).into())
            }
        }
    };
}

pub fn run_command(cmd: &String, args: &[String]) -> Result<(), Box<dyn Error>> {
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