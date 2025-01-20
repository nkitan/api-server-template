mod environment;
use environment::EnvironmentVariables;

#[derive(Clone)]
pub struct ConfigState {
    pub env: EnvironmentVariables,
}

impl ConfigState {
    pub async fn from_env() -> anyhow::Result<Self> {
        let env = EnvironmentVariables::from_env()?;
        Ok(Self {
            env: EnvironmentVariables::from_env()?,
        })
    }
}