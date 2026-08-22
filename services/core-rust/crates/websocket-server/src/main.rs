use std::{sync::Arc, time::Duration};

use surveillance_domain::*;
use websocket_server::WebSocketServer;
use ws_broadcaster::{RadarBroadcaster, RadarMessage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let broadcaster = Arc::new(RadarBroadcaster::new());

    let server = WebSocketServer::new(Arc::clone(&broadcaster));

    let publisher = Arc::clone(&broadcaster);

    tokio::spawn(async move {
        //fake aircraft data
        loop {
            let observation = Observation::new(
                AircraftIdentifier::new("40621D"),
                Position::new(51.4706, -0.4619),
                Altitude::new(38000.0),
                Velocity::new(250.0, 180.0, 0.0),
                SignalQuality::High,
            );

            publisher.publish(RadarMessage::Observation(observation));

            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    });

    server.run("127.0.0.1:8080").await?;

    Ok(())
}
