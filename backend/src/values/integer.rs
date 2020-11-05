use crate::error::CashError;
use crate::value::{Value, ValueResult};
use crate::values::{BooleanValue, FloatValue};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct IntegerValue {
    pub value: i64,
}

impl IntegerValue {
    pub fn boxed(value: i64) -> ValueResult {
        Ok(Arc::new(IntegerValue { value }))
    }
}

impl Value for IntegerValue {
    fn clone(&self) -> Box<dyn Value> {
        Box::new(std::clone::Clone::clone(self))
    }
    fn get_type_name(&self) -> &'static str {
        "integer"
    }
    fn uplus(&self) -> ValueResult {
        IntegerValue::boxed(self.value)
    }
    fn uminus(&self) -> ValueResult {
        IntegerValue::boxed(-self.value)
    }
    fn power(&self, value: Arc<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            if other.value < 0 {
                FloatValue::boxed((self.value as f64).powi(other.value as i32))
            } else {
                IntegerValue::boxed(self.value.pow(other.value as u32))
            }
        } else if let Some(other) = value.downcast_ref::<FloatValue>() {
            FloatValue::boxed((self.value as f64).powf(other.value))
        } else {
            //TODO FLOATS; ...
            CashError::InvalidOperation("power".to_owned(), "integer ".to_owned() + typename)
                .boxed()
        }
    }
    fn multiply(&self, value: Arc<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            IntegerValue::boxed(self.value * other.value)
        } else if let Some(other) = value.downcast_ref::<FloatValue>() {
            FloatValue::boxed((self.value as f64) * other.value)
        } else {
            CashError::InvalidOperation(
                "multiplication".to_owned(),
                "integer ".to_owned() + typename,
            )
            .boxed()
        }
    }
    fn division(&self, value: Arc<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            FloatValue::boxed(self.value as f64 / other.value as f64)
        } else if let Some(other) = value.downcast_ref::<FloatValue>() {
            FloatValue::boxed((self.value as f64) / other.value)
        } else {
            CashError::InvalidOperation("division".to_owned(), "integer ".to_owned() + typename)
                .boxed()
        }
    }
    fn modulo(&self, value: Arc<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            IntegerValue::boxed(self.value % other.value)
        } else if let Some(other) = value.downcast_ref::<FloatValue>() {
            FloatValue::boxed((self.value as f64) % other.value)
        } else {
            CashError::InvalidOperation("modulo".to_owned(), "integer ".to_owned() + typename)
                .boxed()
        }
    }
    fn add(&self, value: Arc<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            IntegerValue::boxed(self.value + other.value)
        } else if let Some(other) = value.downcast_ref::<FloatValue>() {
            FloatValue::boxed((self.value as f64) + other.value)
        } else {
            CashError::InvalidOperation("add".to_owned(), "integer ".to_owned() + typename).boxed()
        }
    }
    fn subtract(&self, value: Arc<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            IntegerValue::boxed(self.value - other.value)
        } else if let Some(other) = value.downcast_ref::<FloatValue>() {
            FloatValue::boxed((self.value as f64) - other.value)
        } else {
            CashError::InvalidOperation("subtract".to_owned(), "integer ".to_owned() + typename)
                .boxed()
        }
    }
    fn bit_shift_l(&self, value: Arc<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            IntegerValue::boxed(self.value << other.value)
        } else {
            CashError::InvalidOperation(
                "left bit shift".to_owned(),
                "integer ".to_owned() + typename,
            )
            .boxed()
        }
    }
    fn bit_shift_r(&self, value: Arc<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            IntegerValue::boxed(self.value >> other.value)
        } else {
            CashError::InvalidOperation(
                "right bit shift".to_owned(),
                "integer ".to_owned() + typename,
            )
            .boxed()
        }
    }
    fn lt(&self, value: Arc<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            BooleanValue::boxed(self.value < other.value)
        } else if let Some(other) = value.downcast_ref::<FloatValue>() {
            BooleanValue::boxed((self.value as f64) < other.value)
        } else {
            CashError::InvalidOperation("less than".to_owned(), "integer ".to_owned() + typename)
                .boxed()
        }
    }
    fn gt(&self, value: Arc<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            BooleanValue::boxed(self.value > other.value)
        } else if let Some(other) = value.downcast_ref::<FloatValue>() {
            BooleanValue::boxed((self.value as f64) > other.value)
        } else {
            CashError::InvalidOperation("greater than".to_owned(), "integer ".to_owned() + typename)
                .boxed()
        }
    }
    fn lte(&self, value: Arc<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            BooleanValue::boxed(self.value <= other.value)
        } else if let Some(other) = value.downcast_ref::<FloatValue>() {
            BooleanValue::boxed((self.value as f64) <= other.value)
        } else {
            CashError::InvalidOperation(
                "less / equal than".to_owned(),
                "integer ".to_owned() + typename,
            )
            .boxed()
        }
    }
    fn gte(&self, value: Arc<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            BooleanValue::boxed(self.value >= other.value)
        } else if let Some(other) = value.downcast_ref::<FloatValue>() {
            BooleanValue::boxed((self.value as f64) >= other.value)
        } else {
            CashError::InvalidOperation(
                "greater / equal than".to_owned(),
                "integer ".to_owned() + typename,
            )
            .boxed()
        }
    }
    fn eq(&self, value: Arc<dyn Value>) -> ValueResult {
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            BooleanValue::boxed(self.value == other.value)
        } else if let Some(other) = value.downcast_ref::<FloatValue>() {
            BooleanValue::boxed(((self.value as f64) - other.value).abs() < super::float::EPSILON)
        } else {
            BooleanValue::boxed(false)
        }
    }
    fn ne(&self, value: Arc<dyn Value>) -> ValueResult {
        self.eq(value)?.not()
    }
    fn and(&self, value: Arc<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            IntegerValue::boxed(self.value & other.value)
        } else {
            CashError::InvalidOperation("bitwise and".to_owned(), "integer ".to_owned() + typename)
                .boxed()
        }
    }
    fn xor(&self, value: Arc<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            IntegerValue::boxed(self.value ^ other.value)
        } else {
            CashError::InvalidOperation("bitwise xor".to_owned(), "integer ".to_owned() + typename)
                .boxed()
        }
    }
    fn or(&self, value: Arc<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            IntegerValue::boxed(self.value | other.value)
        } else {
            CashError::InvalidOperation("bitwise or".to_owned(), "integer ".to_owned() + typename)
                .boxed()
        }
    }
    fn r#async(&self) -> ValueResult {
        unimplemented!();
    }
}

impl std::fmt::Display for IntegerValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
