use crate::error::CashError;
use crate::value::{Value, ValueResult};
use crate::values::{BooleanValue, IntegerValue, RangeValue};

#[derive(Debug)]
pub struct ListValue {
    pub values: Vec<Box<dyn Value>>,
}

impl ListValue {
    pub fn boxed(values: Vec<Box<dyn Value>>) -> ValueResult {
        Ok(Box::new(ListValue { values }))
    }

    pub fn exists(&self, index: &Box<dyn Value>) -> ValueResult {
        let typename = index.get_type_name();
        if let Some(other) = index.downcast_ref::<IntegerValue>() {
            let index: usize;
            if other.value < 0 {
                index = (self.values.len() as i64 + other.value) as usize;
            } else {
                index = other.value as usize;
            }
            if index >= self.values.len() || -other.value > self.values.len() as i64 {
                BooleanValue::boxed(false)
            } else {
                BooleanValue::boxed(true)
            }
        } else if let Some(other) = index.downcast_ref::<RangeValue>() {
            if other.lower < 0 {
                BooleanValue::boxed(false)
            } else if other.upper > self.values.len() as i64 {
                BooleanValue::boxed(false)
            } else {
                let mut v = Vec::with_capacity((other.upper - other.lower) as usize);
                let _ = self
                    .values
                    .iter()
                    .skip(other.lower as usize)
                    .take((other.upper - other.lower) as usize)
                    .map(|val| v.push((*val).clone()))
                    .collect::<Vec<_>>();
                BooleanValue::boxed(true)
            }
        } else {
            CashError::InvalidOperation("index".to_owned(), "string ".to_owned() + typename).boxed()
        }
    }
}

impl Value for ListValue {
    fn get_type_name(&self) -> &'static str {
        "list"
    }

    fn indexed_set(
        &mut self,
        value: Box<dyn Value>,
        indexes: &[Box<dyn Value>],
    ) -> Result<(), Box<dyn std::error::Error>> {
        assert!(indexes.len() > 0);
        if let Some(other) = indexes[0].downcast_ref::<IntegerValue>() {
            let index: usize;
            if other.value < 0 {
                index = (self.values.len() as i64 + other.value) as usize;
            } else {
                index = other.value as usize;
            }
            if index >= self.values.len() || -other.value > self.values.len() as i64 {
                CashError::IndexOutOfBounds(other.value, self.get_type_name().to_owned()).boxed()
            } else {
                if indexes.len() == 1 {
                    self.values[index] = value;
                    Ok(())
                } else {
                    self.values[index].indexed_set(value, &indexes[1..])
                }
            }
        } else if let Some(other) = indexes[0].downcast_ref::<RangeValue>() {
            if other.lower < 0 {
                CashError::IndexOutOfBounds(other.lower, self.get_type_name().to_owned()).boxed()
            } else if other.upper > self.values.len() as i64 {
                CashError::IndexOutOfBounds(other.upper, self.get_type_name().to_owned()).boxed()
            } else {
                for i in other.lower..other.upper {
                    if indexes.len() == 1 {
                        self.values[i as usize] = (*value).clone();
                    } else {
                        self.values[i as usize].indexed_set((*value).clone(), &indexes[1..])?;
                    }
                }
                Ok(())
            }
        } else {
            CashError::InvalidOperation("indexing ".to_owned(), "list ".to_owned() + "todo").boxed()
        }
    }

    fn index(&self, index: &Box<dyn Value>) -> ValueResult {
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
    fn multiply(self: Box<Self>, value: &Box<dyn Value>) -> ValueResult {
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
    fn add(mut self: Box<Self>, value: &Box<dyn Value>) -> ValueResult {
        self.values.push((*value).clone());
        Ok(self)
    }
    fn contains(&self, value: &Box<dyn Value>) -> ValueResult {
        for val in self.values.iter() {
            if let Ok(boxed) = (*val).eq(value) {
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
    fn eq(&self, value: &Box<dyn Value>) -> ValueResult {
        if let Some(other) = value.downcast_ref::<ListValue>() {
            if self.values.len() == other.values.len() {
                for (x, y) in self.values.iter().zip(other.values.iter()) {
                    if let Some(b) = (*x).ne(y)?.downcast_ref::<BooleanValue>() {
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
    fn ne(&self, value: &Box<dyn Value>) -> ValueResult {
        self.eq(value)?.not()
    }
    fn r#async(self: Box<Self>) -> ValueResult {
        unimplemented!()
    }
    fn clone(&self) -> Box<dyn Value> {
        let mut v: Vec<Box<dyn Value>> = Vec::with_capacity(self.values.len());
        for value in &self.values {
            v.push((*value).clone());
        }
        Box::new(Self { values: v })
    }

    fn vec(self: Box<Self>) -> Result<Vec<Box<dyn Value>>, Box<dyn std::error::Error>> {
        Ok(self.values)
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
