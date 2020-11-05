use crate::error::CashError;
use crate::value::{Value, ValueResult};
use crate::values::{BooleanValue, FloatValue, IntegerValue};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct RangeValue {
    pub lower: i64,
    pub upper: i64,
}

impl RangeValue {
    pub fn boxed(lower: i64, upper: i64) -> ValueResult {
        let length = upper - lower;
        if length < 1 {
            CashError::InvalidLength(length, "range".to_owned()).boxed()
        } else {
            Ok(Arc::new(RangeValue { lower, upper }))
        }
    }
}

impl Value for RangeValue {
    fn clone(&self) -> Box<dyn Value> {
        Box::new(std::clone::Clone::clone(self))
    }
    fn get_type_name(&self) -> &'static str {
        "range"
    }
    fn uplus(&self) -> ValueResult {
        RangeValue::boxed(self.lower, self.upper)
    }
    fn uminus(&self) -> ValueResult {
        RangeValue::boxed(-self.upper, -self.lower)
    }
    fn multiply(&self, value: Arc<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        let length = self.upper - self.lower;
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            let newlength = length * other.value;
            RangeValue::boxed(self.lower, self.lower + newlength)
        } else if let Some(other) = value.downcast_ref::<FloatValue>() {
            let newlength = (length as f64 * other.value) as i64;
            RangeValue::boxed(self.lower, self.lower + newlength)
        } else {
            CashError::InvalidOperation("multiply".to_owned(), "range ".to_owned() + typename)
                .boxed()
        }
    }
    fn division(&self, value: Arc<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        let length = self.upper - self.lower;
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            let newlength = length / other.value;
            RangeValue::boxed(self.lower, self.lower + newlength)
        } else if let Some(other) = value.downcast_ref::<FloatValue>() {
            let newlength = (length as f64 * other.value) as i64;
            RangeValue::boxed(self.lower, self.lower + newlength)
        } else {
            CashError::InvalidOperation("division".to_owned(), "range ".to_owned() + typename)
                .boxed()
        }
    }
    fn add(&self, value: Arc<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            RangeValue::boxed(self.lower + other.value, self.upper + other.value)
        } else {
            CashError::InvalidOperation("add".to_owned(), "range ".to_owned() + typename).boxed()
        }
    }
    fn subtract(&self, value: Arc<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            RangeValue::boxed(self.lower - other.value, self.upper - other.value)
        } else {
            CashError::InvalidOperation("subtract".to_owned(), "range ".to_owned() + typename)
                .boxed()
        }
    }
    fn contains(&self, value: Arc<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            BooleanValue::boxed(self.lower <= other.value && self.upper > other.value)
        } else if let Some(other) = value.downcast_ref::<FloatValue>() {
            BooleanValue::boxed(self.lower as f64 <= other.value && self.upper as f64 > other.value)
        } else {
            CashError::InvalidOperation("contains".to_owned(), "range ".to_owned() + typename)
                .boxed()
        }
    }
    fn eq(&self, value: Arc<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<RangeValue>() {
            BooleanValue::boxed(self.lower == other.lower && self.upper == other.upper)
        } else {
            CashError::InvalidOperation("equality".to_owned(), "range ".to_owned() + typename)
                .boxed()
        }
    }
    fn ne(&self, value: Arc<dyn Value>) -> ValueResult {
        self.eq(value)?.not()
    }
    fn r#async(&self) -> ValueResult {
        unimplemented!();
    }
}

impl std::fmt::Display for RangeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.lower, self.upper)
    }
}
