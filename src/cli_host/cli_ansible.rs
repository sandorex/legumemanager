/// Ansible subcommand interface

use std::path::PathBuf;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum AnsibleCommands {
    /// Setup minimum requirements for running ansible inside the container
    #[command(arg_required_else_help = true)]
    Initialize {
        /// Name of the container
        container_name: String,

        /// Aditional packages to install
        #[arg(short)]
        additional_packages: Vec<String>,

        /// Install ansible inside the container
        #[arg(long)]
        ansible: bool,
    },

    /// Get container status
    #[command(arg_required_else_help = true)]
    Status {
        /// Name of the container
        container_name: String,
    },

    /// Push files to a container
    #[command(arg_required_else_help = true)]
    Push {
        /// Name of the container
        container_name: String,

        /// Source on host
        source: PathBuf,

        /// Destination in the container
        destination: PathBuf,
    },

    /// Fetch files from a container
    #[command(arg_required_else_help = true)]
    Pull {
        /// Name of the container
        container_name: String,

        /// Source on host
        source: PathBuf,

        /// Destination in the container
        destination: PathBuf,
    },
}

