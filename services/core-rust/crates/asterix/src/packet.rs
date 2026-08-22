use crate::error::AsterixError;
use bytes::Bytes;

#[derive(Debug, Clone)]

pub struct AsterixPacket {
    category: u8,
    length: u16,
    payload: Bytes,
}

impl AsterixPacket {
    pub fn from_bytes(bytes: Bytes) -> Result<Self, AsterixError> {
        if bytes.len() < 3 {
            return Err(AsterixError::PacketTooShort);
        }

        let category = bytes[0];

        let length = u16::from_be_bytes([bytes[1], bytes[2]]);

        if length as usize != bytes.len() {
            return Err(AsterixError::InvalidLength);
        }

        Ok(Self {
            category,
            length,
            payload: bytes.slice(3..),
        })
    }

    pub fn category(&self) -> u8 {
        self.category
    }

    pub fn length(&self) -> u16 {
        self.length
    }

    pub fn payload(&self) -> &Bytes {
        &self.payload
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    #[test]
    fn parses_header() {
        let packet =
            AsterixPacket::from_bytes(Bytes::from_static(&[21, 0, 7, 0xAA, 0xBB, 0xCC, 0xDD]))
                .unwrap();

        assert_eq!(packet.category(), 21);
        assert_eq!(packet.length(), 7);
        assert_eq!(packet.payload().len(), 4);
    }

    #[test]
    fn rejects_short_packet() {
        let result = AsterixPacket::from_bytes(Bytes::from_static(&[21, 0]));

        assert!(result.is_err());
    }
}
