use crate::error::CashError;
use crate::value::{Value, ValueResult};
use crate::values::{BooleanValue, IntegerValue, RangeValue};

#[derive(Debug, Clone)]
pub struct StringValue {
    pub value: String,
}

impl StringValue {
    pub fn boxed(value: String) -> ValueResult {
        Ok(Box::new(StringValue { value }))
    }
}

impl Value for StringValue {
    fn clone(&self) -> Box<dyn Value> {
        Box::new(std::clone::Clone::clone(self))
    }
    fn get_type_name(&self) -> &'static str {
        "string"
    }
    fn index(&self, index: &Box<dyn Value>) -> ValueResult {
        let typename = index.get_type_name();
        if let Some(other) = index.downcast_ref::<IntegerValue>() {
            let index: usize;
            if other.value < 0 {
                index = (self.value.len() as i64 + other.value) as usize;
            } else {
                index = other.value as usize;
            }
            if index >= self.value.len() || -other.value > self.value.len() as i64 {
                CashError::IndexOutOfBounds(other.value, self.get_type_name().to_owned()).boxed()
            } else {
                StringValue::boxed(self.value.chars().nth(index).unwrap().to_string())
            }
        } else if let Some(other) = index.downcast_ref::<RangeValue>() {
            if other.lower < 0 {
                CashError::IndexOutOfBounds(other.lower, self.get_type_name().to_owned()).boxed()
            } else if other.upper > self.value.len() as i64 {
                CashError::IndexOutOfBounds(other.upper, self.get_type_name().to_owned()).boxed()
            } else {
                StringValue::boxed(
                    self.value
                        .chars()
                        .skip(other.lower as usize)
                        .take((other.upper - other.lower) as usize)
                        .collect(),
                )
            }
        } else {
            CashError::InvalidOperation("index".to_owned(), "string ".to_owned() + typename).boxed()
        }
    }
    fn multiply(mut self: Box<Self>, value: &Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            if other.value < 0 {
                CashError::InvalidValue(format!("{}", other.value), self.get_type_name().to_owned())
                    .boxed()
            } else {
                self.value = self.value.repeat(other.value as usize);
                Ok(self)
            }
        } else {
            CashError::InvalidOperation(
                "multiplication".to_owned(),
                "string ".to_owned() + typename,
            )
            .boxed()
        }
    }
    fn add(mut self: Box<Self>, value: &Box<dyn Value>) -> ValueResult {
        self.value.push_str(&value.to_string());
        Ok(self)
    }
    fn subtract(mut self: Box<Self>, value: &Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<StringValue>() {
            self.value = self.value.replace(&other.value, "");
            Ok(self)
        } else {
            CashError::InvalidOperation("subtract".to_owned(), "string ".to_owned() + typename)
                .boxed()
        }
    }
    fn lt(&self, value: &Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<StringValue>() {
            BooleanValue::boxed(self.value < other.value)
        } else {
            CashError::InvalidOperation("less than".to_owned(), "string ".to_owned() + typename)
                .boxed()
        }
    }
    fn gt(&self, value: &Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<StringValue>() {
            BooleanValue::boxed(self.value > other.value)
        } else {
            CashError::InvalidOperation("greater than".to_owned(), "string ".to_owned() + typename)
                .boxed()
        }
    }
    fn lte(&self, value: &Box<dyn Value>) -> ValueResult {
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
    fn gte(&self, value: &Box<dyn Value>) -> ValueResult {
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
    fn eq(&self, value: &Box<dyn Value>) -> ValueResult {
        if let Some(other) = value.downcast_ref::<StringValue>() {
            BooleanValue::boxed(self.value == other.value)
        } else {
            BooleanValue::boxed(false)
        }
    }
    fn ne(&self, value: &Box<dyn Value>) -> ValueResult {
        self.eq(value)?.not()
    }
    fn contains(&self, value: &Box<dyn Value>) -> ValueResult {
        BooleanValue::boxed(self.value.contains(&value.to_string()))
    }
    fn r#async(self: Box<Self>) -> ValueResult {
        unimplemented!();
    }

    fn vec(self: Box<Self>) -> Result<Vec<Box<dyn Value>>, Box<dyn std::error::Error>> {
        let mut vec: Vec<Box<dyn Value>> = Vec::new();
        for c in self.value.chars() {
            vec.push(Box::new(StringValue {
                value: String::from(c),
            }));
        }
        Ok(vec)
    }
}

impl std::fmt::Display for StringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
