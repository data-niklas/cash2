use crate::context::Context;
use crate::context::LockableContext;
use crate::value::{Value, ValueResult};
use std::sync::Arc;

pub struct BuiltInFunction<
    F: 'static + Fn(Vec<Box<dyn Value>>, LockableContext) -> ValueResult + Send + Sync + Sync,
> {
    pub closure: &'static F,
}

impl<F: 'static + Fn(Vec<Box<dyn Value>>, LockableContext) -> ValueResult + Send + Sync + Sync>
    Value for BuiltInFunction<F>
{
    fn get_type_name(&self) -> &'static str {
        "builtin_function"
    }

    fn call(&self, params: Vec<Box<dyn Value>>, ctx: LockableContext) -> ValueResult {
        (self.closure)(params, ctx)
    }
    fn clone(&self) -> Box<dyn Value> {
        Box::new(Self {
            closure: self.closure,
        })
    }
}

impl<F: 'static + Fn(Vec<Box<dyn Value>>, LockableContext) -> ValueResult + Send + Sync + Sync>
    BuiltInFunction<F>
{
    // different boxed than in other value types
    pub fn boxed(closure: &'static F) -> Option<Box<dyn Value>> {
        Some(Box::new(BuiltInFunction { closure }))
    }
}

impl<F: 'static + Fn(Vec<Box<dyn Value>>, LockableContext) -> ValueResult + Send + Sync + Sync>
    std::fmt::Debug for BuiltInFunction<F>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_type_name())
    }
}

impl<F: 'static + Fn(Vec<Box<dyn Value>>, LockableContext) -> ValueResult + Send + Sync + Sync>
    std::fmt::Display for BuiltInFunction<F>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_type_name())
    }
}
