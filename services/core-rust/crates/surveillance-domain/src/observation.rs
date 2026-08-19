use serde::{Deserialize, Serialize};

use crate::{
    AircraftIdentifier, Altitude, Identifier, Position, SignalQuality, Timestamp, Velocity,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub id: Identifier,
    pub aircraft_id: AircraftIdentifier,
    pub timestamp: Timestamp,
    pub position: Position,
    pub altitude: Altitude,
    pub velocity: Velocity,
    pub quality: SignalQuality,
}

impl Observation {
    pub fn new(
        aircraft_id: AircraftIdentifier,
        position: Position,
        altitude: Altitude,
        velocity: Velocity,
        quality: SignalQuality,
    ) -> Self {
        Self {
            id: Identifier::new(),
            aircraft_id,
            timestamp: Timestamp::now(),
            position,
            altitude,
            velocity,
            quality,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn creates_observation() {
        let observation = Observation::new(
            AircraftIdentifier::new("40621D"),
            Position::new(4.0, 8.0),
            Altitude::new(10000.0),
            Velocity::new(220.0, 180.0, 0.0),
            SignalQuality::High,
        );

        assert_eq!(observation.aircraft_id.value(), "40621D");

        assert_eq!(observation.position.latitude, 4.0);
        assert_eq!(observation.altitude.value, 10000.0);
        assert_eq!(observation.quality, SignalQuality::High);
    }
}
