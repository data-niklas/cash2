use crate::error::CashError;
use crate::value::{Value, ValueResult};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct BooleanValue {
    pub value: bool,
}

impl BooleanValue {
    pub fn boxed(value: bool) -> ValueResult {
        Ok(Arc::new(BooleanValue { value }))
    }
}

impl Value for BooleanValue {
    fn clone(&self) -> Box<dyn Value> {
        Box::new(std::clone::Clone::clone(self))
    }
    fn get_type_name(&self) -> &'static str {
        "boolean"
    }
    fn not(&self) -> ValueResult {
        Self::boxed(!self.value)
    }
    fn eq(&self, value: Arc<dyn Value>) -> ValueResult {
        if let Some(other) = value.downcast_ref::<BooleanValue>() {
            BooleanValue::boxed(self.value == other.value)
        } else {
            BooleanValue::boxed(false)
        }
    }
    fn ne(&self, value: Arc<dyn Value>) -> ValueResult {
        self.eq(value)?.not()
    }
    fn and(&self, value: Arc<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<BooleanValue>() {
            BooleanValue::boxed(self.value && other.value)
        } else {
            CashError::InvalidOperation("and".to_owned(), "boolean ".to_owned() + typename).boxed()
        }
    }
    fn xor(&self, value: Arc<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<BooleanValue>() {
            BooleanValue::boxed((self.value || other.value) && (self.value != other.value))
        } else {
            CashError::InvalidOperation("and".to_owned(), "boolean ".to_owned() + typename).boxed()
        }
    }
    fn or(&self, value: Arc<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<BooleanValue>() {
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
