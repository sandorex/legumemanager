mod util;
mod cli_host;
mod cli_container;

use std::path::Path;

pub const VERSION: &'static str = concat!(env!("CARGO_PKG_VERSION_MAJOR"), env!("CARGO_PKG_VERSION_MINOR"), env!("CARGO_PKG_VERSION_PATCH"));
pub const VERSION_STR: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let force_host = if cfg!(debug_assertions) {
        let value = std::env::var("LM_FORCE_HOST").is_ok();

        if value {
            println!("WARNING: LM_FORCE_HOST env variable is only available in debug builds\n");
        }

        value
    } else {
        false
    };

    if (Path::new("/run/.containerenv").exists()
        || Path::new("/.dockerenv").exists()
        || std::env::var("container").is_ok())
        && !force_host {
        // running in a container
        cli_container::main();
    } else {
        cli_host::main();
    }
}
