use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statistics {
    total_distance_drawn : f32
}

impl Statistics {
    pub fn load_from_file(path : &str) -> Result<Self, syact::Error> {
        Ok(serde_json::from_str(
            std::fs::read_to_string(path)?.as_str()
        )?)
    }

    // pub fn 
}