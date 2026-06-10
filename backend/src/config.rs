/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

//! This module parses all external configuration from environment variables.
//! For more details, see the '.env.example' file in the root of this repository.

use crate::db_structs::mail::{Mailer, SmtpMailer};
use bcrypt::DEFAULT_COST;
use lettre::{AsyncSmtpTransport, Tokio1Executor};
use std::env;
use std::sync::Arc;

/// For more details, see the '.env.example' file in the root of this repository.
#[derive(Clone)]
pub struct Config {
    pub listening_ip: String,
    pub listening_port: u16,
    pub hostname: String,
    pub base_url: String,
    pub database_url: String,

    pub log_level: tracing_subscriber::EnvFilter,

    /// Defines the amount of iterations (2^n) used by the bcrypt algorithm.
    /// Higher values are harder to crack but take longer to compute.
    pub bcrypt_cost: u32,
    /// This duration is in _seconds_.
    pub session_duration: i64,
    /// This duration is in _seconds_.
    pub verification_timer: i64,

    /// Upload images directory
    pub upload_dir: String,
    pub max_upload_size: usize,

    pub mailer: Arc<dyn Mailer>,

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

        let base_url = env::var("BASE_URL").expect("Environment variable BASE_URL must be set");
        let (base_url, hostname) = parse_base_url(base_url.as_str());

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
        let verification_timer = env::var("VERIFICATION_TIMER_SECONDS")
            .unwrap_or("900".to_owned())
            .parse()
            .expect("Failed to parse environment variable VERIFICATION_TIMER_SECONDS");

        let upload_dir = env::var("UPLOAD_DIR").unwrap_or("./uploads".to_string());
        let max_upload_size = env::var("MAX_UPLOAD_SIZE_MB")
            .unwrap_or("10".to_string())
            .parse::<usize>()
            .expect("Failed to parse environment variable MAX_UPLOAD_SIZE_MB")
            * 1_048_576; // Mebibytes to bytes

        let rate_limit_per_minute = env::var("RATE_LIMIT_PER_MINUTE")
            .unwrap_or("30".to_string())
            .parse::<u64>()
            .expect("Failed to parse RATE_LIMIT_PER_MINUTE");

        let rate_limit_burst_size = env::var("RATE_LIMIT_BURST_SIZE")
            .unwrap_or("40".to_string())
            .parse::<u32>()
            .expect("Failed to parse RATE_LIMIT_BURST_SIZE");

        let smtp_server =
            env::var("SMTP_SERVER").expect("Environment variable SMTP_SERVER must be set");
        // builder_dangerous disables TLS and authentication, this is fine because we are only connecting to a localhost relay server.
        let transport =
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(smtp_server).build();
        let mailer = Arc::new(SmtpMailer { transport });

        Self {
            listening_ip,
            listening_port,
            hostname,
            base_url,
            database_url,
            log_level,
            bcrypt_cost,
            session_duration,
            verification_timer,
            upload_dir,
            max_upload_size,
            mailer,
            rate_limit_per_minute,
            rate_limit_burst_size,
        }
    }
}

/// Parses the `BASE_URL` environment variable similar to Caddy's rules, and returns the hostname and base url.
///
/// # Parameters
/// `base_url` - the base URL, optionally specifying each of:
/// - The scheme, with SCHEME:// at the start (defaults to HTTPS if absent)
/// - The hostname (defaults to localhost if absent)
/// - The port, with :PORT at the end (defaults to blank if absent)
///
/// # Returns
/// A tuple, containing:
/// - The base URL, which is the start of the URL to be used to point towards a page on the website,
/// - The hostname, the name of the website
fn parse_base_url(base_url: &str) -> (String, String) {
    let (scheme, rest) = match base_url.split_once("://") {
        None => ("https", base_url),
        Some(split) => split,
    };
    let rest = if rest.is_empty() { "localhost" } else { rest };
    match rest.split_once(':') {
        None => (format!("{scheme}://{rest}"), rest.to_owned()),
        Some((hostname, port)) => {
            let hostname = if hostname.is_empty() {
                "localhost"
            } else {
                hostname
            };
            (format!("{scheme}://{hostname}:{port}"), hostname.to_owned())
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_base_url() {
        assert_eq!(
            super::parse_base_url("http://localhost"),
            ("http://localhost".to_owned(), "localhost".to_owned())
        );
        assert_eq!(
            super::parse_base_url("http://localhost:3000"),
            ("http://localhost:3000".to_owned(), "localhost".to_owned())
        );
        assert_eq!(
            super::parse_base_url("https://example.com"),
            ("https://example.com".to_owned(), "example.com".to_owned())
        );
        assert_eq!(
            super::parse_base_url("sub.example.com"),
            (
                "https://sub.example.com".to_owned(),
                "sub.example.com".to_owned()
            )
        );
        assert_eq!(
            super::parse_base_url("localhost"),
            ("https://localhost".to_owned(), "localhost".to_owned())
        );
        assert_eq!(
            super::parse_base_url("localhost:3000"),
            ("https://localhost:3000".to_owned(), "localhost".to_owned())
        );
        assert_eq!(
            super::parse_base_url("http://:3000"),
            ("http://localhost:3000".to_owned(), "localhost".to_owned())
        );
    }
}
