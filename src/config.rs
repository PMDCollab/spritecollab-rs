use dotenv::dotenv;
use std::env::var;

#[allow(dead_code)] // discord feature
pub enum Config {
    Address,
    GitRepo,
    GitAssetsUrl,
    Workdir,
    RedisHost,
    RedisPort,
}

impl Config {
    pub fn init() {
        dotenv().ok();
    }

    /// Makes sure all required config values are set and panics otherwise.
    pub fn check() {
        Self::Address.get();
        Self::GitRepo.get();
        Self::GitAssetsUrl.get();
        Self::Workdir.get();
        Self::RedisHost.get();
        Self::RedisPort.get();
    }

    pub fn get(&self) -> String {
        match self {
            Config::Address => var("SCSRV_ADDRESS").expect("SCSRV_ADDRESS not set"),
            Config::GitRepo => var("SCSRV_GIT_REPO").expect("SCSRV_GIT_REPO not set"),
            Config::GitAssetsUrl => {
                var("SCSRV_GIT_ASSETS_URL").expect("SCSRV_GIT_ASSETS_URL not set")
            }
            Config::Workdir => var("SCSRV_WORKDIR").expect("SCSRV_WORKDIR is not set"),
            Config::RedisHost => var("SCSRV_REDIS_HOST").expect("SCSRV_REDIS_HOST is not set"),
            Config::RedisPort => var("SCSRV_REDIS_PORT").expect("SCSRV_REDIS_PORT is not set"),
        }
    }

    pub fn get_or_none(&self) -> Option<String> {
        match self {
            Config::Address => var("SCSRV_ADDRESS").ok(),
            Config::GitRepo => var("SCSRV_GIT_REPO").ok(),
            Config::GitAssetsUrl => var("SCSRV_GIT_ASSETS_URL").ok(),
            Config::Workdir => var("SCSRV_WORKDIR").ok(),
            Config::RedisHost => var("SCSRV_REDIS_HOST").ok(),
            Config::RedisPort => var("SCSRV_REDIS_PORT").ok(),
        }
    }

    pub fn redis_config() -> (String, u16) {
        (
            Self::RedisHost.get(),
            Self::RedisPort
                .get()
                .parse::<u16>()
                .expect("Invalid Redis port"),
        )
    }
}
