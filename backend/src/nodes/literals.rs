use crate::ast::Node;
use crate::context::Context;
use crate::value::Value;
use crate::values::{BooleanValue, IntegerValue};
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
pub struct BooleanLiteral {
    pub value: bool,
}

impl Node for BooleanLiteral {
    fn eval(
        &self,
        _ctx: Arc<RwLock<Context>>,
    ) -> Result<Box<dyn Value>, Box<dyn std::error::Error>> {
        BooleanValue::boxed(self.value)
    }
}
impl std::fmt::Display for BooleanLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BooleanLiteral '{}'", self.value)
    }
}

#[derive(Clone, Debug)]
pub struct IntegerLiteral {
    pub value: i64,
}

impl IntegerLiteral {
    pub fn parse_str(text: &str) -> Result<Box<dyn Node>, Box<dyn std::error::Error>> {
        Ok(Box::new(if text.starts_with("0x") {
            IntegerLiteral{
                value: i64::from_str_radix(&text[2..], 16)?
            }
        } else if text.starts_with("0o") {
            IntegerLiteral{
                value: i64::from_str_radix(&text[2..], 8)?
            }
        } else if text.starts_with("0b") {
            IntegerLiteral{
                value: i64::from_str_radix(&text[2..], 2)?
            }
        } else {
            IntegerLiteral{
                value: text.parse::<i64>()?
            }
        }))
    }
}

impl Node for IntegerLiteral {
    fn eval(
        &self,
        _ctx: Arc<RwLock<Context>>,
    ) -> Result<Box<dyn Value>, Box<dyn std::error::Error>> {
        IntegerValue::boxed(self.value)
    }
}
impl std::fmt::Display for IntegerLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IntegerLiteral '{}'", self.value)
    }
}
