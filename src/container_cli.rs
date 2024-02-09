/// This file contains cli interface for use in container only

use clap::{Parser, Subcommand};

/// This help is for commands available inside a container
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliContainer {
    /// Increase verbosity
    #[arg(short, action = clap::ArgAction::Count)]
    pub verbosity: u8,

    #[command(subcommand)]
    pub cmd: CliContainerCommands,
}

#[derive(Subcommand, Debug)]
pub enum CliContainerCommands {
    Init {

    },

    /// Execute command on the host using spawn-host
    HostExec {
        command: String,
        args: Vec<String>,
    }
}

pub fn main_cli_container() {
    // TODO if symlinked to distrobox-host-exec run host-exec directly

    let args = CliContainer::parse();

    use clap::CommandFactory;
    CliContainer::command().debug_assert()
}

