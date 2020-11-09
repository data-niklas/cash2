use crate::ast::*;
use crate::context::Context;
use crate::error::CashError;
use crate::rules::Rule;
use crate::value::Value;
use pest::iterators::Pairs;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Box<dyn Node>>,
}

impl Node for Block {
    fn eval(
        &self,
        ctx: Arc<RwLock<Context>>,
    ) -> Result<Box<dyn Value>, Box<dyn std::error::Error>> {
        let mut lastvalue = None;
        let ctx = Context::from_parent(ctx);
        for statement in &self.statements {
            lastvalue = Some(statement.eval(ctx.clone())?);
        }
        Ok(lastvalue.unwrap())
    }
}

impl Block {
    pub fn parse(pairs: Pairs<Rule>) -> Result<Box<dyn Node>, Box<dyn std::error::Error>> {
        let mut statements = Vec::new();
        for pair in pairs {
            statements.push(make_ast(pair)?);
        }
        Ok(Box::new(Self { statements }))
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = String::new();
        for statement in &self.statements {
            text.push_str(&statement.to_string());
        }
        write!(f, "Block '{}'", text)
    }
}
