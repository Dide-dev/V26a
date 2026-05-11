/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

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

    /// Ratelimiter config (second)
    pub rate_limit_per_second: u64,
    pub rate_limit_burst_size_second: u32,
    pub rate_limit_per_second_strict: u64,
    pub rate_limit_burst_size_second_strict: u32,

    /// Ratelimiter config (minute)
    pub rate_limit_per_minute: u64,
    pub rate_limit_burst_size_minute: u32,
    pub rate_limit_per_minute_strict: u64,
    pub rate_limit_burst_size_minute_strict: u32,
}

impl Config {
    /// Loads the global server configuration from the environment.
    ///
    /// # Panics
    /// This may panic if any of the environment variables have an invalid format or is not set.
    #[must_use]
    pub fn load() -> Self {
        let listening_ip = env::var("LISTENING_IP").unwrap_or("0.0.0.0".to_owned());
        let listening_port = env::var("LISTENING_PORT")
            .unwrap_or("3000".to_owned())
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

        let rate_limit_per_second = env::var("RATE_LIMIT_PER_SECOND")
            .unwrap_or("5".to_string())
            .parse::<u64>()
            .expect("Failed to parse RATE_LIMIT_PER_SECOND");

        let rate_limit_burst_size_second = env::var("RATE_LIMIT_BURST_SIZE_SECOND")
            .unwrap_or("10".to_string())
            .parse::<u32>()
            .expect("Failed to parse RATE_LIMIT_BURST_SIZE_SECOND");

        let rate_limit_per_second_strict = env::var("RATE_LIMIT_PER_SECOND")
            .unwrap_or("1".to_string())
            .parse::<u64>()
            .expect("Failed to parse RATE_LIMIT_PER_SECOND_STRICT");

        let rate_limit_burst_size_second_strict = env::var("RATE_LIMIT_BURST_SIZE_SECOND")
            .unwrap_or("3".to_string())
            .parse::<u32>()
            .expect("Failed to parse RATE_LIMIT_BURST_SIZE_SECOND_STRICT");

        let rate_limit_per_minute = env::var("RATE_LIMIT_PER_MINUTE")
            .unwrap_or("60".to_string())
            .parse::<u64>()
            .expect("Failed to parse RATE_LIMIT_PER_MINUTE");

        let rate_limit_burst_size_minute = env::var("RATE_LIMIT_BURST_SIZE_MINUTE")
            .unwrap_or("20".to_string())
            .parse::<u32>()
            .expect("Failed to parse RATE_LIMIT_BURST_SIZE_MINUTE");

        let rate_limit_per_minute_strict = env::var("RATE_LIMIT_PER_MINUTE")
            .unwrap_or("30".to_string())
            .parse::<u64>()
            .expect("Failed to parse RATE_LIMIT_PER_MINUTE_STRICT");

        let rate_limit_burst_size_minute_strict = env::var("RATE_LIMIT_BURST_SIZE_MINUTE")
            .unwrap_or("10".to_string())
            .parse::<u32>()
            .expect("Failed to parse RATE_LIMIT_BURST_SIZE_MINUTE_STRICT");

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
            rate_limit_per_second,
            rate_limit_burst_size_second,
            rate_limit_per_second_strict,
            rate_limit_burst_size_second_strict,
            rate_limit_per_minute,
            rate_limit_burst_size_minute,
            rate_limit_per_minute_strict,
            rate_limit_burst_size_minute_strict,
        }
    }
}
