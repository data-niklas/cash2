use crate::ast::*;
use crate::context::Context;
use crate::rules::Rule;
use crate::value::Value;
use pest::iterators::Pairs;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Arc<dyn Node>>,
    pub is_root: bool,
}

impl Node for Block {
    fn eval(
        &self,
        ctx: Arc<RwLock<Context>>,
    ) -> Result<Box<dyn Value>, Box<dyn std::error::Error>> {
        let mut lastvalue = None;
        let ctxt;
        if !self.is_root {
            ctxt = Context::from_parent(ctx);
        } else {
            ctxt = ctx;
        }
        for statement in &self.statements {
            lastvalue = Some(statement.eval(ctxt.clone())?);
        }
        Ok(lastvalue.unwrap())
    }
}

impl Block {
    pub fn parse(
        pairs: Pairs<Rule>,
        is_root: bool,
    ) -> Result<Arc<dyn Node>, Box<dyn std::error::Error>> {
        let mut statements = Vec::new();
        for pair in pairs {
            statements.push(make_ast(pair)?);
        }
        Ok(Arc::new(Self {
            statements,
            is_root,
        }))
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
