use std::fmt;

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    #[serde(default = "defaults::port")]
    pub port: u16,
    pub database_url: String,
    #[serde(default = "defaults::db_pool_size")]
    pub db_pool_size: u32,
}

impl Config {
    pub fn new() -> Result<Self, anyhow::Error> {
        let this = envy::from_env::<Self>()?;
        Ok(this)
    }

    #[cfg(test)]
    pub fn new_for_tests() -> Self {
        use crate::config::defaults::{db_pool_size, port};

        dotenv::dotenv().ok();

        Config {
            port: port(),
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL not defined"),
            db_pool_size: db_pool_size(),
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "- Port: {:}", self.port)?;
        match url::Url::parse(&self.database_url) {
            Ok(url) => {
                writeln!(f, "- Database Host: {:?}", url.host())?;
                writeln!(f, "- Database Port: {:?}", url.port())?;
            }
            Err(_) => writeln!(f, "- Malformed URL")?,
        };
        writeln!(f, "- Database Pool Size: {:}", self.db_pool_size)
    }
}

mod defaults {
    pub const fn port() -> u16 {
        5050
    }

    pub const fn db_pool_size() -> u32 {
        5
    }
}
