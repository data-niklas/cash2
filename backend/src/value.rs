use downcast_rs::{impl_downcast, Downcast};
use std::error::Error;

pub type ValueResult = Result<Box<dyn Value>, Box<dyn Error>>;

pub trait Value: Downcast + std::fmt::Display {
    fn get_type_name(&self) -> &'static str;
    fn index(&self, index: usize) -> ValueResult;
    fn call(&self, params: Vec<Box<dyn Value>>) -> ValueResult;
    fn not(&self) -> ValueResult;
    fn uplus(&self) -> ValueResult;
    fn uminus(&self) -> ValueResult;
    fn r#await(&self) -> ValueResult;
    fn power(&self, value: Box<dyn Value>) -> ValueResult;
    fn multiply(&self, value: Box<dyn Value>) -> ValueResult;
    fn division(&self, value: Box<dyn Value>) -> ValueResult;
    fn modulo(&self, value: Box<dyn Value>) -> ValueResult;
    fn add(&self, value: Box<dyn Value>) -> ValueResult;
    fn subtract(&self, value: Box<dyn Value>) -> ValueResult;
    fn bit_shift_l(&self, value: Box<dyn Value>) -> ValueResult;
    fn bit_shift_r(&self, value: Box<dyn Value>) -> ValueResult;
    fn contains(&self, value: Box<dyn Value>) -> ValueResult;
    fn lt(&self, value: Box<dyn Value>) -> ValueResult;
    fn gt(&self, value: Box<dyn Value>) -> ValueResult;
    fn lte(&self, value: Box<dyn Value>) -> ValueResult;
    fn gte(&self, value: Box<dyn Value>) -> ValueResult;
    fn eq(&self, value: Box<dyn Value>) -> ValueResult;
    fn ne(&self, value: Box<dyn Value>) -> ValueResult;
    fn and(&self, value: Box<dyn Value>) -> ValueResult;
    fn xor(&self, value: Box<dyn Value>) -> ValueResult;
    fn or(&self, value: Box<dyn Value>) -> ValueResult;
    fn r#async(&self) -> ValueResult;
}
impl_downcast!(Value);
