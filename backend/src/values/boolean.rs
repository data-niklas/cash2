use crate::error::CashError;
use crate::value::{Value, ValueResult};

#[derive(Debug, Clone)]
pub struct BooleanValue {
    pub value: bool,
}

impl BooleanValue {
    pub fn boxed(value: bool) -> ValueResult {
        Ok(Box::new(BooleanValue { value }))
    }
}

impl Value for BooleanValue {
    fn clone(&self) -> Box<dyn Value> {
        Box::new(std::clone::Clone::clone(self))
    }
    fn get_type_name(&self) -> &'static str {
        "boolean"
    }
    fn not(mut self: Box<Self>) -> ValueResult {
        self.value = !self.value;
        Ok(self)
    }
    fn eq(&self, value: &Box<dyn Value>) -> ValueResult {
        if let Some(other) = value.downcast_ref::<BooleanValue>() {
            BooleanValue::boxed(self.value == other.value)
        } else {
            BooleanValue::boxed(false)
        }
    }
    fn ne(&self, value: &Box<dyn Value>) -> ValueResult {
        self.eq(value)?.not()
    }
    fn and(mut self: Box<Self>, value: &Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<BooleanValue>() {
            self.value = self.value && other.value;
            Ok(self)
        } else {
            CashError::InvalidOperation("and".to_owned(), "boolean ".to_owned() + typename).boxed()
        }
    }
    fn xor(mut self: Box<Self>, value: &Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<BooleanValue>() {
            self.value = (self.value || other.value) && (self.value != other.value);
            Ok(self)
        } else {
            CashError::InvalidOperation("and".to_owned(), "boolean ".to_owned() + typename).boxed()
        }
    }
    fn or(mut self: Box<Self>, value: &Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<BooleanValue>() {
            self.value = self.value || other.value;
            Ok(self)
        } else {
            CashError::InvalidOperation("and".to_owned(), "boolean ".to_owned() + typename).boxed()
        }
    }
    fn r#async(self: Box<Self>) -> ValueResult {
        unimplemented!();
    }
}

impl std::fmt::Display for BooleanValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
