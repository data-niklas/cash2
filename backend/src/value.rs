use crate::error::CashError;
use downcast_rs::{impl_downcast, DowncastSync};
use std::error::Error;
use std::sync::{Arc, RwLock};

pub type ValueResult = Result<Arc<dyn Value>, Box<dyn Error>>;

pub trait Value: DowncastSync + std::marker::Sync + std::fmt::Display + std::fmt::Debug {
    fn get_type_name(&self) -> &'static str;
    fn index(&self, _index: Arc<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("indexing".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn call(&self, _params: Vec<Arc<dyn Value>>) -> ValueResult {
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
    fn power(&self, _value: Arc<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("exponentiation".to_owned(), self.get_type_name().to_owned())
            .boxed()
    }
    fn multiply(&self, _value: Arc<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("multiplication".to_owned(), self.get_type_name().to_owned())
            .boxed()
    }
    fn division(&self, _value: Arc<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("division".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn modulo(&self, _value: Arc<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("modulo".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn add(&self, _value: Arc<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("add".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn subtract(&self, _value: Arc<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("subtract".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn bit_shift_l(&self, _value: Arc<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("bit shift left".to_owned(), self.get_type_name().to_owned())
            .boxed()
    }
    fn bit_shift_r(&self, _value: Arc<dyn Value>) -> ValueResult {
        CashError::InvalidOperation(
            "bit shift right".to_owned(),
            self.get_type_name().to_owned(),
        )
        .boxed()
    }
    fn contains(&self, _value: Arc<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("contains".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn lt(&self, _value: Arc<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("less than".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn gt(&self, _value: Arc<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("greater than".to_owned(), self.get_type_name().to_owned())
            .boxed()
    }
    fn lte(&self, _value: Arc<dyn Value>) -> ValueResult {
        CashError::InvalidOperation(
            "less than equals".to_owned(),
            self.get_type_name().to_owned(),
        )
        .boxed()
    }
    fn gte(&self, _value: Arc<dyn Value>) -> ValueResult {
        CashError::InvalidOperation(
            "greater than equals".to_owned(),
            self.get_type_name().to_owned(),
        )
        .boxed()
    }
    fn eq(&self, _value: Arc<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("equals".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn ne(&self, _value: Arc<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("not equals".to_owned(), self.get_type_name().to_owned())
            .boxed()
    }
    fn and(&self, _value: Arc<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("and".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn xor(&self, _value: Arc<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("xor".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn or(&self, _value: Arc<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("or".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn r#async(&self) -> ValueResult {
        CashError::InvalidOperation("async".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn clone(&self) -> Box<dyn Value>;
}

impl_downcast!(sync Value);
