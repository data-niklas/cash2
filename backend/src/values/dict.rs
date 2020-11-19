use crate::error::CashError;
use crate::value::{Value, ValueResult};
use crate::values::{BooleanValue, StringValue};
use std::collections::HashMap;

#[derive(Debug)]
pub struct DictValue {
    pub values: HashMap<String, Box<dyn Value>>,
}

impl DictValue {
    pub fn boxed(values: HashMap<String, Box<dyn Value>>) -> ValueResult {
        Ok(Box::new(DictValue { values }))
    }

    pub fn insert(
        mut self: Box<Self>,
        index: &Box<dyn Value>,
        value: &Box<dyn Value>,
    ) -> ValueResult {
        if let Some(key) = index.downcast_ref::<StringValue>() {
            self.values.insert(key.value.to_owned(), (*value).clone());
            Ok(self)
        } else {
            CashError::InvalidOperation(
                "insert".to_owned(),
                "dict ".to_owned() + index.get_type_name(),
            )
            .boxed()
        }
    }

    pub fn remove(mut self: Box<Self>, index: &Box<dyn Value>) -> ValueResult {
        if let Some(key) = index.downcast_ref::<StringValue>() {
            self.values.remove(&key.value);
            Ok(self)
        } else {
            CashError::InvalidOperation(
                "remove".to_owned(),
                "dict ".to_owned() + index.get_type_name(),
            )
            .boxed()
        }
    }
}

impl Value for DictValue {
    fn get_type_name(&self) -> &'static str {
        "dict"
    }

    fn indexed_set(
        &mut self,
        value: Box<dyn Value>,
        indexes: &[Box<dyn Value>],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        assert!(!indexes.is_empty());
        let index = indexes[0].to_string();
        if indexes.len() == 1 {
            self.values.insert(index, value);
            Ok(())
        } else if self.values.contains_key(&index) {
            let inner = self
                .values
                .get_mut(&index)
                .expect("cannot happen, was checked");
            inner.indexed_set(value, &indexes[1..])
        } else {
            CashError::KeyNotFound(index, self.get_type_name().to_owned()).boxed()
        }
    }

    fn index(&self, index: &Box<dyn Value>) -> ValueResult {
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
    fn contains(&self, value: &Box<dyn Value>) -> ValueResult {
        let index = value.to_string();
        BooleanValue::boxed(self.values.contains_key(&index))
    }
    fn eq(&self, value: &Box<dyn Value>) -> ValueResult {
        if let Some(other) = value.downcast_ref::<DictValue>() {
            if self.values.len() != other.values.len() {
                return BooleanValue::boxed(false);
            }
            for (key, value) in self.values.iter() {
                if !other.values.contains_key(key) {
                    return BooleanValue::boxed(false);
                }
                let value2 = other.values.get(key).expect("Value should exist");
                if let Some(b) = (*value).ne(value2)?.downcast_ref::<BooleanValue>() {
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
    fn ne(&self, value: &Box<dyn Value>) -> ValueResult {
        self.eq(value)?.not()
    }
    fn clone(&self) -> Box<dyn Value> {
        let mut v: HashMap<String, Box<dyn Value>> = HashMap::with_capacity(self.values.len());
        for (key, value) in &self.values {
            v.insert(key.to_owned(), (*value).clone());
        }
        Box::new(Self { values: v })
    }

    fn vec(
        self: Box<Self>,
    ) -> Result<Vec<Box<dyn Value>>, Box<dyn std::error::Error + Sync + Send>> {
        let mut vec: Vec<Box<dyn Value>> = Vec::new();
        for key in self.values.keys() {
            vec.push(Box::new(StringValue {
                value: key.to_owned(),
            }));
        }
        Ok(vec)
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
