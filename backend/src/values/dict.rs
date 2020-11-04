use crate::error::CashError;
use crate::value::{Value, ValueResult};
use crate::values::BooleanValue;
use std::collections::HashMap;

#[derive(Debug)]
pub struct DictValue {
    pub values: HashMap<String, Box<dyn Value>>,
}

impl DictValue {
    pub fn boxed(values: HashMap<String, Box<dyn Value>>) -> ValueResult {
        Ok(Box::new(DictValue { values }))
    }
}

impl Value for DictValue {
    fn get_type_name(&self) -> &'static str {
        "dict"
    }
    fn index(&self, index: Box<dyn Value>) -> ValueResult {
        let index = index.to_string();
        if self.values.contains_key(&index) {
            let res = self
                .values
                .get(&index)
                .expect("Should not happen due to if");
            Ok((*res).clone())
        } else {
            CashError::KeyNotFound(index, self.get_type_name().to_owned()).boxed()
        }
    }
    fn contains(&self, value: Box<dyn Value>) -> ValueResult {
        let index = value.to_string();
        BooleanValue::boxed(self.values.contains_key(&index))
    }
    fn eq(&self, value: Box<dyn Value>) -> ValueResult {
        if let Some(other) = value.downcast_ref::<DictValue>() {
            if self.values.len() != other.values.len() {
                return BooleanValue::boxed(false);
            }
            for (key, value) in self.values.iter() {
                if !other.values.contains_key(key) {
                    return BooleanValue::boxed(false);
                }
                let value2 = other.values.get(key).expect("Value should exist");
                if let Ok(b) = (*value).ne((*value2).clone())?.downcast::<BooleanValue>() {
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
            BooleanValue::boxed(true)
        } else {
            BooleanValue::boxed(false)
        }
    }
    fn ne(&self, value: Box<dyn Value>) -> ValueResult {
        self.eq(value)?.not()
    }
    fn r#async(&self) -> ValueResult {
        unimplemented!()
    }
    fn clone(&self) -> Box<dyn Value> {
        let mut v: HashMap<String, Box<dyn Value>> = HashMap::with_capacity(self.values.len());
        for (key, value) in &self.values {
            v.insert(key.to_owned(), (*value).clone());
        }
        Box::new(Self { values: v })
    }
}

impl std::fmt::Display for DictValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("{");
        for (idx, (key, value)) in self.values.iter().enumerate() {
            if idx == 0 {
                s.push_str(&format!("{}: {}", key, value));
            } else {
                s.push_str(&format!(", {}: {}", key, value));
            }
        }
        s.push_str("}");
        write!(f, "{}", s)
    }
}
