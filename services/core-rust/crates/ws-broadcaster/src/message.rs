use serde::{Deserialize, Serialize};
use surveillance_domain::Observation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RadarMessage {
    Connected,
    TrackUpdated(Observation),
}
