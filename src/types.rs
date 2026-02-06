use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MouseEvent {
    pub event_type: String,
    pub x: i32,
    pub y: i32,
    pub timestamp: u64,
    pub target: Option<String>,
    pub button: Option<String>,
    pub scroll_y: Option<i32>,
}
