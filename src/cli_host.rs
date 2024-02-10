/// Contains cli interface when running on host operating system (not container)

mod cli_ansible;
mod cli;
mod commands;
mod util;
mod main;

pub use main::main;

