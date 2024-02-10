use std::process::Command;
use serde_json;
use super::cli::ContainerManager;
use anyhow::{Context, Result};

/// Inspects container and returns json, with i32 being the exit code from container manager
pub fn container_inspect(manager: &ContainerManager, container_name: &str) -> Result<Option<serde_json::Value>> {
    let manager_exe = manager.get_executable_name();
    let output = Command::new(manager_exe)
        .args(["container", "inspect", container_name])
        .output()
        .with_context(|| format!("unable to execute manager {}", manager_exe))?;

    let output_stdout = String::from_utf8(output.stdout).unwrap();

    // if it has failed then container probably does not exist
    if !output.status.success() {
        return Ok(None);
    }

    serde_json::from_str(&output_stdout)
        .map(|x| Ok(Some(x)))
        .with_context(|| format!("failed to parse json from manager {}", manager_exe))?
}

/// Returns container state from manager
pub fn get_container_state(manager: &ContainerManager, container_name: &str) -> Result<Option<String>> {
    match container_inspect(manager, container_name) {
        Ok(json) => match json {
            Some(x) => Ok(Some(x[0]["State"]["Status"].to_string())),
            None => Ok(None),
        },
        Err(x) => Err(x.into()),
    }
}

