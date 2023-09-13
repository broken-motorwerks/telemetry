use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EngineData {
    pub rpm: f32,
}

#[allow(warnings)]
pub mod messages;
