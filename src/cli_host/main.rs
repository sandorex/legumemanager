use clap::Parser;
use super::cli::{Cli, CliCommands, ContainerManager};
use super::commands;
use crate::Result;

pub fn main() -> Result<()> {
    let mut args = Cli::parse();

    // set the manager now so its less complicated later on
    if args.manager.is_none() {
        args.manager = Some(ContainerManager::find_available().expect("no container manager found!"));
    }

    // if quiet stay quiet
    if args.quiet {
        args.verbose = 0;
    }

    println!("host {:?}", args);

    match &args.cmd {
        CliCommands::Create(cmd_args) => commands::cmd_create(&args, cmd_args.clone()),
        CliCommands::Shell(cmd_args) => commands::cmd_shell(&args, cmd_args.clone()),
        _ => Ok(()),
    }

    // use clap::CommandFactory;
    // Cli::command().debug_assert()
}
