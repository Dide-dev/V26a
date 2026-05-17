/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

//! This module parses all external configuration from environment variables.
//! For more details, see the '.env.example' file in the root of this repository.

use crate::api_v1::session::Keys;
use crate::errors::ApiError;
use async_trait::async_trait;
use bcrypt::DEFAULT_COST;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use std::env;
use std::sync::Arc;

/// Generic definition of a mailer, so that the mailer can be replaced with a [`AsyncStubTransport`](lettre::transport::stub::AsyncStubTransport) during testing.
#[async_trait]
pub trait Mailer: Send + Sync {
    async fn send(&self, message: Message) -> Result<(), ApiError>;
}

/// Mailer to be used in the program, sends mails as usual.
pub struct SmtpMailer {
    transport: AsyncSmtpTransport<Tokio1Executor>,
}

#[async_trait]
impl Mailer for SmtpMailer {
    async fn send(&self, message: Message) -> Result<(), ApiError> {
        self.transport.send(message).await?;
        Ok(())
    }
}

/// For more details, see the '.env.example' file in the root of this repository.
#[derive(Clone)]
pub struct Config {
    pub listening_ip: String,
    pub listening_port: u16,
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
    pub jwt_keys: Keys,

    /// Upload images directory
    pub upload_dir: String,
    pub max_upload_size: usize,

    pub mailer: Arc<dyn Mailer>,
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
            .expect("Environment variable VERIFICATION_TIMER_SECONDS must be set")
            .parse()
            .expect("Failed to parse environment variable VERIFICATION_TIMER_SECONDS");
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

        let smtp_url = env::var("SMTP_URL").expect("SMTP environment variables must be set");
        let transport = AsyncSmtpTransport::<Tokio1Executor>::from_url(&smtp_url)
            .expect("Failed to create AsyncSmtpTransport.")
            .build();
        let mailer = Arc::new(SmtpMailer { transport });

        Self {
            listening_ip,
            listening_port,
            base_url,
            database_url,
            log_level,
            bcrypt_cost,
            session_duration,
            verification_timer,
            jwt_keys,
            upload_dir,
            max_upload_size,
            mailer,
        }
    }
}
