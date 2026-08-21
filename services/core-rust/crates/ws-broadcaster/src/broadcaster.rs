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
