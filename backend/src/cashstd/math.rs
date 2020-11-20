use crate::context::Context;
use crate::context::LockableContext;
use crate::error::CashError;
use crate::value::{Value, ValueResult};
use crate::values::{BooleanValue, FloatValue, IntegerValue};
use rand::Rng;
use std::sync::Arc;

pub fn sqrt_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
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

pub fn floor_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 1 {
        let first = params.remove(0);
        let type_name = first.get_type_name();
        match type_name {
            "float" => {
                let first = first.downcast::<FloatValue>().unwrap();
                IntegerValue::boxed(first.value.floor() as i64)
            }
            "integer" => Ok(first),
            _ => CashError::InvalidArguments(type_name.to_owned(), "Float or Integer".to_owned())
                .boxed(),
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 1).boxed()
    }
}

pub fn ceil_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 1 {
        let first = params.remove(0);
        let type_name = first.get_type_name();
        match type_name {
            "float" => {
                let first = first.downcast::<FloatValue>().unwrap();
                IntegerValue::boxed(first.value.ceil() as i64)
            }
            "integer" => Ok(first),
            _ => CashError::InvalidArguments(type_name.to_owned(), "Float or Integer".to_owned())
                .boxed(),
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 1).boxed()
    }
}

pub fn round_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 1 {
        let first = params.remove(0);
        let type_name = first.get_type_name();
        match type_name {
            "float" => {
                let first = first.downcast::<FloatValue>().unwrap();
                IntegerValue::boxed(first.value.round() as i64)
            }
            "integer" => Ok(first),
            _ => CashError::InvalidArguments(type_name.to_owned(), "Float or Integer".to_owned())
                .boxed(),
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 1).boxed()
    }
}

pub fn trunc_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 1 {
        let first = params.remove(0);
        let type_name = first.get_type_name();
        match type_name {
            "float" => {
                let first = first.downcast::<FloatValue>().unwrap();
                IntegerValue::boxed(first.value.trunc() as i64)
            }
            "integer" => Ok(first),
            _ => CashError::InvalidArguments(type_name.to_owned(), "Float or Integer".to_owned())
                .boxed(),
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 1).boxed()
    }
}

pub fn abs_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 1 {
        let first = params.remove(0);
        let type_name = first.get_type_name();
        match type_name {
            "float" => {
                let first = first.downcast::<FloatValue>().unwrap();
                FloatValue::boxed(first.value.abs())
            }
            "integer" => {
                let first = first.downcast::<IntegerValue>().unwrap();
                IntegerValue::boxed(first.value.abs())
            }
            _ => CashError::InvalidArguments(type_name.to_owned(), "Float or Integer".to_owned())
                .boxed(),
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 1).boxed()
    }
}

pub fn sign_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 1 {
        let first = params.remove(0);
        let type_name = first.get_type_name();
        match type_name {
            "float" => {
                let first = first.downcast::<FloatValue>().unwrap();
                FloatValue::boxed(first.value.signum())
            }
            "integer" => {
                let first = first.downcast::<IntegerValue>().unwrap();
                IntegerValue::boxed(first.value.signum())
            }
            _ => CashError::InvalidArguments(type_name.to_owned(), "Float or Integer".to_owned())
                .boxed(),
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 1).boxed()
    }
}

pub fn exp_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 1 {
        let first = params.remove(0);
        let type_name = first.get_type_name();
        match type_name {
            "float" => {
                let first = first.downcast::<FloatValue>().unwrap();
                FloatValue::boxed(first.value.exp())
            }
            "integer" => {
                let first = first.downcast::<IntegerValue>().unwrap();
                FloatValue::boxed((first.value as f64).exp())
            }
            _ => CashError::InvalidArguments(type_name.to_owned(), "Float or Integer".to_owned())
                .boxed(),
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 1).boxed()
    }
}

pub fn ln_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 1 {
        let first = params.remove(0);
        let type_name = first.get_type_name();
        match type_name {
            "float" => {
                let first = first.downcast::<FloatValue>().unwrap();
                FloatValue::boxed(first.value.ln())
            }
            "integer" => {
                let first = first.downcast::<IntegerValue>().unwrap();
                FloatValue::boxed((first.value as f64).ln())
            }
            _ => CashError::InvalidArguments(type_name.to_owned(), "Float or Integer".to_owned())
                .boxed(),
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 1).boxed()
    }
}

pub fn sin_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 1 {
        let first = params.remove(0);
        let type_name = first.get_type_name();
        match type_name {
            "float" => {
                let first = first.downcast::<FloatValue>().unwrap();
                FloatValue::boxed(first.value.sin())
            }
            "integer" => {
                let first = first.downcast::<IntegerValue>().unwrap();
                FloatValue::boxed((first.value as f64).sin())
            }
            _ => CashError::InvalidArguments(type_name.to_owned(), "Float or Integer".to_owned())
                .boxed(),
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 1).boxed()
    }
}

pub fn cos_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 1 {
        let first = params.remove(0);
        let type_name = first.get_type_name();
        match type_name {
            "float" => {
                let first = first.downcast::<FloatValue>().unwrap();
                FloatValue::boxed(first.value.cos())
            }
            "integer" => {
                let first = first.downcast::<IntegerValue>().unwrap();
                FloatValue::boxed((first.value as f64).cos())
            }
            _ => CashError::InvalidArguments(type_name.to_owned(), "Float or Integer".to_owned())
                .boxed(),
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 1).boxed()
    }
}

pub fn tan_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 1 {
        let first = params.remove(0);
        let type_name = first.get_type_name();
        match type_name {
            "float" => {
                let first = first.downcast::<FloatValue>().unwrap();
                FloatValue::boxed(first.value.tan())
            }
            "integer" => {
                let first = first.downcast::<IntegerValue>().unwrap();
                FloatValue::boxed((first.value as f64).tan())
            }
            _ => CashError::InvalidArguments(type_name.to_owned(), "Float or Integer".to_owned())
                .boxed(),
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 1).boxed()
    }
}

pub fn asin_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 1 {
        let first = params.remove(0);
        let type_name = first.get_type_name();
        match type_name {
            "float" => {
                let first = first.downcast::<FloatValue>().unwrap();
                FloatValue::boxed(first.value.asin())
            }
            "integer" => {
                let first = first.downcast::<IntegerValue>().unwrap();
                FloatValue::boxed((first.value as f64).asin())
            }
            _ => CashError::InvalidArguments(type_name.to_owned(), "Float or Integer".to_owned())
                .boxed(),
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 1).boxed()
    }
}

pub fn acos_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 1 {
        let first = params.remove(0);
        let type_name = first.get_type_name();
        match type_name {
            "float" => {
                let first = first.downcast::<FloatValue>().unwrap();
                FloatValue::boxed(first.value.acos())
            }
            "integer" => {
                let first = first.downcast::<IntegerValue>().unwrap();
                FloatValue::boxed((first.value as f64).acos())
            }
            _ => CashError::InvalidArguments(type_name.to_owned(), "Float or Integer".to_owned())
                .boxed(),
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 1).boxed()
    }
}

pub fn atan_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 1 {
        let first = params.remove(0);
        let type_name = first.get_type_name();
        match type_name {
            "float" => {
                let first = first.downcast::<FloatValue>().unwrap();
                FloatValue::boxed(first.value.atan())
            }
            "integer" => {
                let first = first.downcast::<IntegerValue>().unwrap();
                FloatValue::boxed((first.value as f64).atan())
            }
            _ => CashError::InvalidArguments(type_name.to_owned(), "Float or Integer".to_owned())
                .boxed(),
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 1).boxed()
    }
}

pub fn sinh_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 1 {
        let first = params.remove(0);
        let type_name = first.get_type_name();
        match type_name {
            "float" => {
                let first = first.downcast::<FloatValue>().unwrap();
                FloatValue::boxed(first.value.sinh())
            }
            "integer" => {
                let first = first.downcast::<IntegerValue>().unwrap();
                FloatValue::boxed((first.value as f64).sinh())
            }
            _ => CashError::InvalidArguments(type_name.to_owned(), "Float or Integer".to_owned())
                .boxed(),
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 1).boxed()
    }
}

pub fn cosh_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 1 {
        let first = params.remove(0);
        let type_name = first.get_type_name();
        match type_name {
            "float" => {
                let first = first.downcast::<FloatValue>().unwrap();
                FloatValue::boxed(first.value.cosh())
            }
            "integer" => {
                let first = first.downcast::<IntegerValue>().unwrap();
                FloatValue::boxed((first.value as f64).cosh())
            }
            _ => CashError::InvalidArguments(type_name.to_owned(), "Float or Integer".to_owned())
                .boxed(),
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 1).boxed()
    }
}

pub fn tanh_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 1 {
        let first = params.remove(0);
        let type_name = first.get_type_name();
        match type_name {
            "float" => {
                let first = first.downcast::<FloatValue>().unwrap();
                FloatValue::boxed(first.value.tanh())
            }
            "integer" => {
                let first = first.downcast::<IntegerValue>().unwrap();
                FloatValue::boxed((first.value as f64).tanh())
            }
            _ => CashError::InvalidArguments(type_name.to_owned(), "Float or Integer".to_owned())
                .boxed(),
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 1).boxed()
    }
}

pub fn asinh_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 1 {
        let first = params.remove(0);
        let type_name = first.get_type_name();
        match type_name {
            "float" => {
                let first = first.downcast::<FloatValue>().unwrap();
                FloatValue::boxed(first.value.asinh())
            }
            "integer" => {
                let first = first.downcast::<IntegerValue>().unwrap();
                FloatValue::boxed((first.value as f64).asinh())
            }
            _ => CashError::InvalidArguments(type_name.to_owned(), "Float or Integer".to_owned())
                .boxed(),
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 1).boxed()
    }
}

pub fn acosh_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 1 {
        let first = params.remove(0);
        let type_name = first.get_type_name();
        match type_name {
            "float" => {
                let first = first.downcast::<FloatValue>().unwrap();
                FloatValue::boxed(first.value.acosh())
            }
            "integer" => {
                let first = first.downcast::<IntegerValue>().unwrap();
                FloatValue::boxed((first.value as f64).acosh())
            }
            _ => CashError::InvalidArguments(type_name.to_owned(), "Float or Integer".to_owned())
                .boxed(),
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 1).boxed()
    }
}

pub fn atanh_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 1 {
        let first = params.remove(0);
        let type_name = first.get_type_name();
        match type_name {
            "float" => {
                let first = first.downcast::<FloatValue>().unwrap();
                FloatValue::boxed(first.value.atanh())
            }
            "integer" => {
                let first = first.downcast::<IntegerValue>().unwrap();
                FloatValue::boxed((first.value as f64).atanh())
            }
            _ => CashError::InvalidArguments(type_name.to_owned(), "Float or Integer".to_owned())
                .boxed(),
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 1).boxed()
    }
}

pub fn isnan_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 1 {
        let first = params.remove(0);
        let type_name = first.get_type_name();
        match type_name {
            "float" => {
                let first = first.downcast::<FloatValue>().unwrap();
                BooleanValue::boxed(first.value.is_nan())
            }
            "integer" => BooleanValue::boxed(false),
            _ => CashError::InvalidArguments(type_name.to_owned(), "Float or Integer".to_owned())
                .boxed(),
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 1).boxed()
    }
}

pub fn max_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 2 {
        let first = params.remove(0);
        let second = params.remove(0);
        let type_name = first.get_type_name();
        if type_name != "float" && type_name != "integer" {
            return CashError::InvalidArguments(
                type_name.to_owned(),
                "Float or Integer".to_owned(),
            )
            .boxed();
        }
        let type_name = second.get_type_name();
        if type_name != "float" && type_name != "integer" {
            return CashError::InvalidArguments(
                type_name.to_owned(),
                "Float or Integer".to_owned(),
            )
            .boxed();
        }
        let result = first.gt(&second)?;
        if let Ok(first_greater) = result.downcast::<BooleanValue>() {
            if first_greater.value {
                Ok(first)
            } else {
                Ok(second)
            }
        } else {
            CashError::Bug("Greater than did not return boolean".to_owned()).boxed()
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 2).boxed()
    }
}

pub fn min_closure(mut params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.len() == 2 {
        let first = params.remove(0);
        let second = params.remove(0);
        let type_name = first.get_type_name();
        if type_name != "float" && type_name != "integer" {
            return CashError::InvalidArguments(
                type_name.to_owned(),
                "Float or Integer".to_owned(),
            )
            .boxed();
        }
        let type_name = second.get_type_name();
        if type_name != "float" && type_name != "integer" {
            return CashError::InvalidArguments(
                type_name.to_owned(),
                "Float or Integer".to_owned(),
            )
            .boxed();
        }
        let result = first.gt(&second)?;
        if let Ok(first_greater) = result.downcast::<BooleanValue>() {
            if first_greater.value {
                Ok(second)
            } else {
                Ok(first)
            }
        } else {
            CashError::Bug("Greater than did not return boolean".to_owned()).boxed()
        }
    } else {
        CashError::InvalidParameterCount(params.len(), 2).boxed()
    }
}

pub fn rand_closure(params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
    if params.is_empty() {
        FloatValue::boxed(rand::thread_rng().gen_range(0.0, 1.0))
    } else {
        CashError::InvalidParameterCount(params.len(), 0).boxed()
    }
}
