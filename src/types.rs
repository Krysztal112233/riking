use serde::{Deserialize, Serialize};

pub mod v1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub time: f64,

    pub result: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct VikingUri(String);

impl VikingUri {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for VikingUri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for VikingUri {
    fn from(s: &str) -> Self {
        if s.starts_with("viking://") {
            Self(s.to_string())
        } else {
            let trimmed = s.trim_start_matches('/');
            Self(format!("viking://{trimmed}"))
        }
    }
}

impl From<String> for VikingUri {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}
