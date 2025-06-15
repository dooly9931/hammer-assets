use serde::{Deserialize, Serialize};

/// Ping response from CAM API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PongResponse {
    pub pong: String,
}

/// V3 API error response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct V3Error {
    pub code: String,
    pub message: String,
}
