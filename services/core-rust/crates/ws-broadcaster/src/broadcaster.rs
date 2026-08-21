use crate::message::RadarMessage;
use tokio::sync::broadcast;

pub struct RadarBroadcaster {
    tx: broadcast::Sender<RadarMessage>,
}

impl Default for RadarBroadcaster {
    fn default() -> Self {
        Self::new()
    }
}

impl RadarBroadcaster {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(1024);
        Self { tx }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<RadarMessage> {
        self.tx.subscribe()
    }

    pub fn publish(&self, message: RadarMessage) {
        let _ = self.tx.send(message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use surveillance_domain::*;

    #[tokio::test]
    async fn broadcasts_observation() {
        let broadcaster = RadarBroadcaster::new();

        let mut client = broadcaster.subscribe();

        let observation = Observation::new(
            AircraftIdentifier::new("40621D"),
            Position::new(51.4706, -0.4619),
            Altitude::new(38000.0),
            Velocity::new(250.0, 180.0, 0.0),
            SignalQuality::High,
        );

        broadcaster.publish(RadarMessage::Observation(observation.clone()));

        let received = client.recv().await.unwrap();

        match received {
            RadarMessage::Observation(obs) => {
                assert_eq!(obs.aircraft_id.value(), "40621D");
            }
        }
    }
}
