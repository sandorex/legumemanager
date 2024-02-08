mod util;
mod generator;
mod cli;

use std::process::Command;
use clap::Parser;

fn main() {
    let args = cli::Cli::parse();

    use clap::CommandFactory;
    cli::Cli::command().debug_assert()
    // match &cli.command {
    //     Commands::Add { name } => {
    //         println!("'myapp add' was used, name is: {name:?}")
    //     }
    // }

    // for i in args {
    //     println!("'{}'", i);
    // }


    // let create_args = CreateArgs {
    //     manager: util::ContainerManager::Podman,
    //     image: "archlinux:latest",
    //     name: "test-container",
    //     hostname: "test-container.localhost",
    //     home: "/home/sandorex/.dbx/test-container",
    //     unshare_ipc: false,
    //     unshare_netns: false,
    //     unshare_process: false,
    //     unshare_devsys: false,
    //     init: false,
    //     rootful: false,
    //     mount_host: true,
    //     extra_env: vec![],
    // };
    // let enter_args = EnterArgs {
    //     manager: util::ContainerManager::Podman,
    //     name: "f39",
    //     home: "/home/sandorex",
    //     headless: false,
    //     workdir: None,
    //     extra_env: vec![],
    //     command: None,
    // };
    //
    // // let podman_args = generate_create_command(&create_args).unwrap();
    // let podman_args = generate_enter_command(&enter_args).unwrap();
    //
    // for i in podman_args {
    //     print!(" {}", i);
    // }

    // let command = Command::new("podman")
    //     .args(podman_args)
    //     .output()
    //     .expect("failed to execute podman");

    // println!("status: {}", command.status.code().unwrap_or(0));
    //
    // let command_output = String::from_utf8(command.stdout).unwrap();
    //
    // println!("{}", command_output);
}
