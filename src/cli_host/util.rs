use std::process::Command;
use super::cli::ContainerManager;
use anyhow::{Context, Result};
use std::collections::HashMap;

/// Inspects container and returns json, if container does not exist it will return None
pub fn container_inspect(manager: &ContainerManager, container_name: &str, format: &str) -> Result<Option<String>> {
    let manager_exe = manager.get_executable_name();
    let output = Command::new(manager_exe)
        .args(["container", "inspect", "--format", format, container_name])
        .output()
        .with_context(|| format!("unable to execute manager '{}'", manager_exe))?;

    let output_stdout = String::from_utf8(output.stdout).unwrap();

    // if it has failed then container probably does not exist
    if !output.status.success() {
        return Ok(None);
    }

    Ok(Some(output_stdout))
}

/// Returns container state from manager
pub fn get_container_state(manager: &ContainerManager, container_name: &str) -> Result<Option<String>> {
    container_inspect(manager, container_name, "{{.State.Status}}")
}

/// Returns container env variables from manager
pub fn get_container_env(manager: &ContainerManager, container_name: &str) -> Result<Option<HashMap<String, String>>> {
    // TODO clean this up bit later, this is messy
    // NOTE: this formats the json array in key=val format
    if let Some(env_str) = container_inspect(manager, container_name, r#"{{ range .Config.Env }}{{ . }}{{ printf "\n" }}{{ end }}"#)? {
        let env_str_lines: Vec<&str> = env_str.split('\n').collect();
        let mut map: HashMap<String, String> = HashMap::new();

        for line in env_str_lines {
            if let Some((before, after)) = line.split_once('=') {
                map.insert(before.into(), after.into());
            }
        }

        Ok(Some(map))
    } else {
        Ok(None)
    }
}
