use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CheckView {
    pub is_valid: bool,
    pub valid_position: Vec<u32>
}