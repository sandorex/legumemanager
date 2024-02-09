//! Module contains shell command

use crate::cli_host::cli::ContainerManager;
use super::super::cli::{Cli, CmdShellArgs};

// pub struct EnterArgs<'a> {
//     pub manager: ContainerManager,
//     pub name: &'a str,
//     pub home: &'a str,
//     pub headless: bool,
//     pub workdir: Option<&'a str>,
//     pub command: Option<&'a str>,
//     pub extra_env: Vec<(&'a str, &'a str)>,
// }

fn generate_shell_command(args: &Cli, cmd_args: &CmdShellArgs, home: &String) -> Result<Vec<String>, String> {
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

    let user_id = users::get_current_username().expect("could not get host username").into_string().unwrap();

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

pub fn cmd_shell(args: &Cli, mut cmd_args: CmdShellArgs) {
    // TODO get home for the container, maybe podman inspect?
    // if cmd_args.workdir.is_none() {
    //     // TODO set workdir here to home if not set already
    //     cmd_args.workdir = Some(home);
    // }
}
