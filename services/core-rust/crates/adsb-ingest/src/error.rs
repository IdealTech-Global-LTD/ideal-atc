use thiserror::Error;

#[derive(Debug, Error)]
pub enum IngestError {
    #[error("network error: {0}")]
    Network(#[from] std::io::Error),

    #[error("empty ADS-B packet received")]
    EmptyPacket,
}
