//! Container init command

use std::path::{Path, PathBuf};
use std::fs;
use std::process::Command;
use super::super::cli::{Cli, ContainerManager};
use crate::{Context, Error, Result};
use serde::Deserialize;

fn get_locked_mount_flags(path: &str) -> Option<Vec<String>> {
    // NOTE this may fail on older versions of findmnt

    #[derive(Deserialize)]
    struct FindMntFilesystem {
        target: String,
        options: String,
    }

    /// Structure returned by findmnt command when using --json argument
    #[derive(Deserialize)]
    struct FindMntResult {
        filesystems: Vec<FindMntFilesystem>,
    }


    let output = Command::new("findmnt")
        .args(["--json", "--target", path])
        .output()
        .expect("unable to execute findmnt");

    if !output.status.success() {
        return None;
    }

    if let Ok(json) = serde_json::from_slice::<FindMntResult>(&output.stdout) {
        if json.filesystems.len() > 1 {
            panic!("findmnt output has more than one filesystem!");
        }

        let fs: &FindMntFilesystem = &json.filesystems[0];
        if fs.target != path {
            return get_locked_mount_flags(&fs.target);
        }

        let mut flags: Vec<String> = vec![];
        for i in fs.options.split(",") {
            if i == "nodev" || i == "noexec" || i == "nosuid" {
                flags.push(i.into())
            }
        }

        return Some(flags);
    }

    // panic as this means something broke
    panic!("failed to parse findmnt output");
}

fn bind_mount(source: &str, mountpoint: &str, flags: Vec<String>) -> Result<()> {
    let mut source_path = PathBuf::from(source);

    // if its a link
    if source_path.as_path().is_symlink() {
        // do not prepend twice
        if !source_path.as_path().starts_with("/run/host/") {
            let symlink_target = source_path.as_path().read_link()
                .with_context(|| format!("failed to read link {:?}", source_path))?;
            source_path = PathBuf::from("/run/host").join(symlink_target);
        }
    }

    // check if source exists
    match source_path.as_path().try_exists() {
        Ok(true) => {},
        // im gonna ignore errors and return here
        _ => return Ok(()),
    }

    let mountpoint_path = Path::new(mountpoint);

    // check if mountpoint is a symlink
    if mountpoint_path.is_symlink() {
        // try to remove the file
        fs::remove_file(mountpoint_path)
            .with_context(|| format!("failed to delete link {}", mountpoint))?;
    } else {
        // check if mountpoint exists
        match mountpoint_path.try_exists() {
            Ok(true) => {
                // check if mountpoint is already mounted and unmount it
                let result = Command::new("findmnt")
                    .arg(mountpoint)
                    .output()
                    .with_context(|| "failed to execute findmnt command")?;

                // mountpoint is mounted
                if result.status.success() {
                    // unmount it
                    let result = Command::new("umount")
                        .arg(mountpoint)
                        .output()
                        .with_context(|| "failed to execute umount command")?;

                    // failed to unmount so fail fully
                    if !result.status.success() {
                        return Err(Error::msg(format!("failed to unmount {}", mountpoint)));
                    }
                }
            },
            // im gonna ignore errors and return here
            _ => return Ok(()),
        }
    }

    // if source path is a directory create mountpoint directory
    if source_path.is_dir() {
        fs::create_dir_all(mountpoint_path)
            .with_context(|| format!("failed to create directory {}", mountpoint))?;
    } else if source_path.is_file() {
        // if parent directory does not exist create it
        if let Some(parent_path) = source_path.parent() {
            if !parent_path.exists() {
                fs::create_dir_all(parent_path)
                    .with_context(|| format!("failed to create parent directory {:?}", parent_path))?;
            }
        }

        let _ = fs::File::create(&source_path)
            .with_context(|| format!("failed to create file {}", source));
    }

    // default flags to rslave
    let mount_flags = if flags.len() > 0 {
        flags
    } else {
        vec!["rslave".into()]
    };

    let result = Command::new("mount")
        .args(["--rbind", "-o", &mount_flags.join(","), source_path.as_path().to_str().unwrap(), mountpoint])
        .output()
        .with_context(|| "failed to execute mount command")?;

    if !result.status.success() {
        return Err(Error::msg(format!("mounting {} at {} has failed", source, mountpoint)));
    }

    Ok(())
}

/// Calls mount command and checks for success generating a clearer error messages on failure
fn mount(args: Vec<&'static str>) -> Result<()> {
    let result = Command::new("mount")
        .args(&args)
        .output()
        .with_context(|| "failed to execute mount")?;

    if !result.status.success() {
        return Err(Error::msg(format!("failed to execute 'mount {}'", args.join(" "))));
    }

    Ok(())
}

pub fn cmd_init(args: &Cli, manager: &ContainerManager) -> Result<()> {
    if args.verbose >= 1 {
        println!("Downloading host-spawn");
    }

    // TODO move all of to host

    if args.verbose >= 1 {
        println!("Setting up mounts");
    }

    mount(vec!["-t", "devpts", "devpts", "-o", "noexec,nosuid,newinstance,ptmxmode=0666,mode=0620", "/dev/pts/"])?;

    mount(vec!["--bind", "/dev/pts/ptmx", "/dev/ptmx"])?;

    mount(vec!["--make-rshared", "/"])?;

    // RO mounts
    for i in [
        "/etc/localtime",
        "/var/lib/systemd/coredump",
        "/var/log/journal",
    ] {
        let path = format!("/run/host{}", i);
        let flags = get_locked_mount_flags(&path)
            .context(format!("could not get mount flags for {}", &path))?;

        bind_mount(&i, path.as_str(), flags)?;
    }

    // TODO get user name and mount for ostree systems
    // if Path::new("/var/home/USER").exists() {
    //     bind_mount("/run/host/var/home/USER", "/home/USER")?;
    // }

    // RW mounts
    for i in [
        "/etc/host.conf",
        "/etc/machine-id",
        "/media",
        "/mnt",
        "/run/libvirt",
        "/run/media",
        "/run/netconfig/",
        "/run/systemd/journal",
        "/run/systemd/resolve/",
        "/run/systemd/seats",
        "/run/systemd/sessions",
        "/run/systemd/users",
        "/run/udev",
        "/var/lib/libvirt",
        "/var/mnt",
    ] {
        let path = format!("/run/host{}", i);
        let flags = get_locked_mount_flags(&path)
            .context(format!("could not get mount flags for {}", &path))?;

        bind_mount(&i, path.as_str(), flags)?;
    }

    // TODO find sockets

    Ok(())
}
