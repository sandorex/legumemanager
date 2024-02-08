use crate::generator::manager::ContainerManager;

pub struct EnterArgs<'a> {
    pub manager: ContainerManager,
    pub name: &'a str,
    pub home: &'a str,
    pub headless: bool,
    pub workdir: Option<&'a str>,
    pub command: Option<&'a str>,
    pub extra_env: Vec<(&'a str, &'a str)>,
}

pub fn generate_enter_command(container: &EnterArgs) -> Result<Vec<String>, String> {
    // TODO filter the env better and allow some useful vars like DISPLAY etc
    let mut cmd: Vec<String> = vec![
        "exec".into(),
        "--interactive".into(),
        "--detach-keys=".into(),
        "--user".into(), "root".into(),
        // workdir defaults to home of the container
        format!("--workdir={}", container.workdir.unwrap_or(container.home)),
        "--env".into(), format!("CONTAINER_ID={}", container.name),
        "--env".into(), "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games".into(),
        // TODO ensure XDG_DATA_DIRS has /usr/local/share /usr/share
        // TODO ensure XDG_CONFIG_DIRS has /etc/xdg
        "--env".into(), format!("XDG_DATA_DIRS={}", std::env::var("XDG_DATA_DIRS").unwrap_or("/usr/local/share:/usr/share".into())),
        "--env".into(), format!("XDG_CONFIG_DIRS={}", std::env::var("XDG_CONFIG_DIRS").unwrap_or("/etc/xdg".into())),
        "--env".into(), format!("XDG_CACHE_HOME={}/.cache", container.home),
        "--env".into(), format!("XDG_CONFIG_HOME={}/.config", container.home),
        "--env".into(), format!("XDG_DATA_HOME={}/.local/share", container.home),
        "--env".into(), format!("XDG_STATE_HOME={}/.local/state", container.home),
    ];

    for (key, val) in &container.extra_env {
        cmd.extend([
            "--env".into(), format!("{}={}", key, val),
        ]);
    }

    if !container.headless {
        cmd.push("--tty".into());
    }

    cmd.push(container.name.into());

    let user_id = users::get_current_username().expect("could not get host username").into_string().unwrap();
    match container.command {
        Some(x) => cmd.extend([
            "sudo".into(), "-u".into(), user_id, "--".into(), x.into(),
        ]),
        None => cmd.extend([
            "sudo".into(), "-u".into(), user_id, "-s".into(),
        ]),
    }

    Ok(cmd)
}

