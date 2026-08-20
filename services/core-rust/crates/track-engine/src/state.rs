use surveillance_domain::Observation;

#[derive(Debug, Clone)]
pub struct TrackState {
    pub observation: Observation,
    pub updates: u64,
}

impl TrackState {
    pub fn new(observation: Observation) -> Self {
        Self {
            observation,
            updates: 1,
        }
    }

    pub fn update(&mut self, observation: Observation) {
        self.observation = observation;
        self.updates += 1;
    }
}
