use std::process::Command;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct WslDistro {
    pub name: String,
    pub is_default: bool,
    pub version: u8,
}

#[derive(Error, Debug)]
pub enum WslError {
    #[error("WSL not available")]
    NotAvailable,

    #[error("Command failed: {stderr}")]
    CommandFailed { stderr: String },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub fn detect_wsl_distros() -> Vec<WslDistro> {
    let output = Command::new("wsl.exe")
        .args(["--list", "--verbose"])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            parse_wsl_list(&stdout)
        }
        _ => Vec::new(),
    }
}

fn parse_wsl_list(output: &str) -> Vec<WslDistro> {
    output
        .lines()
        .skip(1)
        .filter_map(|line| {
            let line = line.trim().replace('\0', "");
            if line.is_empty() {
                return None;
            }

            let is_default = line.starts_with('*');
            let line = line.trim_start_matches('*').trim();

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                Some(WslDistro {
                    name: parts[0].to_string(),
                    is_default,
                    version: parts[2].parse().unwrap_or(2),
                })
            } else if !parts.is_empty() {
                Some(WslDistro {
                    name: parts[0].to_string(),
                    is_default,
                    version: 2,
                })
            } else {
                None
            }
        })
        .collect()
}

pub async fn execute_in_wsl(distro: &str, command: &str) -> Result<String, WslError> {
    let output = tokio::process::Command::new("wsl.exe")
        .args(["-d", distro, "--", "bash", "-c", command])
        .output()
        .await?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(WslError::CommandFailed {
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }
}

pub async fn check_fnm_in_wsl(distro: &str) -> bool {
    execute_in_wsl(distro, "which fnm")
        .await
        .map(|output| !output.trim().is_empty())
        .unwrap_or(false)
}
