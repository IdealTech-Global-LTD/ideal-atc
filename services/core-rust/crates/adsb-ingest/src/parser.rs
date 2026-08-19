use crate::frame::AdsbFrame;

//validate incomming ADS-B frame
pub fn validate(frame: &AdsbFrame) -> bool {
    !frame.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    #[test]
    fn accepts_non_empty_frame() {
        let frame = AdsbFrame::new(Bytes::from_static(b"8D40621D"));

        assert!(validate(&frame));
    }

    #[test]
    fn rejects_empty_frame() {
        let frame = AdsbFrame::new(Bytes::new());

        assert!(!validate(&frame));
    }
}
