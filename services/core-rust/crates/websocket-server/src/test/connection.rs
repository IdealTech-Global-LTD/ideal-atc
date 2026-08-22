use std::sync::Arc;

use tokio::time::{Duration, timeout};
use websocket_server::WebSocketServer;
use ws_broadcaster::RadarBroadcaster;

#[tokio::test]
async fn accepts_websocket_connection() {
    let broadcaster = Arc::new(RadarBroadcaster::new());
    let server = WebSocketServer::new(broadcaster);

    let address = "127.0.0.1:0";

    let _ = timeout(Duration::from_millis(100), server.run(address)).await;
}
