use std::collections::HashMap;

use surveillance_domain::{AircraftIdentifier, Observation};

use crate::state::TrackState;

pub struct TrackEngine {
    tracks: HashMap<AircraftIdentifier, TrackState>,
}

impl Default for TrackEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl TrackEngine {
    pub fn new() -> Self {
        Self {
            tracks: HashMap::new(),
        }
    }

    pub fn process(&mut self, observation: Observation) {
        let key = observation.aircraft_id.clone();

        match self.tracks.get_mut(&key) {
            Some(track) => track.update(observation),
            None => {
                self.tracks.insert(key, TrackState::new(observation));
            }
        }
    }

    pub fn total_tracks(&self) -> usize {
        self.tracks.len()
    }

    pub fn get(&self, aircraft: &AircraftIdentifier) -> Option<&TrackState> {
        self.tracks.get(aircraft)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use surveillance_domain::*;

    fn sample_observation() -> Observation {
        Observation::new(
            AircraftIdentifier::new("40621D"),
            Position::new(51.4706, -0.4619),
            Altitude::new(38000.0),
            Velocity::new(250.0, 180.0, 0.0),
            SignalQuality::High,
        )
    }

    #[test]
    fn creates_new_track() {
        let mut engine = TrackEngine::new();

        engine.process(sample_observation());

        assert_eq!(engine.total_tracks(), 1);
    }

    #[test]
    fn updates_existing_track() {
        let mut engine = TrackEngine::new();

        engine.process(sample_observation());
        engine.process(sample_observation());

        let aircraft = AircraftIdentifier::new("40621D");

        let track = engine.get(&aircraft).unwrap();

        assert_eq!(track.updates, 2);
        assert_eq!(engine.total_tracks(), 1);
    }
}
