//! Module that contains create command

use std::process::Command;
use std::path::Path;
use crate::cli_host::util;
use crate::{env_vars, VERSION, VERSION_STR};
use crate::cli_host::cli::{Cli, CmdCreateArgs, ContainerManager};
use crate::{Error, Result, Context};

fn generate_create_command(args: &Cli, cmd_args: &CmdCreateArgs) -> Result<Vec<String>> {
    let mut cmd: Vec<String> = vec![];

    let home = cmd_args.home.as_ref().unwrap();
    let hostname = cmd_args.hostname.as_ref().unwrap();
    let manager = args.manager.unwrap();

    if hostname.len() > 255 {
        return Err(Error::msg("hostname length is over 255 characters"));
    }

    if cmd_args.container_name.len() > 64 {
        return Err(Error::msg("container name is over 64 characters"));
    }

    cmd.extend(["create".into(),
        "--name".into(), cmd_args.container_name.clone(),
        "--hostname".into(), hostname.clone(),
        "--privileged".into(),
        "--security-opt".into(), "label=disable".into(),
        "--security-opt".into(), "apparmor=unconfined".into(),
        "--user".into(), "root:root".into(),
    ]);

    if !cmd_args.unshare_ipc {
        cmd.extend(["--ipc".into(), "host".into()]);
    }

    if !cmd_args.unshare_netns {
        cmd.extend(["--network".into(), "host".into()]);
    }

    if !cmd_args.unshare_process {
        cmd.extend(["--pid".into(), "host".into()]);
    }

    cmd.extend([
        // information about the manager, kinda compatible with distrobox
        "--label".into(), "manager=legumemanager".into(),
        // TODO add these to env_vars
        "--env".into(), format!("manager_used={}",  manager.get_executable_name()),
        "--env".into(), format!("manager_version={}",  VERSION),
        "--env".into(), format!("manager_version_str={}",  VERSION_STR),
        "--env".into(), format!("container={}", manager.get_executable_name()),

        // im adding /bin/sh as default shell but will override it later
        "--env".into(), "SHELL=/bin/sh".into(),
        "--env".into(), format!("HOME={}", home),

        // use host terminfo as fallback, useful for modern terminals like kitty
        "--env".into(), "TERMINFO_DIRS=/usr/share/terminfo:/run/host/usr/share/terminfo".into(),
        "--volume".into(), "/:/run/host:rslave".into(),
        "--volume".into(), format!("{0}:{0}:rslave", home),
        "--volume".into(), "/tmp:/tmp:rslave".into(),
    ]);

    // TODO mount /var/home/xxx for ostree systems

    let host_home_path = dirs::home_dir().with_context(|| "cannot get home directory path")?;
    let host_home = host_home_path.to_str().unwrap();
    cmd.extend(["--env".into(), format!("HOME_HOST={}", host_home)]);

    if !cmd_args.unshare_devsys {
        cmd.extend([
            "--volume".into(), "/dev:/dev:rslave".into(),
            "--volume".into(), "/sys:/sys:rslave".into(),
        ]);
    }

    // things for systemd
    if cmd_args.init {
        match manager {
            ContainerManager::Docker => {
                cmd.push("--cgroupns".into());
            },
            ContainerManager::Podman => {
                cmd.extend([
                   "--stop-signal".into(), "SIGRTMIN+3".into(),
                   "--mount".into(), "type=tmpfs,destination=/run".into(),
                   "--mount".into(), "type=tmpfs,destination=/run/lock".into(),
                   "--mount".into(), "type=tmpfs,destination=/var/lib/journal".into(),
                ]);
            },
            _ => {},
        }
    }

    if !cmd_args.unshare_devsys {
        cmd.extend([
            "--volume".into(), "/dev/pts".into(),
            "--volume".into(), "/dev/null:/dev/ptmx".into(),
        ]);
    }

    // i think this is obselete as https://github.com/containers/podman/issues/4452
    // has been solved
    if Path::new("/sys/fs/selinux").exists() {
        cmd.extend(["--volume".into(), "/sys/fs/selinux".into()]);
    }

    cmd.extend(["--volume".into(), "/var/log/journal".into()]);

    let shm = Path::new("/dev/shm");
    if shm.is_symlink() && !cmd_args.unshare_ipc {
        let link_target = shm.read_link().expect("failed to read /dev/shm link");
        cmd.extend([
            "--volume".into(), format!("{target}:{target}", target=link_target.to_str().unwrap())
        ]);
    }

    // make RHEL subscriptions work
    let rhel_sub_files: Vec<_> = vec![
        ("/etc/pki/entitlement/", "/run/secrets/etc-pki-entitlement"),
        ("/etc/rhsm/", "/run/secrets/rhsm"),
        ("/etc/yum.repos.d/redhat.repo", "/run/secrets/redhat.repo"),
    ];

    for (host_path, container_path) in rhel_sub_files {
        if Path::new(host_path).exists() {
            cmd.extend([
                "--volume".into(), format!("{}:{}:ro", host_path, container_path)
            ]);
        }
    }

    // mount XDG_RUNTIME_DIR
    let user_id = users::get_current_uid();
    let user_xdg_runtime_path = format!("/run/user/{}", user_id);

    if Path::new(&user_xdg_runtime_path).exists() && !cmd_args.init {
        cmd.extend([
            "--volume".into(), format!("{0}:{0}:rslave", user_xdg_runtime_path),
        ])
    }

    // TODO i think there is a better way than just making these immutable
    // TODO try editing when copying from host and editing a part, maybe also put some marker where
    // user can edit whatever they want and it wont be overwritten
    if !cmd_args.unshare_netns {
        for file in ["/etc/hosts", "/etc/resolv.conf"] {
            if Path::new(file).exists() {
                cmd.extend([
                   "--volume".into(), format!("{0}:{0}:ro", file),
                ]);
            }
        }
    }

    if manager == ContainerManager::Podman {
        cmd.extend([
           "--ulimit".into(), "host".into(),
           "--annotation".into(), "run.oci.keep_original_groups=1".into(),
        ]);

        if cmd_args.init {
            cmd.push("--systemd=always".into());
        }

        if !args.root {
            cmd.extend([
               "--userns".into(), "keep-id".into(),
            ]);
        }
    }

    // add additional env values, i wont check for errors here i dont care
    for i in &cmd_args.env {
        cmd.extend([
            "--env".into(), i.into(),
        ]);
    }

    // add additional flags
    cmd.extend(cmd_args.extra_args.clone());

    // im guessing this is the thing that gets called when the container starts
    // i want to support `podman start <container>` too for use with ansible
    cmd.extend([
        // TODO run login shell
        "--entrypoint".into(), "/bin/sh".into(),
        cmd_args.image.clone().into(),
    ]);

    Ok(cmd)
}

pub fn cmd_create(args: &Cli, mut cmd_args: CmdCreateArgs) -> Result<()> {
    // check if container already exists
    let state = util::get_container_state(args.manager.as_ref().unwrap(), &cmd_args.container_name)?;
    if state.is_some() {
        return Err(Error::msg(format!("container '{}' already exists", &cmd_args.container_name)));
    }

    // hostname defaults to host's hostname
    if cmd_args.hostname.is_none() {
        cmd_args.hostname = Some(gethostname::gethostname().into_string().unwrap());
    }

    // set home properly
    if let Some(home) = &cmd_args.home {
        if cmd_args.home_prefix {
            // treat --home as the prefix
            let new_home = Path::new(&home).join(&cmd_args.container_name);
            cmd_args.home = Some(new_home.to_str().unwrap().into());
        }
    } else {
        // home not set explicitly
        let mut new_home = dirs::home_dir().expect("failed to get home directory");

        // if home prefix then use the default
        if cmd_args.home_prefix {
            // place it in ~/<PREFIX>/<CONTAINER_NAME>
            // NOTE: if prefix is absolute path then it will overwrite the home
            new_home.push(std::env::var(env_vars::LM_HOME_PREFIX).unwrap_or(env_vars::LM_HOME_PREFIX_DEFAULT.into()));
            new_home.push(&cmd_args.container_name);
        }

        cmd_args.home = Some(new_home.to_str().unwrap().into());
    }

    if args.verbose >= 2 {
        println!("HOME: {}", cmd_args.home.as_ref().unwrap());
    }

    let home_path = Path::new(cmd_args.home.as_ref().unwrap());
    if !home_path.exists() {
        // create the home path
        std::fs::create_dir(home_path)
            .with_context(|| format!("cannot create home directory at '{}'", home_path.to_str().unwrap_or("NONE".into())))?;
    }

    let output = generate_create_command(args, &cmd_args)
        .with_context(|| "failed to generate podman create command")?;

    if args.dry_run {
        print!("{}", args.manager.unwrap().get_executable_name());
        for arg in output {
            print!(" {}", arg);
        }
        println!();
        return Ok(());
    }

    if args.verbose >= 1 {
        println!("Creating container {}", &cmd_args.container_name);
    }

    let command = Command::new(args.manager.unwrap().get_executable_name())
        .args(output)
        .output()
        .with_context(|| format!("failed to execute manager '{:?}'", args.manager.unwrap()))?;

    if command.status.success() {
        if args.verbose >= 1 {
            println!("Container successfully created");
        }

        Ok(())
    } else {
        // TODO add stdout and stderr together it will be nicer looking
        Err(Error::msg(
            format!("Container creation failed:\nStdout: {}\n\nStderr: {}",
                String::from_utf8(command.stdout).unwrap(),
                String::from_utf8(command.stderr).unwrap()
            )
        ))
    }
}

