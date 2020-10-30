use thiserror::Error;

#[derive(Debug, Error)]
pub enum CashError {
    #[error("operation '{0}' is not permitted for type '{1}'")]
    InvalidOperation(String, String),
    #[error("invalid length '{0}' for type '{1}'")]
    InvalidLength(i64, String),
    #[error("invalid type '{0}' for '{1}'")]
    InvalidType(String, String),
    #[error("index '{0}' is out of bounds for '{1}'")]
    IndexOutOfBounds(i64, String),
    #[error("invalid input '{0}'")]
    InvalidInput(String),
    #[error("invalid value '{0}' in '{1}'")]
    InvalidValue(String, String),
    #[error("{0}")]
    Bug(String),
}

impl CashError {
    pub fn boxed<T>(self) -> Result<T, Box<dyn std::error::Error>> {
        Err(Box::new(self))
    }
}
