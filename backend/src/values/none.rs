use crate::value::{Value, ValueResult};

#[derive(Default)]
pub struct NoneValue;

impl NoneValue {
    pub fn boxed() -> ValueResult {
        Ok(Box::new(Self::default()))
    }
}

impl Value for NoneValue {
    fn get_type_name(&self) -> &'static str {
        "none"
    }

    fn clone(&self) -> Box<dyn Value> {
        Box::new(Self::default())
    }
}

impl std::fmt::Debug for NoneValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "none")
    }
}

impl std::fmt::Display for NoneValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "none")
    }
}
