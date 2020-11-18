use super::*;
use crate::error::CashError;
use crate::value::{Value, ValueResult};
use crate::values::{BuiltInFunction, NoneValue, StringValue};

fn print_closure(params: Vec<Box<dyn Value>>) -> ValueResult {
    for param in params {
        print!("{}", param);
    }
    println!();
    NoneValue::boxed()
}

fn type_closure(mut params: Vec<Box<dyn Value>>) -> ValueResult {
    if params.len() == 1 {
        StringValue::boxed(params.remove(0).get_type_name().to_owned())
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

        //Math
        "sqrt" => BuiltInFunction::boxed(&sqrt_closure),
        _ => None,
    }
}
