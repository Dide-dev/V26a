/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

//! This module parses all external configuration from environment variables.
//! For more details, see the '.env.example' file in the root of this repository.

use bcrypt::DEFAULT_COST;
use std::env;

/// For more details, see the '.env.example' file in the root of this repository.
#[derive(Clone)]
pub struct Config {
    pub listening_ip: String,
    pub listening_port: u16,
    pub database_url: String,

    pub log_level: tracing_subscriber::EnvFilter,

    /// Defines the amount of iterations (2^n) used by the bcrypt algorithm.
    /// Higher values are harder to crack but take longer to compute.
    pub bcrypt_cost: u32,
    /// This duration is in _seconds_.
    pub session_duration: i64,

    /// Upload images directory
    pub upload_dir: String,
    pub max_upload_size: usize,

    /// Ratelimiter config (minute)
    pub rate_limit_per_minute: u64,
    pub rate_limit_burst_size: u32,
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

        let bcrypt_cost = env::var("BCRYPT_COST").map_or(DEFAULT_COST, |s| {
            s.parse()
                .expect("Failed to parse environment variable BCRYPT_COST")
        });
        let session_duration = env::var("SESSION_EXPIRATION_SECONDS")
            .expect("Environment variable SESSION_EXPIRATION_SECONDS must be set")
            .parse()
            .expect("Failed to parse environment variable SESSION_EXPIRATION_SECONDS");
        let rate_limit_per_minute = env::var("RATE_LIMIT_PER_MINUTE")
            .unwrap_or("30".to_string())
            .parse::<u64>()
            .expect("Failed to parse RATE_LIMIT_PER_MINUTE");

        let rate_limit_burst_size = env::var("RATE_LIMIT_BURST_SIZE")
            .unwrap_or("40".to_string())
            .parse::<u32>()
            .expect("Failed to parse RATE_LIMIT_BURST_SIZE");

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
            bcrypt_cost,
            session_duration,
            upload_dir,
            max_upload_size,
            rate_limit_per_minute,
            rate_limit_burst_size,
        }
    }
}
