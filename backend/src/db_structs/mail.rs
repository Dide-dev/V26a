use crate::errors::ApiError;
use async_trait::async_trait;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

/// Generic definition of a mailer, so that the mailer can be replaced with a [`AsyncStubTransport`](lettre::transport::stub::AsyncStubTransport) during testing.
#[async_trait]
pub trait Mailer: Send + Sync {
    async fn send(&self, message: Message) -> Result<(), ApiError>;
}

/// Mailer to be used in the program, sends mails as usual.
pub struct SmtpMailer {
    pub transport: AsyncSmtpTransport<Tokio1Executor>,
}

#[async_trait]
impl Mailer for SmtpMailer {
    async fn send(&self, message: Message) -> Result<(), ApiError> {
        self.transport.send(message).await?;
        Ok(())
    }
}

#[cfg(test)]
/// Mailer to be used for tests. Does not mail.
pub struct StubMailer {
    pub transport: lettre::transport::stub::AsyncStubTransport,
}

#[cfg(test)]
#[async_trait]
impl Mailer for StubMailer {
    async fn send(&self, message: Message) -> Result<(), ApiError> {
        self.transport.send(message).await.unwrap();
        Ok(())
    }
}
