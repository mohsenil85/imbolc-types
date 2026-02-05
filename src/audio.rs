//! Audio-related types shared across crates.

use serde::{Deserialize, Serialize};

/// SuperCollider server status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ServerStatus {
    #[default]
    Stopped,
    Starting,
    Running,
    Connected,
    Error,
}
