use surveillance_domain::{
    AircraftIdentifier, Altitude, Observation, Position, SignalQuality, Velocity,
};

use crate::{frame::AdsbFrame, parser::parse};

/// Converts a parsed ADS-B frame into an Observation.
///
/// Currently a placeholder until ICAO ADS-B decoding is implemented.
pub fn decode(frame: AdsbFrame) -> Option<Observation> {
    let message = parse(&frame)?;
    let aircraft = AircraftIdentifier::new(message.icao);

    let position = Position::new(message.latitude?, message.longitude?);

    let altitude = Altitude::new(message.altitude? as f64);

    let velocity = Velocity::new(message.ground_speed? as f64, message.track? as f64, 0.0);

    Some(Observation::new(
        aircraft,
        position,
        altitude,
        velocity,
        SignalQuality::Unknown,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    use crate::frame::AdsbFrame;

    #[test]
    fn decodes_sbs_position_message() {
        let frame = AdsbFrame::new(Bytes::from_static(
            b"MSG,3,111,11111,40621D,111111,2026/08/19,09:30:12.000,2026/08/19,09:30:12.000,BAW123,38000,250,180,51.4706,-0.4619,,,,,,",
        ));

        let observation = decode(frame).expect("should decode");

        assert_eq!(observation.aircraft_id.value(), "40621D");
        assert_eq!(observation.position.latitude, 51.4706);
        assert_eq!(observation.position.longitude, -0.4619);
        assert_eq!(observation.altitude.value, 38000.0);
        assert_eq!(observation.velocity.ground_speed, 250.0);
        assert_eq!(observation.velocity.heading, 180.0);
    }

    #[test]
    fn rejects_invalid_frame() {
        let frame = AdsbFrame::new(Bytes::new());

        assert!(decode(frame).is_none());
    }
}
