use std::path::Path;

mod util;

pub struct ContainerCreateArgs<'a> {
    pub manager: util::ContainerManager,
    pub name: &'a str,
    pub hostname: &'a str,
    pub home: &'a str,
    pub home_prefix: bool,

    pub unshare_ipc: bool,
    pub unshare_netns: bool,
    pub unshare_process: bool,
    pub unshare_devsys: bool,

    pub init: bool,
    pub rootful: bool,

    pub mount_host: bool,
}

// this function is pure, as in it does not modify the filesystem
pub fn generate_create_command(container: &ContainerCreateArgs) -> Result<Vec<String>, String> {
    let manager_exe = container.manager.executable();

    // let mut cmd: Args = Default::default();
    let mut cmd: Vec<String> = vec![];

    // TODO check name, hostname length

    cmd.extend([manager_exe.into(), "create".into(),
        "--name".into(), container.name.into(),
        "--hostname".into(), container.hostname.into(),
        "--privileged".into(),
        "--security-opt".into(), "label=disable".into(),
        "--security-opt".into(), "apparmor=unconfined".into(),
        "--user".into(), "root:root".into(),
    ]);

    if !container.unshare_ipc {
        cmd.extend(["--ipc".into(), "host".into()]);
    }

    if !container.unshare_netns {
        cmd.extend(["--network".into(), "host".into()]);
    }

    if !container.unshare_process {
        cmd.extend(["--pid".into(), "host".into()]);
    }

    // TODO do i need to set SHELL when creating seems like a bad idea?
    cmd.extend([
        // information about the manager, kinda compatible with distrobox
        "--label".into(), "manager=legumemanager".into(),
        "--env".into(), format!("manager_version={}",  util::VERSION),
        "--env".into(), format!("manager_version_str={}",  util::VERSION_STR),
        "--env".into(), format!("container={}", manager_exe),

        // im adding /bin/sh as default shell but will override it later
        "--env".into(), "SHELL=/bin/sh".into(),
        "--env".into(), format!("HOME={}", container.home),

        // use host terminfo as fallback, useful for modern terminals like kitty
        "--env".into(), "TERMINFO_DIRS=/usr/share/terminfo:/usr/share/terminfo-host".into(),
        "--volume".into(), "/usr/share/terminfo:/usr/share/terminfo-host:rslave".into(),

        "--volume".into(), format!("{0}:{0}:rslave", container.home),
        "--volume".into(), "/tmp:/tmp:rslave".into(),
    ]);

    // TODO mount /var/home/xxx for ostree systems

    if container.home_prefix {
        // if its a prefix set HOME_HOST to host home directory
        let host_home_path = dirs::home_dir().expect("cannot get host HOME dir");
        let host_home = host_home_path.to_str().unwrap();
        cmd.extend(["--env".into(), format!("HOME_HOST={}", host_home)]);
    } else {
        // keep HOME_HOST defined but same value as HOME
        cmd.extend(["--env".into(), format!("HOME_HOST={}", container.home)]);
    }

    if container.mount_host {
        cmd.extend(["--volume".into(), "/:/run/host:rslave".into()]);
    }

    // NOTE: im not gonna mount the legumemanager executable as it may be moved
    if !container.unshare_devsys {
        cmd.extend([
            "--volume".into(), "/dev:/dev:rslave".into(),
            "--volume".into(), "/sys:/sys:rslave".into(),
        ]);
    }

    // things for systemd
    if container.init {
        match container.manager {
            util::ContainerManager::Docker => {
                cmd.push("--cgroupns".into());
            },
            util::ContainerManager::Podman => {
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

    if !container.unshare_devsys {
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
    if shm.is_symlink() && !container.unshare_ipc {
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

    if Path::new(&user_xdg_runtime_path).exists() && !container.init {
        cmd.extend([
            "--volume".into(), format!("{0}:{0}:rslave", user_xdg_runtime_path),
        ])
    }

    // TODO i think there is a better way than just making these immutable
    if !container.unshare_netns {
        for file in ["/etc/hosts", "/etc/resolv.conf"] {
            if Path::new(file).exists() {
                cmd.extend([
                   "--volume".into(), format!("{0}:{0}:ro", file),
                ]);
            }
        }
    }

    if container.manager == util::ContainerManager::Podman {
        cmd.extend([
           "--ulimit".into(), "host".into(),
           "--annotation".into(), "run.oci.keep_original_groups=1".into(),
        ]);

        if container.init {
            cmd.push("--systemd=always".into());
        }

        if !container.rootful {
            cmd.extend([
               "--userns".into(), "keep-id".into(),
            ]);
        }
    }

    // im guessing this is the thing that gets called when the container starts
    // i want to support `podman start <container>` too for use with ansible
    cmd.extend([
       "--entrypoint".into(), "/usr/bin/entrypoint".into(),
    ]);

    Ok(cmd)
}
