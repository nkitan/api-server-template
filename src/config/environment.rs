use std::borrow::Cow;
use anyhow::bail;

macro_rules! make_config {
    ($struct_name:ident { $( $field_name:ident : $field_type:ty ),* $(,)? }) => {
        impl $struct_name {
            pub fn from_env() -> anyhow::Result<Self> {
                dotenv::from_filename("AXUM.env").ok();

                Ok(Self {
                    $(
                        $field_name: match dotenv::var(stringify!($field_name).to_uppercase()) {
                            Ok(value) => value.into(),
                            Err(err) => bail!("Missing {}: {}", stringify!($field_name), err),
                        }
                    ),*
                })
            }
        }
    };
}

#[derive(Clone, Debug)]
pub struct EnvironmentVariables {
    pub database_host: Cow<'static, str>,
    pub database_port: Cow<'static, str>,
    pub database_creds: Cow<'static, str>,
    pub database_name: Cow<'static, str>,
    pub port: Cow<'static, str>,
    pub secret: Cow<'static, str>,
    pub hostname: Cow<'static, str>,
    pub max_pool_connections: Cow<'static, str>,
    pub kc_client_id: Cow<'static, str>,
    pub kc_client_secret: Cow<'static, str>,
    pub kc_login_url: Cow<'static, str>,
}

make_config!(EnvironmentVariables {
    database_host: Cow<'static, str>,
    database_port: Cow<'static, str>,
    database_creds: Cow<'static, str>,
    database_name: Cow<'static, str>,
    port: Cow<'static, str>,
    secret: Cow<'static, str>,
    hostname: Cow<'static, str>,
    max_pool_connections: Cow<'static, str>,
    kc_client_id: Cow<'static, str>,
    kc_client_secret: Cow<'static, str>,
    kc_login_url: Cow<'static, str>,
});