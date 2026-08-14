use serde::{Deserialize, Serialize};

use crate::{Altitude, Identifier, Position, SignalQuality, Timestamp, Velocity};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub id: Identifier,
    pub timestamp: Timestamp,
    pub position: Position,
    pub altitude: Altitude,
    pub velocity: Velocity,
    pub quality: SignalQuality,
}

impl Observation {
    pub fn new(
        position: Position,
        altitude: Altitude,
        velocity: Velocity,
        quality: SignalQuality,
    ) -> Self {
        Self {
            id: Identifier::new(),
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
            Position::new(4.0, 8.0),
            Altitude::new(10000.0),
            Velocity::new(220.0, 180.0, 0.0),
            SignalQuality::High,
        );

        assert_eq!(observation.position.latitude, 4.0);
        assert_eq!(observation.altitude.value, 10000.0);
        assert_eq!(observation.quality, SignalQuality::High);
    }
}
