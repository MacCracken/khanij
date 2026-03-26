use thiserror::Error;

/// # Examples
///
/// ```
/// # use khanij::*;
/// let err = KhanijError::InvalidMineral("unobtanium".into());
/// assert!(err.to_string().contains("unobtanium"));
///
/// let err = KhanijError::InvalidComposition("negative weight%".into());
/// assert!(err.to_string().contains("negative weight%"));
/// ```
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
