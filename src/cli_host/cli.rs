use clap::{Parser, Subcommand, Args, ArgAction};
use crate::cli_host::cli_ansible::AnsibleCommands;

pub use crate::manager::ContainerManager;

/// Podman wrapper for managing pet containers, focused towards automated container setup without
/// using dedicated images
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Increase verbosity (defaults to 1)
    #[arg(short, action = ArgAction::Count, default_value_t = 1)]
    pub verbose: u8,

    /// Set verbosity (sets verbosity to 0)
    #[arg(short, long, conflicts_with = "verbose")]
    pub quiet: bool,

    /// Run container manager as root (uses sudo or doas)
    #[arg(long)]
    pub root: bool,

    /// Specify which container manager to use (by defaults uses first one found)
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

    /// List all container made by legumemanager and their status
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

    /// Special commands for use with ansible
    #[command(subcommand)]
    Ansible(AnsibleCommands),
}

#[derive(Args, Debug, Clone)]
pub struct CmdCreateArgs {
    /// Name of the new container
    pub container_name: String,

    /// Image to use for the new container
    pub image: String,

    /// Hostname to set inside the container (defaults to host hostname)
    #[arg(short = 'H', long)]
    pub hostname: Option<String>,

    /// Home path for user inside the container (defaults to host home)
    #[arg(long)]
    pub home: Option<String>,

    /// If enabled home is set as home prefix, if home is not set then default prefix will be used
    #[arg(short = 'P', long)]
    pub home_prefix: bool,

    /// Do not mount host filesystem inside container (at /run/host)
    #[arg(long = "no-host-mount", action = clap::ArgAction::SetFalse, default_value_t = true)]
    pub mount_host: bool,

    /// Isolate IPC namespace
    #[arg(long)]
    pub unshare_ipc: bool,

    /// Isolate network namespace
    #[arg(long)]
    pub unshare_netns: bool,

    /// Isolate process namespace
    #[arg(long)]
    pub unshare_process: bool,

    /// Isolate /dev (host devices)
    #[arg(long)]
    pub unshare_devsys: bool,

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
    pub no_confirm: bool,
}

