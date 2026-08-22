use surveillance_domain::{Altitude, Observation, Position, SignalQuality, Velocity};

use crate::{
    cat021::{Cat021Cursor, Cat021Data},
    error::AsterixError,
    packet::AsterixPacket,
};

pub fn decode_cat021(packet: AsterixPacket) -> Result<Observation, AsterixError> {
    if packet.category() != 21 {
        return Err(AsterixError::UnsupportedCategory);
    }

    let mut cursor = Cat021Cursor::new(packet.payload().clone());

    let cat = Cat021Data::decode(&mut cursor)?;

    Ok(Observation::new(
        cat.target_address,
        Position::new(0.0, 0.0),
        Altitude::new(0.0),
        Velocity::new(0.0, 0.0, 0.0),
        SignalQuality::High,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    #[test]
    fn converts_packet_to_observation() {
        let packet =
            AsterixPacket::from_bytes(Bytes::from_static(&[21, 0, 6, 0x40, 0x62, 0x1D])).unwrap();

        let observation = decode_cat021(packet).unwrap();

        assert_eq!(observation.aircraft_id.value(), "40621D");
    }
}
