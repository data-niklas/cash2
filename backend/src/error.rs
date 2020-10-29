use thiserror::Error;

#[derive(Debug, Error)]
pub enum CashError {
    #[error("operation '{0}' is not permitted for type '{1}'")]
    InvalidOperation(String, String),
}

impl CashError {
    pub fn boxed<T>(self) -> Result<T, Box<dyn std::error::Error>> {
        Err(Box::new(self))
    }
}
