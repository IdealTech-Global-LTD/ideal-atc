use surveillance_domain::Observation;

use crate::{frame::AdsbFrame, parser::parse};

/// Converts a validated ADS-B frame into an Observation.
///
/// Currently a placeholder until ICAO ADS-B decoding is implemented.
pub fn decode(frame: AdsbFrame) -> Option<Observation> {
    let _message = parse(&frame)?;

    // TODO: Decode ICAO ADS-B message into Observation
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    #[test]
    fn returns_none_for_empty_frame() {
        let frame = AdsbFrame::new(Bytes::new());

        assert!(decode(frame).is_none());
    }
}
