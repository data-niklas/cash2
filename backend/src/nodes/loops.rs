use crate::ast::*;
use crate::context::Context;
use crate::error::CashError;
use crate::rules::Rule;
use crate::value::Value;
use crate::values::{BooleanValue, NoneValue};
use pest::iterators::Pairs;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct While {
    pub condition: Arc<dyn Node>,
    pub block: Arc<dyn Node>,
}

impl Node for While {
    fn eval(
        &self,
        ctx: Arc<RwLock<Context>>,
    ) -> Result<Box<dyn Value>, Box<dyn std::error::Error>> {
        let ctx = Context::from_parent(ctx);
        let mut lastvalue = NoneValue::boxed();
        loop {
            let val = self.condition.eval(ctx.clone())?;
            if let Some(val) = val.downcast_ref::<BooleanValue>() {
                if val.value {
                    lastvalue = Ok(self.block.eval(ctx.clone())?);
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
    pub fn parse(mut pairs: Pairs<Rule>) -> Result<Arc<dyn Node>, Box<dyn std::error::Error>> {
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
    fn eval(
        &self,
        ctx: Arc<RwLock<Context>>,
    ) -> Result<Box<dyn Value>, Box<dyn std::error::Error>> {
        let mut lastvalue = NoneValue::boxed();
        let values = self.expr.eval(ctx.clone())?.vec()?;
        for value in values {
            let ctx = Context::from_parent(ctx.clone());
            {
                ctx.write()
                    .expect("Could not write to context")
                    .set(&self.ident, value);
            }
            lastvalue = Ok(self.block.eval(ctx)?);
        }
        lastvalue
    }
}

impl For {
    pub fn parse(mut pairs: Pairs<Rule>) -> Result<Arc<dyn Node>, Box<dyn std::error::Error>> {
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
