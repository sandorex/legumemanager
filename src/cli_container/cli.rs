//! This file contains cli interface for use in container only

use clap::{Parser, Subcommand};

pub use crate::manager::ContainerManager;

/// Podman wrapper for managing pet containers, focused towards automated container setup without
/// using dedicated images
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct Cli {
    /// Increase verbosity (defaults to 1)
    #[arg(short, action = clap::ArgAction::Count, default_value_t = 1)]
    pub verbose: u8,

    /// Set verbosity (sets verbosity to 0)
    #[arg(short, long, conflicts_with = "verbose")]
    pub quiet: bool,

    /// Subcommand to use (if omitted defaults to host-exec)
    #[command(subcommand)]
    pub cmd: Option<CliCommands>,

    /// Execute command verbatim on host
    #[arg(last = true)]
    pub host_exec: Vec<String>,
}

#[derive(Subcommand, Debug)]
pub enum CliCommands {
    /// Entrypoint for the container, finish setup of the container
    Init,
}

