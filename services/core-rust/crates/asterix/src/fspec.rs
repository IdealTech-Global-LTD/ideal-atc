use bytes::Bytes;

use crate::error::AsterixError;

#[derive(Debug, Clone)]
pub struct Fspec {
    bytes: Vec<u8>,
    consumed: usize,
}

impl Fspec {
    pub fn parse(payload: &Bytes) -> Result<Self, AsterixError> {
        let mut bytes = Vec::new();
        let mut index = 0;

        loop {
            let byte = *payload.get(index).ok_or(AsterixError::PacketTooShort)?;

            bytes.push(byte);
            index += 1;

            // FX bit = last bit
            if byte & 0x01 == 0 {
                break;
            }
        }

        Ok(Self {
            bytes,
            consumed: index,
        })
    }

    pub fn consumed(&self) -> usize {
        self.consumed
    }

    pub fn is_present(&self, bit: usize) -> bool {
        if bit == 0 {
            return false;
        }

        let byte_index = (bit - 1) / 7;
        let bit_in_byte = (bit - 1) % 7;

        let Some(byte) = self.bytes.get(byte_index) else {
            return false;
        };

        let mask = 1 << (7 - bit_in_byte);

        byte & mask != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    #[test]
    fn parses_single_fspec() {
        let payload = Bytes::from_static(&[0b1110_0000]);

        let fspec = Fspec::parse(&payload).unwrap();

        assert_eq!(fspec.consumed(), 1);

        assert!(fspec.is_present(1));
        assert!(fspec.is_present(2));
        assert!(fspec.is_present(3));
        assert!(!fspec.is_present(4));
    }

    #[test]
    fn parses_extended_fspec() {
        let payload = Bytes::from_static(&[0b1000_0001, 0b0100_0000]);

        let fspec = Fspec::parse(&payload).unwrap();

        assert_eq!(fspec.consumed(), 2);

        assert!(fspec.is_present(1));
        assert!(fspec.is_present(9));
    }
}
