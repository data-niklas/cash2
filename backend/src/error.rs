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
    #[error("key '{0}' not found for '{1}'")]
    KeyNotFound(String, String),
    #[error("variable '{0}' used before assignment")]
    VariableNotFound(String),
    #[error("invalid input '{0}'")]
    InvalidInput(String),
    #[error("invalid value '{0}' in '{1}'")]
    InvalidValue(String, String),
    #[error("invalid parameter count: found '{0}' parameters, but needs '{1}'")]
    InvalidParameterCount(usize, usize),
    #[error("invalid arguments: found '{0}', but needs '{1}'")]
    InvalidArguments(String, String),
    #[error("parse error: '{0}'")]
    ParseError(String),
    #[error("{0}")]
    Bug(String),
}

impl CashError {
    pub fn boxed<T>(self) -> Result<T, Box<dyn std::error::Error + Send + Sync>> {
        Err(Box::new(self))
    }
}
