use serde::{Deserialize, Serialize};

use crate::{Altitude, Identifier, Position, Timestamp, Velocity};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TrackStatus {
    Active,
    Lost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: Identifier,
    pub position: Position,
    pub altitude: Altitude,
    pub velocity: Velocity,
    pub status: TrackStatus,
    pub last_update: Timestamp,
}

impl Track {
    pub fn from_observation(observation: &crate::Observation) -> Self {
        Self {
            id: Identifier::new(),
            position: observation.position,
            altitude: observation.altitude,
            velocity: observation.velocity,
            status: TrackStatus::Active,
            last_update: observation.timestamp.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn creates_track_from_observation() {
        let observation = Observation::new(
            Position::new(4.5, 8.1),
            Altitude::new(15000.0),
            Velocity::new(300.0, 270.0, -700.0),
            SignalQuality::Medium,
        );

        let track = Track::from_observation(&observation);

        assert_eq!(track.position, observation.position);
        assert_eq!(track.altitude, observation.altitude);
        assert_eq!(track.velocity, observation.velocity);
        assert_eq!(track.status, TrackStatus::Active);
    }
}
