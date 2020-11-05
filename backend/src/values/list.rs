use crate::error::CashError;
use crate::value::{Value, ValueResult};
use crate::values::{BooleanValue, IntegerValue, RangeValue};
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct ListValue {
    pub values: Vec<Arc<dyn Value>>,
}

impl ListValue {
    pub fn boxed(values: Vec<Arc<dyn Value>>) -> ValueResult {
        Ok(Arc::new(ListValue { values }))
    }
}

impl Value for ListValue {
    fn get_type_name(&self) -> &'static str {
        "list"
    }
    fn index(&self, index: Arc<dyn Value>) -> ValueResult {
        let typename = index.get_type_name();
        if let Some(other) = index.downcast_ref::<IntegerValue>() {
            let index: usize;
            if other.value < 0 {
                index = (self.values.len() as i64 + other.value) as usize;
            } else {
                index = other.value as usize;
            }
            if index >= self.values.len() || -other.value > self.values.len() as i64 {
                CashError::IndexOutOfBounds(other.value, self.get_type_name().to_owned()).boxed()
            } else {
                Ok(self.values[index].clone())
            }
        } else if let Some(other) = index.downcast_ref::<RangeValue>() {
            if other.lower < 0 {
                CashError::IndexOutOfBounds(other.lower, self.get_type_name().to_owned()).boxed()
            } else if other.upper > self.values.len() as i64 {
                CashError::IndexOutOfBounds(other.upper, self.get_type_name().to_owned()).boxed()
            } else {
                let mut v = Vec::with_capacity((other.upper - other.lower) as usize);
                let _ = self
                    .values
                    .iter()
                    .skip(other.lower as usize)
                    .take((other.upper - other.lower) as usize)
                    .map(|val| v.push((*val).clone()))
                    .collect::<Vec<_>>();
                ListValue::boxed(v)
            }
        } else {
            CashError::InvalidOperation("index".to_owned(), "string ".to_owned() + typename).boxed()
        }
    }
    fn multiply(&self, value: Arc<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Some(other) = value.downcast_ref::<IntegerValue>() {
            if other.value < 0 {
                CashError::InvalidValue(format!("{}", other.value), self.get_type_name().to_owned())
                    .boxed()
            } else {
                let mut v = Vec::with_capacity(other.value as usize * self.values.len());
                for _i in 0..(other.value as usize) {
                    let _ = self
                        .values
                        .iter()
                        .map(|x| v.push((*x).clone()))
                        .collect::<Vec<_>>();
                }
                ListValue::boxed(v)
            }
        } else {
            CashError::InvalidOperation("multiplication".to_owned(), "list ".to_owned() + typename)
                .boxed()
        }
    }
    fn add(&self, value: Arc<dyn Value>) -> ValueResult {
        let mut clone = self
            .clone()
            .downcast::<ListValue>()
            .expect("Did not get a Listvalue from a Listvalue?");
        clone.values.push(value);
        Ok(Arc::new(*clone))
    }
    fn contains(&self, value: Arc<dyn Value>) -> ValueResult {
        for val in self.values.iter() {
            if let Ok(boxed) = (*val).eq(value.clone()) {
                if let Some(b) = boxed.downcast_ref::<BooleanValue>() {
                    if b.value {
                        return BooleanValue::boxed(true);
                    }
                } else {
                    return CashError::Bug(
                        "eq did not return a boolean value, what happened here?".to_owned(),
                    )
                    .boxed();
                }
            }
        }
        BooleanValue::boxed(false)
    }
    fn eq(&self, value: Arc<dyn Value>) -> ValueResult {
        if let Some(other) = value.downcast_ref::<ListValue>() {
            if self.values.len() == other.values.len() {
                for (x, y) in self.values.iter().zip(other.values.iter()) {
                    if let Some(b) = (*x).ne((*y).clone())?.downcast_ref::<BooleanValue>() {
                        if b.value {
                            return BooleanValue::boxed(false);
                        }
                    } else {
                        return CashError::Bug(
                            "NE did not return a boolean value, what happened here?".to_owned(),
                        )
                        .boxed();
                    }
                }
                return BooleanValue::boxed(true);
            }
            BooleanValue::boxed(false)
        } else {
            BooleanValue::boxed(false)
        }
    }
    fn ne(&self, value: Arc<dyn Value>) -> ValueResult {
        self.eq(value)?.not()
    }
    fn r#async(&self) -> ValueResult {
        unimplemented!()
    }
    fn clone(&self) -> Box<dyn Value> {
        let mut v: Vec<Arc<dyn Value>> = Vec::with_capacity(self.values.len());
        for value in &self.values {
            v.push((*value).clone());
        }
        Box::new(Self { values: v })
    }
}

impl std::fmt::Display for ListValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("[");
        for (idx, value) in self.values.iter().enumerate() {
            if idx == 0 {
                s.push_str(&format!("{}", value));
            } else {
                s.push_str(&format!(", {}", value));
            }
        }
        s.push_str("]");
        write!(f, "{}", s)
    }
}
