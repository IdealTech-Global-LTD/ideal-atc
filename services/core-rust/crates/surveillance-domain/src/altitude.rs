use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Altitude {
    pub value: f64,
}

impl Altitude {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_altitude() {
        let altitude = Altitude::new(12000.0);

        assert_eq!(altitude.value, 12000.0);
    }
}
