use std::{sync::Arc, time::Duration};

use tokio::sync::Mutex;

use surveillance_domain::*;
use track_engine::TrackEngine;
use websocket_server::WebSocketServer;
use ws_broadcaster::{RadarBroadcaster, RadarMessage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let broadcaster = Arc::new(RadarBroadcaster::new());
    let track_engine = Arc::new(Mutex::new(TrackEngine::new()));

    let server = WebSocketServer::new(Arc::clone(&broadcaster));

    let publisher = Arc::clone(&broadcaster);
    let engine = Arc::clone(&track_engine);

    tokio::spawn(async move {
        loop {
            let observation = Observation::new(
                AircraftIdentifier::new("40621D"),
                Position::new(51.4706, -0.4619),
                Altitude::new(38000.0),
                Velocity::new(250.0, 180.0, 0.0),
                SignalQuality::High,
            );

            let mut engine = engine.lock().await;
            let track = engine.process(observation);

            publisher.publish(RadarMessage::TrackUpdated(track.observation.clone()));

            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    });

    server.run("127.0.0.1:8080").await?;

    Ok(())
}
