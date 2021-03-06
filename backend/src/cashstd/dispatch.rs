use super::*;
use crate::error::CashError;
use crate::value::{Value, ValueResult};
use crate::values::{BooleanValue, BuiltInFunction, DictValue, ListValue, NoneValue, StringValue};
use std::env;

use crate::context::LockableContext;

fn print_closure(params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    for param in params {
        print!("{}", param);
    }
    println!();
    NoneValue::boxed()
}

fn type_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 1 {
        StringValue::boxed(params.remove(0).get_type_name().to_owned())
    } else {
        CashError::InvalidParameterCount(params.len(), 1).boxed()
    }
}

fn exists_closure(mut params: Vec<Box<dyn Value>>, ctx: LockableContext) -> ValueResult {
    if params.len() == 1 {
        let value = format!("{}", params.remove(0));
        BooleanValue::boxed(ctx.read().exists(&value))
    } else if params.len() == 2 {
        let value = params.remove(0);
        let second = params.remove(0);
        if let Some(value) = value.downcast_ref::<ListValue>() {
            value.exists(&second)
        } else if let Some(value) = value.downcast_ref::<DictValue>() {
            value.contains(&second) // contains checks for key
        } else {
            CashError::InvalidArguments(value.get_type_name().to_owned(), "list or dict".to_owned())
                .boxed()
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 1).boxed()
    }
}

fn cwd_closure(params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.is_empty() {
        StringValue::boxed(
            env::current_dir()
                .expect("Could not get current working directory")
                .to_str()
                .expect("Could not convert Pathbuf to String")
                .to_owned(),
        )
    } else {
        CashError::InvalidParameterCount(params.len(), 0).boxed()
    }
}

fn cd_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 1 {
        env::set_current_dir(std::path::Path::new(&params.remove(0).to_string()))?;
        NoneValue::boxed()
    } else {
        CashError::InvalidParameterCount(params.len(), 1).boxed()
    }
}

pub fn get_stdlib_function(ident: &str) -> Option<Box<dyn Value>> {
    match ident {
        "cd" => BuiltInFunction::boxed(&cd_closure),
        "cwd" => BuiltInFunction::boxed(&cwd_closure),

        "print" => BuiltInFunction::boxed(&print_closure),
        "each" => BuiltInFunction::boxed(&each_closure),
        "map" => BuiltInFunction::boxed(&map_closure),
        "filter" => BuiltInFunction::boxed(&filter_closure),
        "reduce" => BuiltInFunction::boxed(&reduce_closure),
        "join" => BuiltInFunction::boxed(&join_closure),
        "len" => BuiltInFunction::boxed(&len_closure),

        "remove" => BuiltInFunction::boxed(&remove_closure),
        "insert" => BuiltInFunction::boxed(&insert_closure),
        "push" => BuiltInFunction::boxed(&push_closure),
        "pop" => BuiltInFunction::boxed(&pop_closure),

        "type" => BuiltInFunction::boxed(&type_closure),
        "exists" => BuiltInFunction::boxed(&exists_closure),

        //Math
        "sqrt" => BuiltInFunction::boxed(&sqrt_closure),
        "abs" => BuiltInFunction::boxed(&abs_closure),
        "ceil" => BuiltInFunction::boxed(&ceil_closure),
        "floor" => BuiltInFunction::boxed(&floor_closure),
        "trunc" => BuiltInFunction::boxed(&trunc_closure),
        "round" => BuiltInFunction::boxed(&round_closure),
        "exp" => BuiltInFunction::boxed(&exp_closure),
        "ln" => BuiltInFunction::boxed(&ln_closure),
        "is_nan" => BuiltInFunction::boxed(&isnan_closure),
        "sign" => BuiltInFunction::boxed(&sign_closure),
        "max" => BuiltInFunction::boxed(&max_closure),
        "min" => BuiltInFunction::boxed(&min_closure),
        "rand" => BuiltInFunction::boxed(&rand_closure),

        //trig
        "sin" => BuiltInFunction::boxed(&sin_closure),
        "cos" => BuiltInFunction::boxed(&cos_closure),
        "tan" => BuiltInFunction::boxed(&tan_closure),
        "asin" => BuiltInFunction::boxed(&asin_closure),
        "acos" => BuiltInFunction::boxed(&acos_closure),
        "atan" => BuiltInFunction::boxed(&atan_closure),
        "sinh" => BuiltInFunction::boxed(&sinh_closure),
        "cosh" => BuiltInFunction::boxed(&cosh_closure),
        "tanh" => BuiltInFunction::boxed(&tanh_closure),
        "asinh" => BuiltInFunction::boxed(&asinh_closure),
        "acosh" => BuiltInFunction::boxed(&acosh_closure),
        "atanh" => BuiltInFunction::boxed(&atanh_closure),
        _ => None,
    }
}
