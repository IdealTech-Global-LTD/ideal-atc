use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Velocity {
    pub ground_speed: f64,
    pub heading: f64,
    pub vertical_rate: f64,
}

impl Velocity {
    pub fn new(ground_speed: f64, heading: f64, vertical_rate: f64) -> Self {
        Self {
            ground_speed,
            heading,
            vertical_rate,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_velocity() {
        let velocity = Velocity::new(250.0, 90.0, 500.0);

        assert_eq!(velocity.ground_speed, 250.0);
        assert_eq!(velocity.heading, 90.0);
        assert_eq!(velocity.vertical_rate, 500.0);
    }
}
