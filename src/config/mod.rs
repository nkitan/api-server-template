mod environment;

use crate::cli_divider;
use anyhow::bail;
use environment::EnvironmentVariables;
use reqwest::Client;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing::{info, span, Level};

#[derive(Clone)]
pub struct ConfigState {
    pub env: EnvironmentVariables,
    pub appname: String,
    pub version: String,
    pub pgpool: Pool<Postgres>,
    pub client: Client,
}

impl ConfigState {
    pub async fn from_env() -> anyhow::Result<Self> {
        let env = EnvironmentVariables::from_env()?;
        let span = span!(Level::INFO, "db_connect_span", task = "connecting");
        let _enter = span.enter();
    
        // Hardcoded Values
        let appname: String = "API Server Template".to_string();
        let version: String = "0.1".to_string();
        
        // Database Connections
        let database_fqdn: String = format!("{}:{}", &env.database_host, &env.database_port);
        let connection_url: String = format!("postgresql://{}@{}/{}", &env.database_creds, database_fqdn, &env.database_name);

        println!("Attempting to connect to PgPool @ {database_fqdn}");
        info!("Attempting to connect to PgPool @ {database_fqdn}");
        let pgpool: Pool<Postgres> = match PgPoolOptions::new().max_connections(env.max_pool_connections.parse()?).connect(&connection_url).await {
            Ok(pool) => {
                println!("Connected to DB: {database_fqdn}");
                info!("Connected to DB: {database_fqdn}");
                pool
            },
            Err(err) => {
                info!("Failed To Connect To DB: {err}");
                bail!("Failed To Connect To DB: {err}");
            },
        };

        let client: Client = Client::new();

        cli_divider!();
        println!("Started {}", format!("{}:{} on port {}", appname.as_str(), version.as_str(), env.port));
        println!("Secret: {}", env.secret);

        Ok(Self {
            env,
            appname,
            version,
            pgpool,
            client,
        })
    }
}