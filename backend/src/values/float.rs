use crate::error::CashError;
use crate::value::{Value, ValueResult};
use crate::values::{BooleanValue, IntegerValue};

pub static EPSILON: f64 = 1e-15f64;

#[derive(Debug, Clone)]
pub struct FloatValue {
    pub value: f64,
}

impl FloatValue {
    pub fn boxed(value: f64) -> ValueResult {
        Ok(Box::new(FloatValue { value }))
    }
}

impl Value for FloatValue {
    fn clone(&self) -> Box<dyn Value> {
        Box::new(std::clone::Clone::clone(self))
    }
    fn get_type_name(&self) -> &'static str {
        "float"
    }
    fn uplus(self: Box<Self>) -> ValueResult {
        Ok(self)
    }
    fn uminus(mut self: Box<Self>) -> ValueResult {
        self.value = -self.value;
        Ok(self)
    }
    fn power(mut self: Box<Self>, value: &Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<FloatValue>() {
            self.value = self.value.powf(other.value);
            Ok(self)
        } else if let Some(other) = value.downcast_ref::<IntegerValue>() {
            self.value = self.value.powi(other.value as i32);
            Ok(self)
        } else {
            CashError::InvalidOperation("power".to_owned(), "float ".to_owned() + typename).boxed()
        }
    }
    fn multiply(mut self: Box<Self>, value: &Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<FloatValue>() {
            self.value *= other.value;
            Ok(self)
        } else if let Some(other) = value.downcast_ref::<IntegerValue>() {
            self.value *= other.value as f64;
            Ok(self)
        } else {
            CashError::InvalidOperation("multiply".to_owned(), "float ".to_owned() + typename)
                .boxed()
        }
    }
    fn division(mut self: Box<Self>, value: &Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<FloatValue>() {
            self.value /= other.value;
            Ok(self)
        } else if let Some(other) = value.downcast_ref::<IntegerValue>() {
            self.value /= other.value as f64;
            Ok(self)
        } else {
            CashError::InvalidOperation("division".to_owned(), "float ".to_owned() + typename)
                .boxed()
        }
    }
    fn modulo(mut self: Box<Self>, value: &Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<FloatValue>() {
            self.value %= other.value;
            Ok(self)
        } else if let Some(other) = value.downcast_ref::<IntegerValue>() {
            self.value %= other.value as f64;
            Ok(self)
        } else {
            CashError::InvalidOperation("modulo".to_owned(), "float ".to_owned() + typename).boxed()
        }
    }
    fn add(mut self: Box<Self>, value: &Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<FloatValue>() {
            self.value += other.value;
            Ok(self)
        } else if let Some(other) = value.downcast_ref::<IntegerValue>() {
            self.value += other.value as f64;
            Ok(self)
        } else {
            CashError::InvalidOperation("add".to_owned(), "float ".to_owned() + typename).boxed()
        }
    }
    fn subtract(mut self: Box<Self>, value: &Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<FloatValue>() {
            self.value -= other.value;
            Ok(self)
        } else if let Some(other) = value.downcast_ref::<IntegerValue>() {
            self.value -= other.value as f64;
            Ok(self)
        } else {
            CashError::InvalidOperation("subtract".to_owned(), "float ".to_owned() + typename)
                .boxed()
        }
    }
    fn lt(&self, value: &Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<FloatValue>() {
            BooleanValue::boxed(self.value < other.value)
        } else if let Some(other) = value.downcast_ref::<IntegerValue>() {
            BooleanValue::boxed(self.value < other.value as f64)
        } else {
            CashError::InvalidOperation("less than".to_owned(), "float ".to_owned() + typename)
                .boxed()
        }
    }
    fn gt(&self, value: &Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<FloatValue>() {
            BooleanValue::boxed(self.value > other.value)
        } else if let Some(other) = value.downcast_ref::<IntegerValue>() {
            BooleanValue::boxed(self.value > other.value as f64)
        } else {
            CashError::InvalidOperation("greater than".to_owned(), "float ".to_owned() + typename)
                .boxed()
        }
    }
    fn lte(&self, value: &Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<FloatValue>() {
            BooleanValue::boxed(self.value <= other.value)
        } else if let Some(other) = value.downcast_ref::<IntegerValue>() {
            BooleanValue::boxed(self.value <= other.value as f64)
        } else {
            CashError::InvalidOperation(
                "less than equal".to_owned(),
                "float ".to_owned() + typename,
            )
            .boxed()
        }
    }
    fn gte(&self, value: &Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<FloatValue>() {
            BooleanValue::boxed(self.value < other.value)
        } else if let Some(other) = value.downcast_ref::<IntegerValue>() {
            BooleanValue::boxed(self.value < other.value as f64)
        } else {
            CashError::InvalidOperation(
                "greater than equal".to_owned(),
                "float ".to_owned() + typename,
            )
            .boxed()
        }
    }
    fn eq(&self, value: &Box<dyn Value>) -> ValueResult {
        if let Some(other) = value.downcast_ref::<FloatValue>() {
            BooleanValue::boxed((self.value - other.value).abs() < EPSILON)
        } else if let Some(other) = value.downcast_ref::<IntegerValue>() {
            BooleanValue::boxed((self.value - other.value as f64).abs() < EPSILON)
        } else {
            BooleanValue::boxed(false)
        }
    }
    fn ne(&self, value: &Box<dyn Value>) -> ValueResult {
        self.eq(value)?.not()
    }
}

impl std::fmt::Display for FloatValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
