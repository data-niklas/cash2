use crate::error::CashError;
use crate::value::{Value, ValueResult};
use crate::values::{BooleanValue, DictValue, IntegerValue, ListValue, NoneValue, StringValue};
use std::collections::HashMap;

/// Takes exactly 2 params
/// first: a list/dict to be mapped
/// second: the function, which will receive either the value and index of the list item, or the key and value of the dict item
pub fn map_closure(mut params: Vec<Box<dyn Value>>) -> ValueResult {
    if params.len() == 2 {
        let first = params.remove(0);
        let second = params.remove(0);
        let type_name = first.get_type_name();
        match type_name {
            "list" => {
                let first = first.downcast::<ListValue>().unwrap();
                let mut vec: Vec<Box<dyn Value>> = Vec::with_capacity(first.values.len());
                for (i, value) in first.values.into_iter().enumerate() {
                    let index = Box::new(IntegerValue { value: i as i64 });
                    vec.push(second.call(vec![value, index])?);
                }
                ListValue::boxed(vec)
            }
            "dict" => {
                let first = first.downcast::<DictValue>().unwrap();
                let mut dict: HashMap<String, Box<dyn Value>> =
                    HashMap::with_capacity(first.values.len());
                for (key, value) in first.values.into_iter() {
                    let key_value = Box::new(StringValue { value: key.clone() });
                    dict.insert(key, second.call(vec![key_value, value])?);
                }
                DictValue::boxed(dict)
            }
            _ => {
                CashError::InvalidArguments(type_name.to_owned(), "List or Dict".to_owned()).boxed()
            }
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 2).boxed()
    }
}

pub fn each_closure(params: Vec<Box<dyn Value>>) -> ValueResult {
    map_closure(params)?;
    NoneValue::boxed()
}

/// Takes exactly 2 params
/// first: a list/dict to be filtered
/// second: the function, which will receive either the value and index of the list item, or the key and value of the dict item
pub fn filter_closure(mut params: Vec<Box<dyn Value>>) -> ValueResult {
    if params.len() == 2 {
        let first = params.remove(0);
        let second = params.remove(0);
        let type_name = first.get_type_name();
        match type_name {
            "list" => {
                let first = first.downcast::<ListValue>().unwrap();
                let mut vec: Vec<Box<dyn Value>> = Vec::with_capacity(first.values.len());
                for (i, value) in first.values.into_iter().enumerate() {
                    let index = Box::new(IntegerValue { value: i as i64 });
                    let res = second.call(vec![value.clone(), index])?;
                    let res_type = res.get_type_name();
                    if let Ok(boolean) = res.downcast::<BooleanValue>() {
                        if boolean.value {
                            vec.push(value);
                        }
                    } else {
                        return CashError::InvalidType(res_type.to_owned(), "Boolean".to_owned())
                            .boxed();
                    }
                }
                ListValue::boxed(vec)
            }
            "dict" => {
                let first = first.downcast::<DictValue>().unwrap();
                let mut dict: HashMap<String, Box<dyn Value>> =
                    HashMap::with_capacity(first.values.len());
                for (key, value) in first.values.into_iter() {
                    let key_value = Box::new(StringValue { value: key.clone() });
                    let res = second.call(vec![key_value, value.clone()])?;
                    let res_type = res.get_type_name();
                    if let Ok(boolean) = res.downcast::<BooleanValue>() {
                        if boolean.value {
                            dict.insert(key, value);
                        }
                    } else {
                        return CashError::InvalidType(res_type.to_owned(), "Boolean".to_owned())
                            .boxed();
                    }
                }
                DictValue::boxed(dict)
            }
            _ => {
                CashError::InvalidArguments(type_name.to_owned(), "List or Dict".to_owned()).boxed()
            }
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 2).boxed()
    }
}

/// Takes exactly 1 param
/// first: a list/dict
pub fn len_closure(mut params: Vec<Box<dyn Value>>) -> ValueResult {
    if params.len() == 1 {
        let first = params.remove(0);
        let type_name = first.get_type_name();
        match type_name {
            "list" => {
                let first = first.downcast::<ListValue>().unwrap();
                IntegerValue::boxed(first.values.len() as i64)
            }
            "dict" => {
                let first = first.downcast::<DictValue>().unwrap();
                IntegerValue::boxed(first.values.len() as i64)
            }
            _ => {
                CashError::InvalidArguments(type_name.to_owned(), "List or Dict".to_owned()).boxed()
            }
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 1).boxed()
    }
}