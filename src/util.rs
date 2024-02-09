use std::process::Command;

pub const HOST_SPAWN_URL: &'static str = "https://github.com/1player/host-spawn/releases/download/v1.5.1/host-spawn-x86_64";

/// Check whether executable exists in PATH
#[cfg(target_os = "linux")]
pub fn executable_exists(cmd: &str) -> bool {
    let output = Command::new("sh")
        .arg("-c").arg(format!("which {}", cmd))
        .output()
        .expect("failed to execute 'which'");

    output.status.success()
}

