use clap::Parser;
use crate::cli_container::cli;

pub fn main() {
    // TODO if symlinked to distrobox-host-exec run host-exec directly
    let args = cli::Cli::parse();

    println!("host {:?}", args);
}

