//! This module parses all external configuration from environment variables.
//! For more details, see the '.env.example' file in the root of this repository.

use crate::api_v1::session::Keys;
use std::env;

/// For more details, see the '.env.example' file in the root of this repository.
#[derive(Clone)]
pub struct Config {
    pub listening_ip: String,
    pub listening_port: u16,
    pub database_url: String,

    pub log_level: tracing_subscriber::EnvFilter,

    /// This duration is in _seconds_.
    pub session_duration: i64,
    pub jwt_keys: Keys,

    /// Upload images directory
    pub upload_dir: String,
    pub max_upload_size: usize,
}

impl Config {
    /// Loads the global server configuration from the environment.
    ///
    /// # Panics
    /// This may panic if any of the environment variables have an invalid format or is not set.
    #[must_use]
    pub fn load() -> Self {
        let listening_ip =
            env::var("LISTENING_IP").expect("Environment variable LISTENING_IP must be set");
        let listening_port = env::var("LISTENING_PORT")
            .expect("Environment variable LISTENING_PORT must be set")
            .parse()
            .expect("Failed to parse environment variable LISTENING_PORT");
        let database_url =
            env::var("DATABASE_URL").expect("Environment variable DATABASE_URL must be set");

        let log_level = tracing_subscriber::EnvFilter::try_from_env("RUST_LOG")
            .expect("Environment variable RUST_LOG must be correctly configured");

        let session_duration = env::var("SESSION_EXPIRATION_SECONDS")
            .expect("Environment variable SESSION_EXPIRATION_SECONDS must be set")
            .parse()
            .expect("Failed to parse environment variable SESSION_EXPIRATION_SECONDS");
        let jwt_keys = Keys::new(
            env::var("JWT_SECRET")
                .expect("Environment variable JWT_SECRET must be set")
                .as_bytes(),
        );
        let upload_dir = env::var("UPLOAD_DIR").unwrap_or("./uploads".to_string());
        let max_upload_size = env::var("MAX_UPLOAD_SIZE_MB")
            .unwrap_or("10".to_string())
            .parse::<usize>()
            .expect("Failed to parse environment variable MAX_UPLOAD_SIZE_MB")
            * 1_048_576; // Mebibytes to bytes

        Self {
            listening_ip,
            listening_port,
            database_url,
            log_level,
            session_duration,
            jwt_keys,
            upload_dir,
            max_upload_size,
        }
    }
}
