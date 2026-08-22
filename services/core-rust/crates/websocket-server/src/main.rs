
use std::sync::Arc;

use websocket_server::WebSocketServer;
use ws_broadcaster::RadarBroadcaster;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let broadcaster = Arc::new(RadarBroadcaster::new());
    let server = WebSocketServer::new(broadcaster);

    server.run("127.0.0.1:8080").await?;

    Ok(())
}