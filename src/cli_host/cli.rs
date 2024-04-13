use clap::{Parser, Subcommand, Args, ArgAction};
use std::path::PathBuf;

pub use crate::manager::ContainerManager;

/// Podman wrapper for managing pet containers, get VM like experience using containers
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Increase verbosity (defaults to 1)
    #[arg(short, action = ArgAction::Count, default_value_t = 1)]
    pub verbose: u8,

    /// No logging at all (sets verbosity to 0)
    #[arg(short, long, conflicts_with = "verbose")]
    pub quiet: bool,

    /// Output the command and quit
    #[arg(long)]
    pub dry_run: bool,

    /// Run container manager as root (uses sudo or doas)
    #[arg(long)]
    pub root: bool,

    /// Specify which container manager to use (by default uses first one found)
    #[arg(long, value_enum)]
    pub manager: Option<ContainerManager>,

    #[command(subcommand)]
    pub cmd: CliCommands,
}

#[derive(Subcommand, Debug)]
pub enum CliCommands {
    /// Create a container
    #[command(arg_required_else_help = true)]
    Create(CmdCreateArgs),

    /// Execute shell in a container
    #[command(arg_required_else_help = true)]
    #[clap(visible_alias = "enter")]
    Shell(CmdShellArgs),

    /// Execute command inside container
    #[command(arg_required_else_help = true)]
    Exec(CmdExecArgs),

    /// List all containers made by legumemanager and their status
    #[command(visible_alias = "ls")]
    List(CmdListArgs),

    /// Start a container
    #[command(arg_required_else_help = true)]
    Start(CmdStartArgs),

    /// Stop a container
    #[command(arg_required_else_help = true)]
    Stop(CmdStopArgs),

    /// Stop and delete a container
    #[command(arg_required_else_help = true)]
    #[clap(visible_alias = "rm")]
    Destroy(CmdDestroyArgs),

    /// Run ansible playbook inside container
    #[command(arg_required_else_help = true)]
    Ansible {
        /// Name of the container
        container_name: String,

        /// Path to playbook.yml
        playbook_path: PathBuf,

        // TODO add extra arguments which are passed to ansible-playbook itself
    },

    /// Get container status
    #[command(arg_required_else_help = true)]
    Status {
        /// Name of the container
        container_name: String,

        /// Return status as JSON formatted string
        #[arg(short, long)]
        json: bool,
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

#[derive(Args, Debug, Clone)]
pub struct CmdCreateArgs {
    // TODO allow podman to generate the name of container and just return it
    /// Name of the new container
    pub container_name: String,

    /// Image to use for the new container
    pub image: String,

    /// Hostname to set inside the container (defaults to host hostname)
    #[arg(short = 'H', long)]
    pub hostname: Option<String>,

    // TODO make it so default is ~/.lm/<container-name>-<container-image>
    // TODO make it so relative path is a suffix in ~/.lm/
    /// Home path for user inside the container (defaults to host home)
    #[arg(long)]
    pub home: Option<String>,

    /// If enabled home is set as home prefix, if home is not set then default prefix will be used
    #[arg(short = 'P', long)]
    pub home_prefix: bool,

    /// Use init system inside container (eg. systemd)
    #[arg(long)]
    pub init: bool,

    /// Define extra environment variables in container (eg. 'MY_VAR=value'), not error checked!
    #[arg(short, long)]
    pub env: Vec<String>,

    /// Pass extra arguments verbatim to container manager
    #[arg(short = 'a', long = "extra-arg")]
    pub extra_args: Vec<String>,
}

#[derive(Args, Debug, Clone)]
pub struct CmdShellArgs {
    /// Name of the container
    pub container_name: String,

    /// Use login shell
    #[arg(short, long)]
    pub login: bool,

    /// Set current working directory in the container
    #[arg(short, long)]
    pub workdir: Option<String>,

    /// Run in headless mode (no tty)
    #[arg(long)]
    pub headless: bool,

    /// Define extra environment variables in container (eg. 'MY_VAR=value')
    #[arg(short, long)]
    pub env: Vec<String>,

    /// Pass extra arguments verbatim to container manager
    #[arg(short = 'a', long = "extra-arg")]
    pub extra_args: Vec<String>,
}

#[derive(Args, Debug, Clone)]
pub struct CmdExecArgs {
    /// Name of the container
    pub container_name: String,

    /// Command to execute
    pub command: Vec<String>,

    /// Use login shell
    #[arg(short, long)]
    pub login: bool,

    /// Set current working directory in the container
    #[arg(short, long)]
    pub workdir: Option<String>,

    /// Define extra environment variables in container (eg. 'MY_VAR=value')
    #[arg(short, long)]
    pub env: Vec<String>,

    /// Pass extra arguments verbatim to container manager
    #[arg(short = 'a', long = "extra-arg")]
    pub extra_args: Vec<String>,
}

#[derive(Args, Debug, Clone)]
pub struct CmdListArgs{
    /// Show only running containers
    #[arg(long)]
    pub running: bool,
}

#[derive(Args, Debug, Clone)]
pub struct CmdStartArgs{
    /// Name of the container
    pub container_name: String,
}

#[derive(Args, Debug, Clone)]
pub struct CmdStopArgs {
    /// Name of the container
    pub container_name: String,

    /// Forcefully stop the container, may corrupt data
    #[arg(short, long)]
    pub force: bool,
}

#[derive(Args, Debug, Clone)]
pub struct CmdDestroyArgs {
    /// Name of the container
    pub container_name: String,

    /// Do not ask for confirmation
    #[arg(long)]
    pub force: bool,
}

