use crate::ast::*;
use crate::context::Context;
use crate::error::CashError;
use crate::rules::Rule;
use crate::value::Value;
use crate::values::*;
use pest::iterators::{Pair, Pairs};
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
        let text = text.replace("_", "");
        Ok(Box::new(if text.starts_with("0x") {
            IntegerLiteral {
                value: i64::from_str_radix(&text[2..], 16)?,
            }
        } else if text.starts_with("0o") {
            IntegerLiteral {
                value: i64::from_str_radix(&text[2..], 8)?,
            }
        } else if text.starts_with("0b") {
            IntegerLiteral {
                value: i64::from_str_radix(&text[2..], 2)?,
            }
        } else {
            IntegerLiteral {
                value: text.parse::<i64>()?,
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

#[derive(Clone, Debug)]
pub struct FloatLiteral {
    value: f64,
}

impl Node for FloatLiteral {
    fn eval(
        &self,
        _ctx: Arc<RwLock<Context>>,
    ) -> Result<Box<dyn Value>, Box<dyn std::error::Error>> {
        FloatValue::boxed(self.value)
    }
}
impl std::fmt::Display for FloatLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FloatLiteral '{}'", self.value)
    }
}

impl FloatLiteral {
    pub fn parse_str(text: &str) -> Result<Box<dyn Node>, Box<dyn std::error::Error>> {
        let text = text.replace("_", "");
        Ok(Box::new(FloatLiteral {
            value: text.parse::<f64>()?,
        }))
    }
}

#[derive(Debug)]
pub struct RangeLiteral {
    lower: Box<dyn Node>,
    upper: Box<dyn Node>,
}

impl Node for RangeLiteral {
    fn eval(
        &self,
        ctx: Arc<RwLock<Context>>,
    ) -> Result<Box<dyn Value>, Box<dyn std::error::Error>> {
        let lower = self.lower.eval(ctx.clone())?;
        let lower = lower.downcast_ref::<IntegerValue>();
        let upper = self.upper.eval(ctx)?;
        let upper = upper.downcast_ref::<IntegerValue>();
        if let (Some(lower), Some(upper)) = (lower, upper) {
            RangeValue::boxed(lower.value, upper.value)
        } else {
            CashError::InvalidType("unknown".to_owned(), "range".to_owned()).boxed()
        }
    }
}
impl std::fmt::Display for RangeLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RangeLiteral '{}'..'{}'", self.lower, self.upper)
    }
}

impl RangeLiteral {
    pub fn parse_inner(mut pair: Pairs<Rule>) -> Result<Box<dyn Node>, Box<dyn std::error::Error>> {
        let lower_node = pair.next().expect("Could not happen - grammar!");
        let upper_node = pair.next().expect("Could not happen - grammar!");
        let lower = make_ast(lower_node)?;
        let upper = make_ast(upper_node)?;

        Ok(Box::new(RangeLiteral { lower, upper }))
    }
}
