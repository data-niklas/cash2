use crate::ast::Node;
use crate::context::Context;
use crate::error::CashError;
use crate::value::{Value, ValueResult};
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
pub struct BooleanLiteral {
    pub value: bool,
}

impl Node for BooleanLiteral {
    fn eval(
        &self,
        ctx: Arc<RwLock<Context>>,
    ) -> Result<Box<dyn Value>, Box<dyn std::error::Error>> {
        BooleanValue::boxed(self.value)
    }
}
impl std::fmt::Display for BooleanLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BooleanLiteral '{}'", self.value)
    }
}

#[derive(Debug)]
pub struct BooleanValue {
    value: bool,
}

impl BooleanValue {
    pub fn boxed(value: bool) -> ValueResult {
        Ok(Box::new(BooleanValue { value }))
    }
}

impl Value for BooleanValue {
    fn get_type_name(&self) -> &'static str {
        "boolean"
    }
    fn index(&self, _: usize) -> ValueResult {
        CashError::InvalidOperation("indexing".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn call(&self, _: Vec<Box<dyn Value>>) -> ValueResult {
        CashError::InvalidOperation("calling".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn not(&self) -> ValueResult {
        Self::boxed(!self.value)
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
    fn power(&self, _: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("power".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn multiply(&self, _: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("multiply".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn division(&self, _: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("division".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn modulo(&self, _: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("modulo".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn add(&self, _: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("add".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn subtract(&self, _: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("subtract".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn bit_shift_l(&self, _: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("left bit shift".to_owned(), self.get_type_name().to_owned())
            .boxed()
    }
    fn bit_shift_r(&self, _: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation(
            "right bit shift".to_owned(),
            self.get_type_name().to_owned(),
        )
        .boxed()
    }
    fn contains(&self, _: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("contains".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn lt(&self, _: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("less than".to_owned(), self.get_type_name().to_owned()).boxed()
    }
    fn gt(&self, _: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation("greater than".to_owned(), self.get_type_name().to_owned())
            .boxed()
    }
    fn lte(&self, _: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation(
            "less / equal than".to_owned(),
            self.get_type_name().to_owned(),
        )
        .boxed()
    }
    fn gte(&self, _: Box<dyn Value>) -> ValueResult {
        CashError::InvalidOperation(
            "greater / equal than".to_owned(),
            self.get_type_name().to_owned(),
        )
        .boxed()
    }
    fn eq(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Ok(other) = value.downcast::<BooleanValue>() {
            BooleanValue::boxed(self.value == other.value)
        } else {
            CashError::InvalidOperation("equality".to_owned(), "boolean ".to_owned() + typename)
                .boxed()
        }
    }
    fn ne(&self, value: Box<dyn Value>) -> ValueResult {
        self.eq(value)?.not()
    }
    fn and(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Ok(other) = value.downcast::<BooleanValue>() {
            BooleanValue::boxed(self.value && other.value)
        } else {
            CashError::InvalidOperation("and".to_owned(), "boolean ".to_owned() + typename).boxed()
        }
    }
    fn xor(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Ok(other) = value.downcast::<BooleanValue>() {
            BooleanValue::boxed((self.value || other.value) && (self.value != other.value))
        } else {
            CashError::InvalidOperation("and".to_owned(), "boolean ".to_owned() + typename).boxed()
        }
    }
    fn or(&self, value: Box<dyn Value>) -> ValueResult {
        let typename = value.get_type_name();
        if let Ok(other) = value.downcast::<BooleanValue>() {
            BooleanValue::boxed(self.value || other.value)
        } else {
            CashError::InvalidOperation("and".to_owned(), "boolean ".to_owned() + typename).boxed()
        }
    }
    fn r#async(&self) -> ValueResult {
        unimplemented!();
    }
}

impl std::fmt::Display for BooleanValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
