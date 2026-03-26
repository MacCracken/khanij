use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum KhanijError {
    #[error("invalid mineral: {0}")]
    InvalidMineral(String),
    #[error("invalid composition: {0}")]
    InvalidComposition(String),
    #[error("invalid hardness: {0}")]
    InvalidHardness(String),
    #[error("computation error: {0}")]
    ComputationError(String),
}

pub type Result<T> = std::result::Result<T, KhanijError>;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn error_display() {
        let e = KhanijError::InvalidMineral("unobtanium".into());
        assert!(e.to_string().contains("unobtanium"));
    }
}
