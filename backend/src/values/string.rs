use crate::error::CashError;
use crate::value::{Value, ValueResult};
use crate::values::{BooleanValue, FloatValue, IntegerValue};

#[derive(Debug)]
pub struct StringValue {
    pub value: String,
}

impl StringValue {
    pub fn boxed(value: String) -> ValueResult {
        Ok(Box::new(StringValue { value }))
    }
}

impl Value for StringValue {
    fn get_type_name(&self) -> &'static str {
        "string"
    }
    fn multiply(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            StringValue::boxed(self.value.repeat(other.value as usize))
        } else {
            CashError::InvalidOperation(
                "multiplication".to_owned(),
                "string ".to_owned() + typename,
            )
            .boxed()
        }
    }
    fn add(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            StringValue::boxed(self.value.to_owned() + &other.value.to_string())
        } else if let Some(other) = value.downcast_ref::<FloatValue>() {
            StringValue::boxed(self.value.to_owned() + &other.value.to_string())
        } else if let Some(other) = value.downcast_ref::<StringValue>() {
            StringValue::boxed(self.value.to_owned() + &other.value)
        } else {
            CashError::InvalidOperation("add".to_owned(), "string ".to_owned() + typename).boxed()
        }
    }
    fn subtract(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<StringValue>() {
            StringValue::boxed(self.value.replace(&other.value, ""))
        } else {
            CashError::InvalidOperation("subtract".to_owned(), "string ".to_owned() + typename)
                .boxed()
        }
    }
    fn lt(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<StringValue>() {
            BooleanValue::boxed(self.value < other.value)
        } else {
            CashError::InvalidOperation("less than".to_owned(), "string ".to_owned() + typename)
                .boxed()
        }
    }
    fn gt(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<StringValue>() {
            BooleanValue::boxed(self.value > other.value)
        } else {
            CashError::InvalidOperation("greater than".to_owned(), "string ".to_owned() + typename)
                .boxed()
        }
    }
    fn lte(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<StringValue>() {
            BooleanValue::boxed(self.value <= other.value)
        } else {
            CashError::InvalidOperation(
                "less / equal than".to_owned(),
                "string ".to_owned() + typename,
            )
            .boxed()
        }
    }
    fn gte(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<StringValue>() {
            BooleanValue::boxed(self.value >= other.value)
        } else {
            CashError::InvalidOperation(
                "greater / equal than".to_owned(),
                "string ".to_owned() + typename,
            )
            .boxed()
        }
    }
    fn eq(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<StringValue>() {
            BooleanValue::boxed(self.value == other.value)
        } else {
            CashError::InvalidOperation("equality".to_owned(), "string ".to_owned() + typename)
                .boxed()
        }
    }
    fn ne(&self, value: Box<dyn Value>) -> ValueResult {
        self.eq(value)?.not()
    }
    fn contains(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            BooleanValue::boxed(self.value.contains(&other.value.to_string()))
        } else if let Some(other) = value.downcast_ref::<FloatValue>() {
            BooleanValue::boxed(self.value.contains(&other.value.to_string()))
        } else if let Some(other) = value.downcast_ref::<StringValue>() {
            BooleanValue::boxed(self.value.contains(&other.value))
        } else {
            CashError::InvalidOperation("contains".to_owned(), "string ".to_owned() + typename)
                .boxed()
        }
    }
    fn r#async(&self) -> ValueResult {
        unimplemented!();
    }
}

impl std::fmt::Display for StringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.value)
    }
}
