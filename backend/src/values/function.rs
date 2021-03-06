use crate::ast::Node;
use crate::context::Context;
use crate::value::{Value, ValueResult};
use crate::values::ReturnValue;

use crate::error::CashError;

use crate::context::LockableContext;
use std::sync::Arc;

#[derive(Debug)]
pub struct FunctionValue {
    pub node: Arc<dyn Node>,
    pub ctx: LockableContext,
    pub params: Vec<(String, Option<Box<dyn Value>>)>,
}

impl Value for FunctionValue {
    fn get_type_name(&self) -> &'static str {
        "function"
    }

    fn call(&self, params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
        let ctx = Context::from_parent(self.ctx.clone());
        let param_count = params.len();
        if param_count > self.params.len() {
            return CashError::InvalidParameterCount(param_count, self.params.len()).boxed();
        }
        let mut user_values = params.into_iter();
        {
            let mut ctx_lock = ctx.write();
            let mut iter = self.params.iter();
            while let Some((name, val)) = iter.next() {
                if let Some(user_value) = user_values.next() {
                    ctx_lock.set(name, user_value);
                } else if let Some(default_value) = val {
                    ctx_lock.set(name, (*default_value).clone());
                } else {
                    let mut new_params = Vec::new();
                    new_params.push((name.to_string(), None));
                    while let Some((name, val)) = iter.next() {
                        if let Some(val) = val {
                            new_params.push((name.to_string(), Some((*val).clone())));
                        } else {
                            new_params.push((name.to_string(), None));
                        }
                    }
                    drop(ctx_lock);
                    return FunctionValue::boxed(self.node.clone(), ctx, new_params);
                    //return CashError::InvalidParameterCount(param_count, self.params.len())
                    // .boxed();
                }
            }
        }
        let value = self.node.eval(ctx)?;
        if value.get_type_name() == "return" {
            let value = value.downcast::<ReturnValue>().unwrap();
            Ok(value.value)
        } else {
            Ok(value)
        }
    }
    fn clone(&self) -> Box<dyn Value> {
        let mut params = Vec::with_capacity(self.params.len());
        for (name, value) in &self.params {
            let mut optional = None;
            if let Some(value) = value {
                optional = Some((*value).clone());
            }
            params.push((name.clone(), optional));
        }
        Box::new(Self {
            node: self.node.clone(),
            ctx: self.ctx.clone(),
            params,
        })
    }
}

impl FunctionValue {
    // different boxed than in other value types
    pub fn boxed(
        node: Arc<dyn Node>,
        ctx: LockableContext,
        params: Vec<(String, Option<Box<dyn Value>>)>,
    ) -> ValueResult {
        Ok(Box::new(FunctionValue { node, ctx, params }))
    }
}

impl std::fmt::Display for FunctionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_type_name())
    }
}
