use bytes::Bytes;
use surveillance_domain::AircraftIdentifier;

use crate::error::AsterixError;

/// Sequential CAT021 payload reader.
pub struct Cat021Cursor {
    data: Bytes,
    offset: usize,
}

impl Cat021Cursor {
    pub fn new(data: Bytes) -> Self {
        Self { data, offset: 0 }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.offset)
    }

    pub fn read_u8(&mut self) -> Result<u8, AsterixError> {
        let value = *self
            .data
            .get(self.offset)
            .ok_or(AsterixError::PacketTooShort)?;

        self.offset += 1;
        Ok(value)
    }

    pub fn read_u16(&mut self) -> Result<u16, AsterixError> {
        let a = self.read_u8()?;
        let b = self.read_u8()?;
        Ok(u16::from_be_bytes([a, b]))
    }

    pub fn read_u24(&mut self) -> Result<u32, AsterixError> {
        let a = self.read_u8()? as u32;
        let b = self.read_u8()? as u32;
        let c = self.read_u8()? as u32;

        Ok((a << 16) | (b << 8) | c)
    }

    pub fn read_bytes(&mut self, length: usize) -> Result<Bytes, AsterixError> {
        if self.remaining() < length {
            return Err(AsterixError::PacketTooShort);
        }

        let end = self.offset + length;
        let bytes = self.data.slice(self.offset..end);

        self.offset = end;

        Ok(bytes)
    }
}

/// Decoded CAT021 fields (MVP).
#[derive(Debug, Clone)]
pub struct Cat021Data {
    pub target_address: AircraftIdentifier,
}

impl Cat021Data {
    /// Decode the ICAO Target Address (I021/080).
    pub fn decode(cursor: &mut Cat021Cursor) -> Result<Self, AsterixError> {
        let address = cursor.read_u24()?;

        Ok(Self {
            target_address: AircraftIdentifier::new(format!("{:06X}", address)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_target_address() {
        let payload = Bytes::from_static(&[0x40, 0x62, 0x1D]);

        let mut cursor = Cat021Cursor::new(payload);

        let data = Cat021Data::decode(&mut cursor).unwrap();

        assert_eq!(data.target_address.value(), "40621D");
    }

    #[test]
    fn cursor_reads_u24() {
        let payload = Bytes::from_static(&[0x40, 0x62, 0x1D]);

        let mut cursor = Cat021Cursor::new(payload);

        assert_eq!(cursor.read_u24().unwrap(), 0x40621D);
        assert_eq!(cursor.offset(), 3);
    }
}
