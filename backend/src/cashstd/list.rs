use crate::context::LockableContext;
use crate::error::CashError;
use crate::value::{Value, ValueResult};
use crate::values::{BooleanValue, DictValue, IntegerValue, ListValue, NoneValue, StringValue};
use std::collections::HashMap;

/// Takes exactly 2 params
/// first: a list/dict to be mapped
/// second: the function, which will receive either the value and index of the list item, or the key and value of the dict item
pub fn map_closure(mut params: Vec<Box<dyn Value>>, ctx: LockableContext) -> ValueResult {
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
                    vec.push(second.call(vec![value, index], ctx.clone())?);
                }
                ListValue::boxed(vec)
            }
            "dict" => {
                let first = first.downcast::<DictValue>().unwrap();
                let mut dict: HashMap<String, Box<dyn Value>> =
                    HashMap::with_capacity(first.values.len());
                for (key, value) in first.values.into_iter() {
                    let key_value = Box::new(StringValue { value: key.clone() });
                    dict.insert(key, second.call(vec![key_value, value], ctx.clone())?);
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

pub fn each_closure(params: Vec<Box<dyn Value>>, ctx: LockableContext) -> ValueResult {
    map_closure(params, ctx)?;
    NoneValue::boxed()
}

/// Takes exactly 2 params
/// first: a list/dict to be filtered
/// second: the function, which will receive either the value and index of the list item, or the key and value of the dict item
pub fn filter_closure(mut params: Vec<Box<dyn Value>>, ctx: LockableContext) -> ValueResult {
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
                    let res = second.call(vec![value.clone(), index], ctx.clone())?;
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
                    let res = second.call(vec![key_value, value.clone()], ctx.clone())?;
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

pub fn reduce_closure(mut params: Vec<Box<dyn Value>>, ctx: LockableContext) -> ValueResult {
    if params.len() == 3 {
        let first = params.remove(0);
        let second = params.remove(0);
        let mut third = params.remove(0);
        let type_name = first.get_type_name();
        if let Ok(first) = first.downcast::<ListValue>() {
            for (i, value) in first.values.into_iter().enumerate() {
                let index = Box::new(IntegerValue { value: i as i64 });
                third = second.call(vec![third, value, index], ctx.clone())?;
            }
            Ok(third)
        } else {
            CashError::InvalidArguments(type_name.to_owned(), "List".to_owned()).boxed()
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 4).boxed()
    }
}

/// Takes exactly 1 param
/// first: a list/dict
pub fn len_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
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

pub fn insert_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 3 {
        let first = params.remove(0);
        let second = params.remove(0);
        let third = params.remove(0);
        let type_name = first.get_type_name();
        match type_name {
            "list" => {
                let first = first.downcast::<ListValue>().unwrap();
                first.insert(&second, &third)
            }
            "dict" => {
                let first = first.downcast::<DictValue>().unwrap();
                first.insert(&second, &third)
            }
            _ => {
                CashError::InvalidArguments(type_name.to_owned(), "List or Dict".to_owned()).boxed()
            }
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 3).boxed()
    }
}

pub fn remove_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 2 {
        let first = params.remove(0);
        let second = params.remove(0);
        let type_name = first.get_type_name();
        match type_name {
            "list" => {
                let first = first.downcast::<ListValue>().unwrap();
                first.remove(&second)
            }
            "dict" => {
                let first = first.downcast::<DictValue>().unwrap();
                first.remove(&second)
            }
            _ => {
                CashError::InvalidArguments(type_name.to_owned(), "List or Dict".to_owned()).boxed()
            }
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 2).boxed()
    }
}

pub fn push_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 2 {
        let first = params.remove(0);
        let second = params.remove(0);
        let type_name = first.get_type_name();
        if let Ok(first) = first.downcast::<ListValue>() {
            first.add(&second)
        } else {
            CashError::InvalidArguments(type_name.to_owned(), "List".to_owned()).boxed()
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 2).boxed()
    }
}

pub fn pop_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 1 {
        let first = params.remove(0);
        let type_name = first.get_type_name();
        if let Ok(first) = first.downcast::<ListValue>() {
            let index = first.values.len() - 1;
            first.remove(&IntegerValue::boxed((index) as i64)?)
        } else {
            CashError::InvalidArguments(type_name.to_owned(), "List".to_owned()).boxed()
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 1).boxed()
    }
}

pub fn join_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 2 {
        let first = params.remove(0);
        let second = params.remove(0);
        let first_type_name = first.get_type_name();
        let second_type_name = first.get_type_name();
        if let Ok(first) = first.downcast::<ListValue>() {
            if let Ok(second) = second.downcast::<StringValue>() {
                let mut joined = String::new();
                for (index, item) in first.values.into_iter().enumerate() {
                    if index != 0 {
                        joined += &second.value;
                    }
                    joined += &item.to_string();
                }
                StringValue::boxed(joined)
            } else {
                CashError::InvalidArguments(second_type_name.to_owned(), "String".to_owned())
                    .boxed()
            }
        } else {
            CashError::InvalidArguments(first_type_name.to_owned(), "List".to_owned()).boxed()
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 2).boxed()
    }
}
