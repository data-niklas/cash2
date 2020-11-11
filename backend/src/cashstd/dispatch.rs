use crate::value::{Value, ValueResult};
use crate::values::{BuiltInFunction, NoneValue};

fn print_closure(params: Vec<Box<dyn Value>>) -> ValueResult {
    for param in params {
        print!("{}", param);
    }
    println!("");
    NoneValue::boxed()
}

pub fn get_stdlib_function(ident: &str) -> Option<Box<dyn Value>> {
    match ident {
        "print" => BuiltInFunction::boxed(&print_closure),
        _ => None,
    }
}
