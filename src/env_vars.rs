/// File containing names of every environment variable used in legumemanager

/// Development only flag to force host mode in a container
#[cfg(debug_assertions)]
pub const LM_FORCE_HOST: &'static str = "LM_FORCE_HOST";

/// Set custom home prefix
pub const LM_HOME_PREFIX: &'static str = "LM_HOME_PREFIX";

