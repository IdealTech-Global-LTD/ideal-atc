use chrono::{DateTime, Utc};
use surveillance_domain::Observation;

#[derive(Debug, Clone)]
pub struct TrackState {
    pub observation: Observation,
    pub updates: u64,
    pub last_seen: DateTime<Utc>,
}

impl TrackState {
    pub fn new(observation: Observation) -> Self {
        Self {
            observation,
            updates: 1,
            last_seen: Utc::now(),
        }
    }

    pub fn update(&mut self, observation: Observation) {
        self.observation = observation;
        self.updates += 1;
        self.last_seen = Utc::now();
    }
}
