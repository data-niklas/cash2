use crate::error::CashError;
use crate::value::{Value, ValueResult};
use crate::values::BooleanValue;

#[derive(Debug)]
pub struct IntegerValue {
    value: i64,
}

impl IntegerValue {
    pub fn boxed(value: i64) -> ValueResult {
        Ok(Box::new(IntegerValue { value }))
    }
}

impl Value for IntegerValue {
    fn get_type_name(&self) -> &'static str {
        "integer"
    }
    fn uplus(&self) -> ValueResult {
        IntegerValue::boxed(self.value)
    }
    fn uminus(&self) -> ValueResult {
        IntegerValue::boxed(-self.value)
    }
    fn power(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Ok(other) = value.downcast::<IntegerValue>() {
            if other.value < 0 {
                unimplemented!();
            } else {
                IntegerValue::boxed(self.value.pow(other.value as u32))
            }
        } else {
            //TODO FLOATS; ...
            CashError::InvalidOperation("power".to_owned(), "integer ".to_owned() + typename)
                .boxed()
        }
    }
    fn multiply(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Ok(other) = value.downcast::<IntegerValue>() {
            IntegerValue::boxed(self.value * other.value)
        } else {
            //TODO FLOATS; ...
            CashError::InvalidOperation(
                "multiplication".to_owned(),
                "integer ".to_owned() + typename,
            )
            .boxed()
        }
    }
    fn division(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Ok(other) = value.downcast::<IntegerValue>() {
            unimplemented!();
        //FloatValue::boxed(self.value / other.value)
        } else {
            //TODO FLOATS; ...
            CashError::InvalidOperation("division".to_owned(), "integer ".to_owned() + typename)
                .boxed()
        }
    }
    fn modulo(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Ok(other) = value.downcast::<IntegerValue>() {
            IntegerValue::boxed(self.value % other.value)
        } else {
            CashError::InvalidOperation("modulo".to_owned(), "integer ".to_owned() + typename)
                .boxed()
        }
    }
    fn add(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Ok(other) = value.downcast::<IntegerValue>() {
            IntegerValue::boxed(self.value + other.value)
        } else {
            //TODO FLOATS, ...
            CashError::InvalidOperation("add".to_owned(), "integer ".to_owned() + typename).boxed()
        }
    }
    fn subtract(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Ok(other) = value.downcast::<IntegerValue>() {
            IntegerValue::boxed(self.value - other.value)
        } else {
            //TODO FLOATS, ...
            CashError::InvalidOperation("subtract".to_owned(), "integer ".to_owned() + typename)
                .boxed()
        }
    }
    fn bit_shift_l(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Ok(other) = value.downcast::<IntegerValue>() {
            IntegerValue::boxed(self.value << other.value)
        } else {
            CashError::InvalidOperation(
                "left bit shift".to_owned(),
                "integer ".to_owned() + typename,
            )
            .boxed()
        }
    }
    fn bit_shift_r(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Ok(other) = value.downcast::<IntegerValue>() {
            IntegerValue::boxed(self.value >> other.value)
        } else {
            CashError::InvalidOperation(
                "right bit shift".to_owned(),
                "integer ".to_owned() + typename,
            )
            .boxed()
        }
    }
    fn lt(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Ok(other) = value.downcast::<IntegerValue>() {
            BooleanValue::boxed(self.value < other.value)
        } else {
            //TODO floats
            CashError::InvalidOperation("less than".to_owned(), "integer ".to_owned() + typename)
                .boxed()
        }
    }
    fn gt(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Ok(other) = value.downcast::<IntegerValue>() {
            BooleanValue::boxed(self.value > other.value)
        } else {
            //TODO floats
            CashError::InvalidOperation("greater than".to_owned(), "integer ".to_owned() + typename)
                .boxed()
        }
    }
    fn lte(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Ok(other) = value.downcast::<IntegerValue>() {
            BooleanValue::boxed(self.value <= other.value)
        } else {
            //TODO floats
            CashError::InvalidOperation(
                "less / equal than".to_owned(),
                "integer ".to_owned() + typename,
            )
            .boxed()
        }
    }
    fn gte(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Ok(other) = value.downcast::<IntegerValue>() {
            BooleanValue::boxed(self.value >= other.value)
        } else {
            //TODO floats
            CashError::InvalidOperation(
                "greater / equal than".to_owned(),
                "integer ".to_owned() + typename,
            )
            .boxed()
        }
    }
    fn eq(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Ok(other) = value.downcast::<IntegerValue>() {
            BooleanValue::boxed(self.value == other.value)
        } else {
            //TODO floats
            CashError::InvalidOperation("equality".to_owned(), "integer ".to_owned() + typename)
                .boxed()
        }
    }
    fn ne(&self, value: Box<dyn Value>) -> ValueResult {
        self.eq(value)?.not()
    }
    fn and(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Ok(other) = value.downcast::<IntegerValue>() {
            IntegerValue::boxed(self.value & other.value)
        } else {
            CashError::InvalidOperation("bitwise and".to_owned(), "integer ".to_owned() + typename)
                .boxed()
        }
    }
    fn xor(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Ok(other) = value.downcast::<IntegerValue>() {
            IntegerValue::boxed(self.value ^ other.value)
        } else {
            CashError::InvalidOperation("bitwise xor".to_owned(), "integer ".to_owned() + typename)
                .boxed()
        }
    }
    fn or(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Ok(other) = value.downcast::<IntegerValue>() {
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
