/// This file contains cli interface for use in container only

use clap::{Parser, Subcommand};

// TODO write better description for in container help
/// legumemanager wrapper for podman, container centric commands
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Increase verbosity (defaults to 1)
    #[arg(short, action = clap::ArgAction::Count, default_value_t = 1)]
    pub verbose: u8,

    /// Set verbosity (sets verbosity to 0)
    #[arg(short, long, conflicts_with = "verbose")]
    pub quiet: bool,

    #[command(subcommand)]
    pub cmd: CliCommands,
}

#[derive(Subcommand, Debug)]
pub enum CliCommands {
    Init {

    },

    /// Execute command on the host using spawn-host
    HostExec {
        command: String,
        args: Vec<String>,
    }
}

