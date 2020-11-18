use crate::error::CashError;
use crate::value::{Value, ValueResult};
use crate::context::Context;
use crate::values::{BooleanValue, DictValue, FloatValue, IntegerValue, ListValue, NoneValue};
use std::sync::{RwLock, Arc};

pub fn sqrt_closure(mut params: Vec<Box<dyn Value>>, _ctx: Arc<RwLock<Context>>) -> ValueResult {
    if params.len() == 1 {
        let first = params.remove(0);
        let type_name = first.get_type_name();
        match type_name {
            "float" => {
                let first = first.downcast::<FloatValue>().unwrap();
                FloatValue::boxed(first.value.sqrt())
            }
            "integer" => {
                let first = first.downcast::<IntegerValue>().unwrap();
                FloatValue::boxed((first.value as f64).sqrt())
            }
            _ => CashError::InvalidArguments(type_name.to_owned(), "Float or Integer".to_owned())
                .boxed(),
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 1).boxed()
    }
}
