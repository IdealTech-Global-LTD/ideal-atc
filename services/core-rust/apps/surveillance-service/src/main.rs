use adsb_ingest::AdsbListener;
use std::net::{Ipv4Addr, SocketAddr};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //initialize logging
    tracing_subscriber::fmt::init();

    info!("Surveillance servic started");
    // Default ADS-B port (SBS-1 compatible)
    let address = SocketAddr::from((Ipv4Addr::LOCALHOST, 30003));

    let listener = AdsbListener::bind(address).await?;

    info!("Listening On {}", listener.address());

    loop {
        let (frame, sender) = listener.receive().await?;

        info!(
            sender = %sender,
            bytes = frame.len(),
            "ADS-B packet received"
        );
    }

    // Ok(())
}
