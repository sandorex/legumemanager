use std::{path::PathBuf, process::Command};
use super::cli::ContainerManager;
use crate::{Context, Error, Result};
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

/// Pushes the binary into container
pub fn push_executable_into_container(manager: &ContainerManager, container_name: &str, path: PathBuf) -> Result<()> {
    let manager_exe = manager.get_executable_name();

    // i am only testing if the container exists
    match get_container_state(manager, container_name) {
        // the container exists
        Ok(Some(_)) => {
            match std::env::current_exe() {
                Ok(current_exe) => {
                    let status = Command::new(manager_exe)
                        .args(["container", "cp", current_exe.to_str().expect("error converting current_exe to &str"), format!("{}:/lm", container_name).as_str()])
                        .status()
                        .with_context(|| format!("unable to execute manager '{}'", manager_exe))?;

                    if status.success() {
                        Ok(())
                    } else {
                        Err(Error::msg(format!("Failed to copy executable into container '{}'", container_name)))
                    }
                }
                Err(err) => Err(err.into()),
            }
        },

        // the container does not exist
        Ok(None) => Err(Error::msg(format!("Container '{}' does not exist", container_name))),

        // Some kind of error happen
        Err(err) => Err(err),
    }
}

// TODO create is_owned_container() to check if the container is made by legumemanager, forbid
// using it for foreign containers to avoid problems

