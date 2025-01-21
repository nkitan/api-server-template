mod environment;
use environment::EnvironmentVariables;

#[derive(Clone)]
pub struct ConfigState {
    pub env: EnvironmentVariables,
    pub appname: String,
    pub version: String,
}

impl ConfigState {
    pub async fn from_env() -> anyhow::Result<Self> {
        let env = EnvironmentVariables::from_env()?;
        let appname: String = "API Server Template".to_string();
        let version: String = "0.1".to_string();
        
        Ok(Self {
            env: env,
            appname: appname,
            version: version,
        })
    }
}