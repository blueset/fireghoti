// FIXME: Are these options used?
#[crate::export(object)]
pub struct EnvConfig {
    pub only_queue: bool,
    pub only_server: bool,
    pub no_daemons: bool,
    pub disable_clustering: bool,
    pub verbose: bool,
    pub with_log_time: bool,
    pub slow: bool,
}

#[crate::export]
pub fn read_environment_config() -> EnvConfig {
    let node_env = std::env::var("NODE_ENV").unwrap_or_default().to_lowercase();
    let is_testing = node_env == "test";

    EnvConfig {
        only_queue: std::env::var("MK_ONLY_QUEUE").is_ok(),
        only_server: std::env::var("MK_ONLY_SERVER").is_ok(),
        no_daemons: is_testing || std::env::var("MK_NO_DAEMONS").is_ok(),
        disable_clustering: is_testing || std::env::var("MK_DISABLE_CLUSTERING").is_ok(),
        verbose: std::env::var("MK_VERBOSE").is_ok(),
        with_log_time: std::env::var("MK_WITH_LOG_TIME").is_ok(),
        slow: std::env::var("MK_SLOW").is_ok(),
    }
}
