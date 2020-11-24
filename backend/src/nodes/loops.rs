use crate::ast::*;
use crate::context::Context;
use crate::context::LockableContext;
use crate::error::CashError;
use crate::rules::Rule;
use crate::value::ValueResult;
use crate::values::{BooleanValue, BreakValue, ContinueValue, NoneValue};
use pest::iterators::Pairs;
use std::sync::Arc;

#[derive(Debug)]
pub struct While {
    pub condition: Arc<dyn Node>,
    pub block: Arc<dyn Node>,
}

impl Node for While {
    fn eval(&self, ctx: LockableContext) -> ValueResult {
        let ctx = Context::from_parent(ctx);
        let mut lastvalue = NoneValue::boxed();
        loop {
            let val = self.condition.eval(ctx.clone())?;
            if let Some(val) = val.downcast_ref::<BooleanValue>() {
                if val.value {
                    let mut value = self.block.eval(ctx.clone())?;
                    if value.get_type_name() == "break" {
                        let value = value.downcast::<BreakValue>().unwrap();
                        return Ok(value.value);
                    } else if value.get_type_name() == "return" {
                        return Ok(value);
                    } else if value.get_type_name() == "continue" {
                        let val = value.downcast::<ContinueValue>().unwrap();
                        value = val.value;
                    }
                    lastvalue = Ok(value);
                } else {
                    return lastvalue;
                }
            } else {
                //TODO Print type
                return CashError::InvalidType(
                    val.get_type_name().to_owned(),
                    "condition".to_owned(),
                )
                .boxed();
            }
        }
    }
}

impl While {
    pub fn parse(
        mut pairs: Pairs<Rule>,
    ) -> Result<Arc<dyn Node>, Box<dyn std::error::Error + Sync + Send>> {
        let condition = make_ast(pairs.next().expect("due to grammar.pest"))?;
        let block = make_ast(pairs.next().expect("due to grammar.pest"))?;
        Ok(Arc::new(Self { condition, block }))
    }
}

impl std::fmt::Display for While {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = String::new();
        text.push_str(&format!("{} {}\n", self.condition, self.block));
        write!(f, "While '{}'", text)
    }
}

#[derive(Debug)]
pub struct For {
    pub ident: String,
    pub expr: Arc<dyn Node>,
    pub block: Arc<dyn Node>,
}

impl Node for For {
    fn eval(&self, ctx: LockableContext) -> ValueResult {
        let mut lastvalue = NoneValue::boxed();
        let values = self.expr.eval(ctx.clone())?.vec()?;
        for value in values {
            let ctx = Context::from_parent(ctx.clone());
            {
                ctx.write().set(&self.ident, value);
            }
            let mut value = self.block.eval(ctx)?;
            if value.get_type_name() == "break" {
                let value = value.downcast::<BreakValue>().unwrap();
                return Ok(value.value);
            } else if value.get_type_name() == "return" {
                return Ok(value);
            } else if value.get_type_name() == "continue" {
                let val = value.downcast::<ContinueValue>().unwrap();
                value = val.value;
            }
            lastvalue = Ok(value);
        }
        lastvalue
    }
}

impl For {
    pub fn parse(
        mut pairs: Pairs<Rule>,
    ) -> Result<Arc<dyn Node>, Box<dyn std::error::Error + Sync + Send>> {
        let ident = pairs
            .next()
            .expect("due to grammar.pest")
            .as_span()
            .as_str()
            .to_owned();
        let expr = make_ast(pairs.next().expect("due to grammar.pest"))?;
        let block = make_ast(pairs.next().expect("due to grammar.pest"))?;
        Ok(Arc::new(Self { ident, expr, block }))
    }
}

impl std::fmt::Display for For {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = String::new();
        text.push_str(&format!("{} {} {}\n", self.ident, self.expr, self.block));
        write!(f, "For '{}'", text)
    }
}
