#[derive(Debug, thiserror::Error)]
pub enum AsterixError {
    #[error("packet too short")]
    PacketTooShort,

    #[error("invalid packet length")]
    InvalidLength,

    #[error("unsupported category")]
    UnsupportedCategory,
}
