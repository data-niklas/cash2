use thiserror::Error;

#[derive(Debug, Error)]
pub enum CashError {
    #[error("operation '{0}' is not permitted for type '{1}'")]
    InvalidOperation(String, String),
    #[error("invalid length '{0}' for type '{1}'")]
    InvalidLength(i64, String),
    #[error("invalid type '{0}' for '{1}'")]
    InvalidType(String, String),
}

impl CashError {
    pub fn boxed<T>(self) -> Result<T, Box<dyn std::error::Error>> {
        Err(Box::new(self))
    }
}
