use std::path::Path;
use crate::{VERSION_STR, VERSION};
use crate::cli_host::cli::{Cli, CmdCreateArgs, ContainerManager};

fn generate_create_command(manager: ContainerManager, root: bool, absolute_home: String, args: &CmdCreateArgs) -> Result<Vec<String>, String> {
    let mut cmd: Vec<String> = vec![];

    // TODO check name, hostname length

    // get hostname but default to container_name
    let hostname = if let Some(hostname) = &args.hostname {
        hostname.clone()
    } else {
        gethostname::gethostname().into_string().unwrap()
    };

    cmd.extend(["create".into(),
        "--name".into(), args.container_name.clone(),
        "--hostname".into(), hostname,
        "--privileged".into(),
        "--security-opt".into(), "label=disable".into(),
        "--security-opt".into(), "apparmor=unconfined".into(),
        "--user".into(), "root:root".into(),
    ]);

    if !args.unshare_ipc {
        cmd.extend(["--ipc".into(), "host".into()]);
    }

    if !args.unshare_netns {
        cmd.extend(["--network".into(), "host".into()]);
    }

    if !args.unshare_process {
        cmd.extend(["--pid".into(), "host".into()]);
    }

    cmd.extend([
        // information about the manager, kinda compatible with distrobox
        "--label".into(), "manager=legumemanager".into(),
        "--env".into(), format!("manager_version={}",  VERSION),
        "--env".into(), format!("manager_version_str={}",  VERSION_STR),
        "--env".into(), format!("container={}", manager.get_executable_name()),

        // im adding /bin/sh as default shell but will override it later
        "--env".into(), "SHELL=/bin/sh".into(),
        "--env".into(), format!("HOME={}", absolute_home),

        // use host terminfo as fallback, useful for modern terminals like kitty
        "--env".into(), "TERMINFO_DIRS=/usr/share/terminfo:/usr/share/terminfo-host".into(),
        "--volume".into(), "/usr/share/terminfo:/usr/share/terminfo-host:ro".into(),

        "--volume".into(), format!("{0}:{0}:rslave", absolute_home),
        "--volume".into(), "/tmp:/tmp:rslave".into(),
    ]);

    // TODO mount /var/home/xxx for ostree systems

    if args.mount_host {
        cmd.extend(["--volume".into(), "/:/run/host:rslave".into()]);

        // TODO make HOME_HOST be /run/host/home/... so it works always
        // HOME_HOST is gonna be undefined if host is not mounted
        let host_home_path = dirs::home_dir().expect("cannot get host HOME dir");
        let host_home = host_home_path.to_str().unwrap();
        cmd.extend(["--env".into(), format!("HOME_HOST={}", host_home)]);
    }

    if !args.unshare_devsys {
        cmd.extend([
            "--volume".into(), "/dev:/dev:rslave".into(),
            "--volume".into(), "/sys:/sys:rslave".into(),
        ]);
    }

    // things for systemd
    if args.init {
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

    if !args.unshare_devsys {
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
    if shm.is_symlink() && !args.unshare_ipc {
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

    if Path::new(&user_xdg_runtime_path).exists() && !args.init {
        cmd.extend([
            "--volume".into(), format!("{0}:{0}:rslave", user_xdg_runtime_path),
        ])
    }

    // TODO i think there is a better way than just making these immutable
    // TODO try editing when copying from host and editing a part, maybe also put some marker where
    // user can edit whatever they want and it wont be overwritten
    if !args.unshare_netns {
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

        if args.init {
            cmd.push("--systemd=always".into());
        }

        if !root {
            cmd.extend([
               "--userns".into(), "keep-id".into(),
            ]);
        }
    }

    // add additional env values, i wont check for errors here i dont care
    for i in &args.env {
        cmd.extend([
            "--env".into(), i.into(),
        ]);
    }

    // add additional flags
    cmd.extend(args.extra_args.clone());

    // im guessing this is the thing that gets called when the container starts
    // i want to support `podman start <container>` too for use with ansible
    cmd.extend([
       "--entrypoint".into(), "/bin/sh".into(),
       args.image.clone().into(),
    ]);

    Ok(cmd)
}

pub fn cmd_create(args: &Cli, cmd_args: &CmdCreateArgs) {
    // TODO
    // check if container already exists
    // check if directory at home already exists and warn the user
    // check if

    let output = generate_create_command(args.manager.unwrap(), args.root, "aa".into(), cmd_args);

    for i in output.expect("failed to generate create command") {
        print!(" {}", i);
    }
}

