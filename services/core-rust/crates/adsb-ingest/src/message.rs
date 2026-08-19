#[derive(Debug, Clone, PartialEq)]
pub struct RawMessage {
    pub message_type: String,
    pub transmission_type: u8,
    pub icao: String,
    pub callsign: Option<String>,
    pub altitude: Option<i32>,
    pub ground_speed: Option<f32>,
    pub track: Option<f32>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}
