use crate::value::{ValueResult, Value};
use crate::error::CashError;

#[derive(Debug)]
pub struct BooleanValue {
    value: bool,
}

impl BooleanValue {
    pub fn boxed(value: bool) -> ValueResult {
        Ok(Box::new(BooleanValue { value }))
    }
}

impl Value for BooleanValue {
    fn get_type_name(&self) -> &'static str {
        "boolean"
    }
    fn not(&self) -> ValueResult {
        Self::boxed(!self.value)
    }
    fn eq(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Ok(other) = value.downcast::<BooleanValue>() {
            BooleanValue::boxed(self.value == other.value)
        } else {
            CashError::InvalidOperation("equality".to_owned(), "boolean ".to_owned() + typename)
                .boxed()
        }
    }
    fn ne(&self, value: Box<dyn Value>) -> ValueResult {
        self.eq(value)?.not()
    }
    fn and(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Ok(other) = value.downcast::<BooleanValue>() {
            BooleanValue::boxed(self.value && other.value)
        } else {
            CashError::InvalidOperation("and".to_owned(), "boolean ".to_owned() + typename).boxed()
        }
    }
    fn xor(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Ok(other) = value.downcast::<BooleanValue>() {
            BooleanValue::boxed((self.value || other.value) && (self.value != other.value))
        } else {
            CashError::InvalidOperation("and".to_owned(), "boolean ".to_owned() + typename).boxed()
        }
    }
    fn or(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Ok(other) = value.downcast::<BooleanValue>() {
            BooleanValue::boxed(self.value || other.value)
        } else {
            CashError::InvalidOperation("and".to_owned(), "boolean ".to_owned() + typename).boxed()
        }
    }
    fn r#async(&self) -> ValueResult {
        unimplemented!();
    }
}

impl std::fmt::Display for BooleanValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}