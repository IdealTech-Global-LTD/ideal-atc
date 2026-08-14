use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Timestamp(pub DateTime<Utc>);

impl Timestamp {
    pub fn now() -> Self {
        Self(Utc::now())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_timestamp() {
        let timestamp = Timestamp::now();

        assert!(timestamp.0.timestamp() > 0);
    }
}
