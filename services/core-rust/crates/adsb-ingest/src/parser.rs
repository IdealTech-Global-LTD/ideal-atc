// use crate::frame::AdsbFrame;

// //validate incomming ADS-B frame
// pub fn validate(frame: &AdsbFrame) -> bool {
//     !frame.is_empty()
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use bytes::Bytes;

//     #[test]
//     fn accepts_non_empty_frame() {
//         let frame = AdsbFrame::new(Bytes::from_static(b"8D40621D"));

//         assert!(validate(&frame));
//     }

//     #[test]
//     fn rejects_empty_frame() {
//         let frame = AdsbFrame::new(Bytes::new());

//         assert!(!validate(&frame));
//     }
// }

use crate::{frame::AdsbFrame, message::RawMessage};

pub fn parse(frame: &AdsbFrame) -> Option<RawMessage> {
    let text = std::str::from_utf8(frame.payload()).ok()?;

    let fields: Vec<&str> = text.trim().split(',').collect();

    if fields.len() < 22 {
        return None;
    }

    Some(RawMessage {
        message_type: fields[0].to_string(),
        transmission_type: fields[1].parse().ok()?,
        icao: fields[4].to_string(),
        callsign: optional_string(fields[10]),
        altitude: optional_i32(fields[11]),
        ground_speed: optional_f32(fields[12]),
        track: optional_f32(fields[13]),
        latitude: optional_f64(fields[14]),
        longitude: optional_f64(fields[15]),
    })
}

fn optional_string(value: &str) -> Option<String> {
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn optional_i32(value: &str) -> Option<i32> {
    value.parse().ok()
}

fn optional_f32(value: &str) -> Option<f32> {
    value.parse().ok()
}

fn optional_f64(value: &str) -> Option<f64> {
    value.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    use crate::frame::AdsbFrame;

    #[test]
    fn parses_sbs_message() {
        let frame = AdsbFrame::new(Bytes::from_static(
            b"MSG,3,111,11111,40621D,111111,2026/08/19,09:30:12.000,2026/08/19,09:30:12.000,BAW123,38000,250,180,51.4706,-0.4619,,,,,,",
        ));

        let message = parse(&frame).unwrap();

        assert_eq!(message.message_type, "MSG");
        assert_eq!(message.transmission_type, 3);
        assert_eq!(message.icao, "40621D");
        assert_eq!(message.callsign.unwrap(), "BAW123");
        assert_eq!(message.altitude.unwrap(), 38000);
        assert_eq!(message.ground_speed.unwrap(), 250.0);
        assert_eq!(message.track.unwrap(), 180.0);
    }
}
