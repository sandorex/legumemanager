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

