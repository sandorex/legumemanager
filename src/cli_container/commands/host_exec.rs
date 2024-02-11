//! Host exec command

use super::super::cli::{Cli, ContainerManager};
use crate::{Result, Context};
use std::process::{Command, exit, Termination};

// TODO this will be temporary, experiment with writing custom host exec using named pipes
// https://stackoverflow.com/a/63719458
pub const HOST_SPAWN_URL: &'static str = "https://github.com/1player/host-spawn/releases/download/v1.5.1/host-spawn-x86_64";

// TODO download host-spawn on creation/first start

pub fn cmd_host_exec(args: &Cli, manager: &ContainerManager) -> Result<()> {
    let status = Command::new("host-spawn")
        .arg("--")
        .args(&args.host_exec)
        .spawn()
        .with_context(|| format!("failed to execute command on host"))?
        .wait()
        .with_context(|| "failed to wait for the child to execute")?;

    let rc = status.code().context("failed to get child exit code")?;

    // exit with same exit code
    exit(rc);
}

