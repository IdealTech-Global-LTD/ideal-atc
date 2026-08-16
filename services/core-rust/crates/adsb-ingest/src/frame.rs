use bytes::Bytes;
#[derive(Debug, Clone, PartialEq)]
pub struct AdsbFrame {
    payload: Bytes,
}

impl AdsbFrame {
    pub fn new(payload: Bytes) -> Self {
        Self { payload }
    }

    pub fn payload(&self) -> &Bytes {
        &self.payload()
    }

    pub fn len(&self) -> usize {
        self.payload().len()
    }

    pub fn is_empty(&self) -> bool {
        self.payload.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    #[test]
    fn creates_frame() {
        let frame = AdsbFrame::new(Bytes::from_static(b"8D40621D"));

        assert_eq!(frame.len(), 8);
        assert!(!frame.is_empty());
    }
}
