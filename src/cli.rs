use std::path::PathBuf;
use clap::{Parser, Subcommand};

/// Podman wrapper for managing pet containers, focused towards automated container setup without
/// using dedicated images
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Increase verbosity
    #[arg(short, action = clap::ArgAction::Count)]
    pub verbosity: u8,

    /// Run container manager as root (uses sudo or doas)
    #[arg(long)]
    root: bool,

    #[command(subcommand)]
    pub cmd: CliCommands,
}

#[derive(Subcommand, Debug)]
pub enum CliCommands {
    /// Create a container
    #[command(arg_required_else_help = true)]
    Create {
        /// Name of the new container
        container_name: String,

        /// Image to use for the new container
        image: String,

        /// Hostname to set inside the container (defaults to host hostname)
        #[arg(short = 'H', long)]
        hostname: Option<String>,

        /// Do not mount host filesystem inside container (at /run/host)
        #[arg(long = "no-host-mount", action = clap::ArgAction::SetFalse, default_value_t = true)]
        mount_host: bool,

        /// Isolate IPC namespace
        #[arg(long)]
        unshare_ipc: bool,

        /// Isolate network namespace
        #[arg(long)]
        unshare_netns: bool,

        /// Isolate process namespace
        #[arg(long)]
        unshare_process: bool,

        /// Isolate /dev (host devices)
        #[arg(long)]
        unshare_devsys: bool,

        /// Use init system inside container (eg. systemd)
        #[arg(long)]
        init: bool,

        /// Define extra environment variables in container (eg. 'MY_VAR=value')
        #[arg(short, long)]
        env: Vec<String>,

        /// Pass extra arguments to container manager
        #[arg(short = 'a')]
        manager_args: Vec<String>,
    },

    /// Enter a container
    #[command(arg_required_else_help = true)]
    #[clap(visible_alias = "shell")]
    Enter {
        /// Name of the container
        container_name: String,

        /// Use login shell
        #[arg(short, long)]
        login: bool,

        /// Set current working directory in the container
        #[arg(short, long)]
        workdir: String,

        /// Run in headless mode (no tty)
        #[arg(long)]
        headless: bool,

        /// Define extra environment variables in container (eg. 'MY_VAR=value')
        #[arg(short, long)]
        env: Vec<String>,

        /// Pass extra arguments to container manager
        #[arg(short = 'a')]
        manager_args: Vec<String>,
    },

    /// Execute command inside container
    #[command(arg_required_else_help = true)]
    Exec {
        /// Name of the container
        container_name: String,

        // TODO test if this actually gather everything including arguments
        /// Command to execute
        command: Vec<String>,

        /// Use login shell
        #[arg(short, long)]
        login: bool,

        /// Set current working directory in the container
        #[arg(short, long)]
        workdir: String,

        /// Define extra environment variables in container (eg. 'MY_VAR=value')
        #[arg(short, long)]
        env: Vec<String>,

        /// Pass extra arguments to container manager
        #[arg(short = 'a')]
        manager_args: Vec<String>,
    },

    /// List all container made by legumemanager and their status
    List {
        /// Show only running containers
        #[arg(long)]
        running: bool,
    },

    /// Start a container
    #[command(arg_required_else_help = true)]
    Start {
        /// Name of the container
        container_name: String,
    },

    /// Stop a container
    #[command(arg_required_else_help = true)]
    Stop {
        /// Name of the container
        container_name: String,

        /// Forcefully stop the container, may corrupt data
        #[arg(short, long)]
        force: bool,
    },

    /// Stop and delete a container
    #[command(arg_required_else_help = true)]
    Destroy {
        /// Name of the container
        container_name: String,

        /// Do not ask for confirmation
        #[arg(long)]
        no_confirm: bool,
    },

    /// Special commands for use with ansible
    #[command(subcommand)]
    Ansible(AnsibleCommands)
}

#[derive(Subcommand, Debug)]
pub enum AnsibleCommands {
    /// Setup minimum requirements for running ansible inside the container
    #[command(arg_required_else_help = true)]
    Init {
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

