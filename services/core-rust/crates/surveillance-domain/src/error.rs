use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("invalid coordinate")]
    InvalidCoordinate,

    #[error("invalid altitude")]
    InvalidAltitude,

    #[error("invalid velocity")]
    InvalidVelocity,
}
