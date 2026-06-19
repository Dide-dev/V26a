/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/
use axum::body::Body;
use axum::extract::ConnectInfo;
use axum::http::Request;
use governor::clock::QuantaInstant;
use governor::middleware::NoOpMiddleware;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::key_extractor::KeyExtractor;
use tower_governor::{GovernorError, GovernorLayer};

/// A struct specifying the options needed to make a ratelimit layer for a router.
///
/// A user (determined by the extracted key) starts off with `burst_size` requests, and every
/// minute they regain `refresh_rate` requests. When they are out of requests, a "Ratelimit
/// exceeded" error will be returned.
#[derive(Clone)]
pub struct RatelimitOptions {
    /// The amount of requests that should be regained per minute. Used to determine the duration
    /// of time between each refill.
    pub refresh_rate: u64,
    /// The starting amount of requests a user can send to a router.
    pub burst_size: u32,
}

#[derive(Clone)]
pub struct IpKeyExtractor;

impl KeyExtractor for IpKeyExtractor {
    type Key = String;

    fn extract<T>(&self, req: &Request<T>) -> Result<Self::Key, GovernorError> {
        // Try XFF header to identify ip address.
        if let Some(res) = req
            .headers()
            .get("X-Forwarded-For")
            .and_then(|header: &axum::http::HeaderValue| header.to_str().ok())
        {
            return Ok(res.to_string());
        }

        // Try socket address (TCP connection), if the XFF fails.
        if let Some(ConnectInfo(addr)) = req.extensions().get::<ConnectInfo<SocketAddr>>() {
            return Ok(addr.ip().to_string());
        }

        // Mockup for tests, where no TCP connection exists and reject if no IP found.
        if cfg!(test) {
            return Ok("127.0.0.1".to_string());
        }
        Err(GovernorError::UnableToExtractKey)
    }
}

impl RatelimitOptions {
    pub fn to_layer(&self) -> GovernorLayer<IpKeyExtractor, NoOpMiddleware<QuantaInstant>, Body> {
        let config = Arc::new(
            GovernorConfigBuilder::default()
                .period(Duration::from_millis(60_000 / self.refresh_rate))
                .burst_size(self.burst_size)
                .key_extractor(IpKeyExtractor)
                .finish()
                .unwrap(),
        );
        GovernorLayer::new(config)
    }
}

// --- Auth ---
pub const LOGIN_RATELIMIT: RatelimitOptions = RatelimitOptions {
    refresh_rate: 3,
    burst_size: 5,
};

// --- Classrooms ---
pub const CREATE_CLASSROOM_RATELIMIT: RatelimitOptions = RatelimitOptions {
    refresh_rate: 10,
    burst_size: 10,
};
pub const ADD_LESSON_RATELIMIT: RatelimitOptions = RatelimitOptions {
    refresh_rate: 10,
    burst_size: 10,
};
pub const CLASSROOM_LOGIN_RATELIMIT: RatelimitOptions = RatelimitOptions {
    refresh_rate: 100,
    burst_size: 200,
};

// --- Feedback ---
pub const PROVIDE_FEEDBACK_RATELIMIT: RatelimitOptions = RatelimitOptions {
    refresh_rate: 5,
    burst_size: 50,
};

// --- Images ---
pub const ADD_IMAGE_RATELIMIT: RatelimitOptions = RatelimitOptions {
    refresh_rate: 10,
    burst_size: 20,
};

// --- Trees ---
pub const GET_TREE_LIST_RATELIMIT: RatelimitOptions = RatelimitOptions {
    refresh_rate: 150,
    burst_size: 250,
};
pub const ADD_TREE_RATELIMIT: RatelimitOptions = RatelimitOptions {
    refresh_rate: 10,
    burst_size: 20,
};
pub const DUPLICATE_TREE_RATELIMIT: RatelimitOptions = RatelimitOptions {
    refresh_rate: 5,
    burst_size: 10,
};
pub const ADD_CARD_RATELIMIT: RatelimitOptions = RatelimitOptions {
    refresh_rate: 25,
    burst_size: 50,
};
pub const GET_EDIT_LOCK_RATELIMIT: RatelimitOptions = RatelimitOptions {
    refresh_rate: 15,
    burst_size: 30,
};

// --- Users ---
pub const ADD_USER_RATELIMIT: RatelimitOptions = RatelimitOptions {
    refresh_rate: 2,
    burst_size: 5,
};
pub const VERIFY_USER_RATELIMIT: RatelimitOptions = RatelimitOptions {
    refresh_rate: 2,
    burst_size: 5,
};
