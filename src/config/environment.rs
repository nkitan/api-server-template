use std::borrow::Cow;

use anyhow::bail;

#[derive(Clone, Debug)]
pub struct EnvironmentVariables {
    pub database_host: Cow<'static, str>,
    pub database_port: u16,
    pub database_creds: Cow<'static, str>,
    pub database_name: Cow<'static, str>,
    pub port: u16,
    pub secret: Cow<'static, str>,
    pub hostname: Cow<'static, str>,
    pub max_pool_connections: u32,
}

impl EnvironmentVariables {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv::from_filename("AXUM.env").ok();

        Ok(Self {
            database_host: match dotenv::var("DATABASE_HOST") {
                Ok(url) => url.into(),
                Err(err) => bail!("missing DATABASE_URL: {err}"),
            },
            database_port: match dotenv::var("DATABASE_PORT") {
                Ok(port) => port.parse()?,
                _ => 8000,
            },
            database_creds: match dotenv::var("DATABASE_CREDS") {
                Ok(url) => url.into(),
                Err(err) => bail!("missing DATABASE_CREDS: {err}"),
            },
            database_name: match dotenv::var("DATABASE_NAME") {
                Ok(url) => url.into(),
                Err(err) => bail!("missing DATABASE_NAME: {err}"),
            },
            port: match dotenv::var("PORT") {
                Ok(port) => port.parse()?,
                _ => 8000,
            },
            secret: match dotenv::var("SECRET") {
                Ok(secret) => secret.into(),
                Err(err) => bail!("missing SECRET: {err}"),
            },
            hostname: match dotenv::var("HOSTNAME") {
                Ok(hostname) => hostname.into(),
                Err(err) => bail!("missing HOSTNAME: {err}"),
            },
            max_pool_connections: match dotenv::var("MAX_POOL_CONNECTIONS") {
                Ok(max_pool_connections) => max_pool_connections.parse()?,
                _ => 5, 
            }
        })
    }
}