use serde::Deserialize;
use std::{
    net::{Ipv6Addr, SocketAddr},
    str::FromStr,
    sync::Arc,
};
pub type Config = Arc<Configuration>;

#[derive(Deserialize)]
pub struct Configuration {
    pub env: Environment,
    pub listen_address: SocketAddr,
    pub app_port: u16,
    pub db_dsn: String,
    pub db_pool_max_size: u32,
}

#[derive(Deserialize, Debug)]
pub enum Environment {
    Development,
    Production,
}

impl Configuration {
    pub fn new() -> Config {
        let env = env_var("APP_ENVIRONMENT").parse::<Environment>().expect("Unable to parse the value of the APP_ENVIRONMENT env variable. Please make sure it is set.");
        let app_port = env_var("PORT").parse::<u16>().expect(
            "Unable to parse the value of the PORT env variable. Please make sure it is set.",
        );
        let db_dsn = env_var("DATABASE_URL");
        let db_pool_max_size = env_var("DATABASE_POOL_MAX_SIZE").parse::<u32>().expect("Unable to parse the value of the DATBASE_POOL_MAX_SIZE env variable. Please make sure it is set.");
        let listen_address = SocketAddr::from((Ipv6Addr::UNSPECIFIED, app_port));

        Arc::new(Configuration {
            env,
            listen_address,
            app_port,
            db_dsn,
            db_pool_max_size,
        })
    }

    pub fn set_dsn(&mut self, db_dsn: String) {
        self.db_dsn = db_dsn
    }
}

impl FromStr for Environment {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "development" => Ok(Environment::Development),
            "production" => Ok(Environment::Production),
            _ => Err(format!(
                "Invalid environment: {}. Please set it to a valid value.",
                s
            )),
        }
    }
}

pub fn env_var(name: &str) -> String {
    std::env::var(name)
        .map_err(|e| format!("{}: {}", name, e))
        .expect("Missing environment variable")
}
