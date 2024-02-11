use crate::util;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum ContainerManager {
    /// Use podman
    Podman,

    /// Use docker
    Docker,

    /// Use Lilypod
    Lilypod,
}

impl ContainerManager {
    /// Finds first available container manager in PATH, () if not found
    pub fn find_available() -> Result<Self, ()> {
        for manager in [
            ContainerManager::Podman,
            ContainerManager::Docker,
            ContainerManager::Lilypod,
        ] {
            let name = manager.get_executable_name();
            if util::executable_exists(name) {
                return Ok(manager);
            }
        }

        Err(())
    }

    /// Returns executable name of the container manager
    pub fn get_executable_name(&self) -> &'static str {
        match *self {
            ContainerManager::Podman => "podman",
            ContainerManager::Docker => "docker",
            ContainerManager::Lilypod => "lilypod",
        }
    }

    /// Returns enum which matches executable name
    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "podman" => Some(ContainerManager::Podman),
            "docker" => Some(ContainerManager::Docker),
            "lilypod" => Some(ContainerManager::Lilypod),
            _ => None,
        }
    }

}

