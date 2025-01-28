#[macro_export]
macro_rules! make_config {
    ($struct_name:ident { $( $field_name:ident : $field_type:ty ),* $(,)? }) => {
        impl $struct_name {
            pub fn from_env() -> anyhow::Result<Self> {
                dotenv::from_filename("AXUM.env").ok();

                Ok(Self {
                    $(
                        $field_name: match dotenv::var(stringify!($field_name).to_uppercase()) {
                            Ok(value) => value.into(),
                            Err(err) => anyhow::bail!("Missing {}: {}", stringify!($field_name), err),
                        }
                    ),*
                })
            }
        }
    };
}

#[macro_export]
macro_rules! cli_divider {
    () => {
        const CLI_DIVIDER_WIDTH: usize = 32;
        println!("{}", "-".repeat(CLI_DIVIDER_WIDTH));
    };
}

#[macro_export]
macro_rules! expect_admin {
    ($token: expr) => {
        if let Err(_) = axum_keycloak_auth::role::ExpectRoles::expect_roles($token, &[String::from("administrator")]) {
            return (
                StatusCode::FORBIDDEN,
                Json(json!({
                    "error": "insufficient privileges",
                })),
            )
        }
    };
}