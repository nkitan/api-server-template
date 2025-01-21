mod environment;
use anyhow::bail;
use environment::EnvironmentVariables;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Clone)]
pub struct ConfigState {
    pub env: EnvironmentVariables,
    pub appname: String,
    pub version: String,
    pub pgpool: Pool<Postgres>,
}

impl ConfigState {
    pub async fn from_env() -> anyhow::Result<Self> {
        let env = EnvironmentVariables::from_env()?;

        // Hardcoded Values
        let appname: String = "API Server Template".to_string();
        let version: String = "0.1".to_string();

        // Database Connections
        let pgpool: Pool<Postgres> = match PgPoolOptions::new().max_connections(env.max_pool_connections).connect(&env.database_url).await {
            Ok(pool) => pool,
            Err(err) => bail!("Failed To Connect To DB: {err}"),
        };

        Ok(Self {
            env: env,
            appname: appname,
            version: version,
            pgpool: pgpool,
        })
    }
}