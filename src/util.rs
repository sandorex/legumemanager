pub const VERSION: &'static str = concat!(env!("CARGO_PKG_VERSION_MAJOR"), env!("CARGO_PKG_VERSION_MINOR"), env!("CARGO_PKG_VERSION_PATCH"));
pub const VERSION_STR: &'static str = env!("CARGO_PKG_VERSION");

#[derive(PartialEq, Eq)]
pub enum ContainerManager {
    Podman,
    Docker,
    Lilipod,
}

impl ContainerManager {
    pub fn executable(&self) -> &str {
        match *self {
            ContainerManager::Podman => "podman",
            ContainerManager::Docker => "docker",
            ContainerManager::Lilipod => "lilipod"
        }
    }
}

