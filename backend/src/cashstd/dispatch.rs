use super::*;
use crate::error::CashError;
use crate::context::Context;
use crate::value::{Value, ValueResult};
use crate::values::{BuiltInFunction, NoneValue, StringValue, BooleanValue, ListValue, DictValue};

use std::sync::{Arc, RwLock};

fn print_closure(params: Vec<Box<dyn Value>>, _ctx:Arc<RwLock<Context>>) -> ValueResult {
    for param in params {
        print!("{}", param);
    }
    println!();
    NoneValue::boxed()
}

fn type_closure(mut params: Vec<Box<dyn Value>>, _ctx: Arc<RwLock<Context>>) -> ValueResult {
    if params.len() == 1 {
        StringValue::boxed(params.remove(0).get_type_name().to_owned())
    } else {
        CashError::InvalidParameterCount(params.len(), 1).boxed()
    }
}

fn exists_closure(mut params: Vec<Box<dyn Value>>, ctx: Arc<RwLock<Context>>) -> ValueResult {
    if params.len() == 1 {
        let value = format!("{}", params.remove(0));
        BooleanValue::boxed(ctx.read().expect("Could not read from Context").exists(&value))
    } else if params.len() == 2 {
        let value = params.remove(0);
        let second = params.remove(0);
        if let Some(value) = value.downcast_ref::<ListValue>() {
            value.exists(&second)
        } else if let Some(value) = value.downcast_ref::<DictValue>() {
            value.contains(&second) // contains checks for key
        } else {
            CashError::InvalidArguments(value.get_type_name().to_owned(), "list or dict".to_owned()).boxed()
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 1).boxed()
    }
}

pub fn get_stdlib_function(ident: &str) -> Option<Box<dyn Value>> {
    match ident {
        "print" => BuiltInFunction::boxed(&print_closure),
        "each" => BuiltInFunction::boxed(&each_closure),
        "map" => BuiltInFunction::boxed(&map_closure),
        "filter" => BuiltInFunction::boxed(&filter_closure),
        "len" => BuiltInFunction::boxed(&len_closure),

        "type" => BuiltInFunction::boxed(&type_closure),
        "exists" => BuiltInFunction::boxed(&exists_closure),

        //Math
        "sqrt" => BuiltInFunction::boxed(&sqrt_closure),
        _ => None,
    }
}
