use crate::context::Context;
use crate::context::LockableContext;
use crate::error::CashError;
use downcast_rs::{impl_downcast, DowncastSync};
use std::error::Error;
use std::sync::Arc;

pub type ValueResult = Result<Box<dyn Value>, Box<dyn Error + Send + Sync>>;

pub trait Value: DowncastSync + std::fmt::Display + std::fmt::Debug {
    fn get_type_name(&self) -> &'static str;
    fn indexed_set(
        &mut self,
        _value: Box<dyn Value>,
        _indexes: &[Box<dyn Value>],
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        CashError::InvalidOperation("indexing".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn index(&self, _index: &Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("indexing".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn call(&self, _params: Vec<Box<dyn Value>>, _ctx: LockableContext) -> ValueResult {
        CashError::InvalidOperation("function call".to_owned(), self.get_type_name().to_owned())
            .boxed()
    }
    fn not(self: Box<Self>) -> ValueResult {
        CashError::InvalidOperation("not".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn uplus(self: Box<Self>) -> ValueResult {
        CashError::InvalidOperation("unary plus".to_owned(), self.get_type_name().to_owned())
            .boxed()
    }
    fn uminus(self: Box<Self>) -> ValueResult {
        CashError::InvalidOperation("unary minus".to_owned(), self.get_type_name().to_owned())
            .boxed()
    }
    fn r#await(self: Box<Self>) -> ValueResult {
        CashError::InvalidOperation("await".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn power(self: Box<Self>, _value: &Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("exponentiation".to_owned(), self.get_type_name().to_owned())
            .boxed()
    }
    fn multiply(self: Box<Self>, _value: &Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("multiplication".to_owned(), self.get_type_name().to_owned())
            .boxed()
    }
    fn division(self: Box<Self>, _value: &Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("division".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn modulo(self: Box<Self>, _value: &Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("modulo".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn add(self: Box<Self>, _value: &Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("add".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn subtract(self: Box<Self>, _value: &Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("subtract".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn bit_shift_l(self: Box<Self>, _value: &Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("bit shift left".to_owned(), self.get_type_name().to_owned())
            .boxed()
    }
    fn bit_shift_r(self: Box<Self>, _value: &Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation(
            "bit shift right".to_owned(),
            self.get_type_name().to_owned(),
        )
        .boxed()
    }
    fn contains(&self, _value: &Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("contains".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn lt(&self, _value: &Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("less than".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn gt(&self, _value: &Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("greater than".to_owned(), self.get_type_name().to_owned())
            .boxed()
    }
    fn lte(&self, _value: &Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation(
            "less than equals".to_owned(),
            self.get_type_name().to_owned(),
        )
        .boxed()
    }
    fn gte(&self, _value: &Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation(
            "greater than equals".to_owned(),
            self.get_type_name().to_owned(),
        )
        .boxed()
    }
    fn eq(&self, _value: &Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("equals".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn ne(&self, _value: &Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("not equals".to_owned(), self.get_type_name().to_owned())
            .boxed()
    }
    fn and(self: Box<Self>, _value: &Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("and".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn xor(self: Box<Self>, _value: &Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("xor".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn or(self: Box<Self>, _value: &Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("or".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn clone(&self) -> Box<dyn Value>;
    fn vec(
        self: Box<Self>,
    ) -> Result<Vec<Box<dyn Value>>, Box<dyn std::error::Error + Send + Sync>> {
        CashError::InvalidOperation("vec".to_owned(), self.get_type_name().to_owned()).boxed()
    }
}

impl_downcast!(sync Value);
