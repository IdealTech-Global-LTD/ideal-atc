use std::collections::HashMap;

use surveillance_domain::{AircraftIdentifier, Observation};

use crate::state::TrackState;

use chrono::{Duration, Utc};
const TRACK_TIMEOUT_SECONDS: i64 = 60;

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

    pub fn remove_stale_tracks(&mut self) {
        let now = Utc::now();

        self.tracks
            .retain(|_, track| now - track.last_seen < Duration::seconds(TRACK_TIMEOUT_SECONDS));
    }

    pub fn active_tracks(&self) -> Vec<&TrackState> {
        self.tracks.values().collect()
    }

    // pub fn process(&mut self, observation: Observation) {
    //     let key = observation.aircraft_id.clone();

    //     match self.tracks.get_mut(&key) {
    //         Some(track) => track.update(observation),
    //         None => {
    //             self.tracks.insert(key, TrackState::new(observation));
    //         }
    //     }
    // }

    pub fn process(&mut self, observation: Observation) -> &TrackState {
        let key = observation.aircraft_id.clone();

        if let Some(track) = self.tracks.get_mut(&key) {
            track.update(observation);
        } else {
            self.tracks
                .insert(key.clone(), TrackState::new(observation));
        }

        self.tracks.get(&key).unwrap()
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

    #[test]
    fn removes_stale_track() {
        use chrono::Duration;

        let mut engine = TrackEngine::new();

        engine.process(sample_observation());

        let aircraft = AircraftIdentifier::new("40621D");

        let track = engine.tracks.get_mut(&aircraft).unwrap();
        track.last_seen -= Duration::seconds(61);

        engine.remove_stale_tracks();

        assert_eq!(engine.total_tracks(), 0);
    }
}
