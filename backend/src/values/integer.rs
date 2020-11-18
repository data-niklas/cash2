use crate::error::CashError;
use crate::value::{Value, ValueResult};
use crate::values::{BooleanValue, FloatValue};

#[derive(Debug, Clone)]
pub struct IntegerValue {
    pub value: i64,
}

impl IntegerValue {
    pub fn boxed(value: i64) -> ValueResult {
        Ok(Box::new(IntegerValue { value }))
    }
}

impl Value for IntegerValue {
    fn clone(&self) -> Box<dyn Value> {
        Box::new(std::clone::Clone::clone(self))
    }
    fn get_type_name(&self) -> &'static str {
        "integer"
    }
    fn uplus(self: Box<Self>) -> ValueResult {
        Ok(self)
    }
    fn uminus(mut self: Box<Self>) -> ValueResult {
        self.value = -self.value;
        Ok(self as Box<dyn Value>)
    }
    fn power(mut self: Box<Self>, value: &Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            if other.value < 0 {
                FloatValue::boxed((self.value as f64).powi(other.value as i32))
            } else {
                self.value = self.value.pow(other.value as u32);
                Ok(self)
            }
        } else if let Some(other) = value.downcast_ref::<FloatValue>() {
            FloatValue::boxed((self.value as f64).powf(other.value))
        } else {
            CashError::InvalidOperation("power".to_owned(), "integer ".to_owned() + typename)
                .boxed()
        }
    }
    fn multiply(mut self: Box<Self>, value: &Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            self.value *= other.value;
            Ok(self)
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
    fn division(self: Box<Self>, value: &Box<dyn Value>) -> ValueResult {
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
    fn modulo(mut self: Box<Self>, value: &Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            self.value %= other.value;
            Ok(self)
        } else if let Some(other) = value.downcast_ref::<FloatValue>() {
            FloatValue::boxed((self.value as f64) % other.value)
        } else {
            CashError::InvalidOperation("modulo".to_owned(), "integer ".to_owned() + typename)
                .boxed()
        }
    }
    fn add(mut self: Box<Self>, value: &Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            self.value += other.value;
            Ok(self)
        } else if let Some(other) = value.downcast_ref::<FloatValue>() {
            FloatValue::boxed((self.value as f64) + other.value)
        } else {
            CashError::InvalidOperation("add".to_owned(), "integer ".to_owned() + typename).boxed()
        }
    }
    fn subtract(mut self: Box<Self>, value: &Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            self.value -= other.value;
            Ok(self)
        } else if let Some(other) = value.downcast_ref::<FloatValue>() {
            FloatValue::boxed((self.value as f64) - other.value)
        } else {
            CashError::InvalidOperation("subtract".to_owned(), "integer ".to_owned() + typename)
                .boxed()
        }
    }
    fn bit_shift_l(mut self: Box<Self>, value: &Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            self.value <<= other.value;
            Ok(self)
        } else {
            CashError::InvalidOperation(
                "left bit shift".to_owned(),
                "integer ".to_owned() + typename,
            )
            .boxed()
        }
    }
    fn bit_shift_r(mut self: Box<Self>, value: &Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            self.value >>= other.value;
            Ok(self)
        } else {
            CashError::InvalidOperation(
                "right bit shift".to_owned(),
                "integer ".to_owned() + typename,
            )
            .boxed()
        }
    }
    fn lt(&self, value: &Box<dyn Value>) -> ValueResult {
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
    fn gt(&self, value: &Box<dyn Value>) -> ValueResult {
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
    fn lte(&self, value: &Box<dyn Value>) -> ValueResult {
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
    fn gte(&self, value: &Box<dyn Value>) -> ValueResult {
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
    fn eq(&self, value: &Box<dyn Value>) -> ValueResult {
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            BooleanValue::boxed(self.value == other.value)
        } else if let Some(other) = value.downcast_ref::<FloatValue>() {
            BooleanValue::boxed(((self.value as f64) - other.value).abs() < super::float::EPSILON)
        } else {
            BooleanValue::boxed(false)
        }
    }
    fn ne(&self, value: &Box<dyn Value>) -> ValueResult {
        self.eq(value)?.not()
    }
    fn and(mut self: Box<Self>, value: &Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            self.value &= other.value;
            Ok(self)
        } else {
            CashError::InvalidOperation("bitwise and".to_owned(), "integer ".to_owned() + typename)
                .boxed()
        }
    }
    fn xor(mut self: Box<Self>, value: &Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            self.value ^= other.value;
            Ok(self)
        } else {
            CashError::InvalidOperation("bitwise xor".to_owned(), "integer ".to_owned() + typename)
                .boxed()
        }
    }
    fn or(mut self: Box<Self>, value: &Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            self.value |= other.value;
            Ok(self)
        } else {
            CashError::InvalidOperation("bitwise or".to_owned(), "integer ".to_owned() + typename)
                .boxed()
        }
    }
    fn r#async(self: Box<Self>) -> ValueResult {
        unimplemented!();
    }
}

impl std::fmt::Display for IntegerValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
