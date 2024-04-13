//! Module contains shell command

use crate::cli_host::cli::ContainerManager;
use crate::cli_host::util;
use crate::{Result, Context, Error};
use super::super::cli::{Cli, CmdShellArgs};

fn generate_shell_command(args: &Cli, cmd_args: &CmdShellArgs, home: &String) -> Result<Vec<String>> {
    // TODO move all of this into /init.sh script
    // TODO filter the env better and allow some useful vars like DISPLAY etc
    let mut cmd: Vec<String> = vec![
        "exec".into(),
        "--interactive".into(),
        "--detach-keys=".into(),
        "--user".into(), "root".into(),
        format!("--workdir={}", cmd_args.workdir.as_ref().unwrap()),
        "--env".into(), format!("CONTAINER_ID={}", cmd_args.container_name),
        "--env".into(), "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games".into(),
        // TODO ensure XDG_DATA_DIRS has /usr/local/share /usr/share
        // TODO ensure XDG_CONFIG_DIRS has /etc/xdg
        "--env".into(), format!("XDG_DATA_DIRS={}", std::env::var("XDG_DATA_DIRS").unwrap_or("/usr/local/share:/usr/share".into())),
        "--env".into(), format!("XDG_CONFIG_DIRS={}", std::env::var("XDG_CONFIG_DIRS").unwrap_or("/etc/xdg".into())),
        "--env".into(), format!("XDG_CACHE_HOME={}/.cache", &home),
        "--env".into(), format!("XDG_CONFIG_HOME={}/.config", &home),
        "--env".into(), format!("XDG_DATA_HOME={}/.local/share", &home),
        "--env".into(), format!("XDG_STATE_HOME={}/.local/state", &home),
    ];

    for i in &cmd_args.env {
        cmd.extend([
            "--env".into(), i.into(),
        ]);
    }

    if !cmd_args.headless {
        cmd.push("--tty".into());
    }

    cmd.extend(cmd_args.extra_args.clone());

    cmd.push(cmd_args.container_name.clone());

    let user_id = users::get_current_username().with_context(|| "could not get host username")?.into_string().unwrap();

    if cmd_args.login {
        cmd.extend([
           "sudo".into(), "-u".into(), user_id, "-i".into(),
        ]);
    } else {
        cmd.extend([
           "sudo".into(), "-u".into(), user_id, "-s".into(),
        ]);
    }

    Ok(cmd)
}

pub fn cmd_shell(args: &Cli, mut cmd_args: CmdShellArgs) -> Result<()> {
    // TODO start the container if not running already?

    // check if container already exists
    let state = util::get_container_state(args.manager.as_ref().unwrap(), &cmd_args.container_name)?;
    if state.is_none() {
        return Err(Error::msg(format!("container '{}' does not exist", &cmd_args.container_name)));
    }

    let env_vars = util::get_container_env(args.manager.as_ref().unwrap(), &cmd_args.container_name)?
        .with_context(|| format!("could not inspect env variables of container '{}'", &cmd_args.container_name))?;

    let home = env_vars.get("HOME").with_context(|| format!("could not inspect HOME variable from container '{}'", &cmd_args.container_name))?;

    // default workdir to home
    if cmd_args.workdir.is_none() {
        cmd_args.workdir = Some(home.clone().into());
    }

    let cmd = generate_shell_command(args, &cmd_args, &home)?;

    if args.dry_run {
        print!("{}", args.manager.unwrap().get_executable_name());
        for arg in cmd {
            print!(" {}", arg);
        }
        println!();
        return Ok(());
    }

    // TODO run command

    Ok(())
}

