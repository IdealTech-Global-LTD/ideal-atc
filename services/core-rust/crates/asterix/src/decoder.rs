use surveillance_domain::{Observation, SignalQuality};

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

    let data = Cat021Data::decode(&mut cursor)?;

    Ok(Observation::new(
        data.target_address,
        data.position,
        data.altitude,
        data.velocity,
        SignalQuality::High,
    ))
}
#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    #[test]
    fn decodes_aircraft_identifier() {
        let packet = AsterixPacket::from_bytes(Bytes::from_static(&[
            21, // CAT021
            0, 6, // Total packet length = 6 bytes
            0x40, 0x62, 0x1D, // ICAO address
        ]))
        .unwrap();

        let observation = decode_cat021(packet).unwrap();

        assert_eq!(observation.aircraft_id.value(), "40621D");
    }
}
