use clap::Parser;
use crate::cli_container::cli::{self, CliCommands};
use crate::{Result, Context};
use crate::manager::ContainerManager;

use super::commands;

pub fn main() -> Result<()> {
    // TODO if symlinked to distrobox-host-exec run host-exec directly
    let mut args = cli::Cli::parse();

    // if quiet stay quiet
    if args.quiet {
        args.verbose = 0;
    }

    // NOTE using env var set on creation, reading label from within a container seems like pain
    let manager_used = std::env::var("manager_used")
        .with_context(|| "environment variable 'manager_used' is not defined, this container was not managed by legumemanager")?;
    let manager = ContainerManager::from_str(&manager_used)
        .context(format!("unsupported container manager '{}' used for container", &manager_used))?;

    match &args.cmd {
        Some(CliCommands::Init) => commands::cmd_init(&args, &manager),
        _ => commands::cmd_host_exec(&args, &manager),
    }
}

