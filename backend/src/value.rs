use crate::error::CashError;
use downcast_rs::{impl_downcast, Downcast};
use std::error::Error;

pub type ValueResult = Result<Box<dyn Value>, Box<dyn Error>>;

pub trait Value: Downcast + std::fmt::Display {
    fn get_type_name(&self) -> &'static str;
    fn index(&self, _index: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("indexing".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn call(&self, _params: Vec<Box<dyn Value>>) -> ValueResult {
        CashError::InvalidOperation("function call".to_owned(), self.get_type_name().to_owned())
            .boxed()
    }
    fn not(&self) -> ValueResult {
        CashError::InvalidOperation("not".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn uplus(&self) -> ValueResult {
        CashError::InvalidOperation("unary plus".to_owned(), self.get_type_name().to_owned())
            .boxed()
    }
    fn uminus(&self) -> ValueResult {
        CashError::InvalidOperation("unary minus".to_owned(), self.get_type_name().to_owned())
            .boxed()
    }
    fn r#await(&self) -> ValueResult {
        CashError::InvalidOperation("await".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn power(&self, _value: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("exponentiation".to_owned(), self.get_type_name().to_owned())
            .boxed()
    }
    fn multiply(&self, _value: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("multiplication".to_owned(), self.get_type_name().to_owned())
            .boxed()
    }
    fn division(&self, _value: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("division".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn modulo(&self, _value: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("modulo".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn add(&self, _value: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("add".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn subtract(&self, _value: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("subtract".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn bit_shift_l(&self, _value: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("bit shift left".to_owned(), self.get_type_name().to_owned())
            .boxed()
    }
    fn bit_shift_r(&self, _value: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation(
            "bit shift right".to_owned(),
            self.get_type_name().to_owned(),
        )
        .boxed()
    }
    fn contains(&self, _value: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("contains".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn lt(&self, _value: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("less than".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn gt(&self, _value: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("greater than".to_owned(), self.get_type_name().to_owned())
            .boxed()
    }
    fn lte(&self, _value: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation(
            "less than equals".to_owned(),
            self.get_type_name().to_owned(),
        )
        .boxed()
    }
    fn gte(&self, _value: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation(
            "greater than equals".to_owned(),
            self.get_type_name().to_owned(),
        )
        .boxed()
    }
    fn eq(&self, _value: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("equals".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn ne(&self, _value: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("not equals".to_owned(), self.get_type_name().to_owned())
            .boxed()
    }
    fn and(&self, _value: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("and".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn xor(&self, _value: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("xor".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn or(&self, _value: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("or".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn r#async(&self) -> ValueResult {
        CashError::InvalidOperation("async".to_owned(), self.get_type_name().to_owned()).boxed()
    }
}
impl_downcast!(Value);
