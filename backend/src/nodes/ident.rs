use crate::ast::*;
use crate::context::Context;
use crate::error::CashError;
use crate::rules::Rule;
use crate::value::Value;
use pest::iterators::{Pair, Pairs};
use std::iter::Peekable;
use std::slice::Iter;
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
pub struct Ident {
    pub ident: String,
}

impl Node for Ident {
    fn eval(
        &self,
        ctx: Arc<RwLock<Context>>,
    ) -> Result<Arc<dyn Value>, Box<dyn std::error::Error>> {
        if let Some(val) = ctx.read().expect("could not read value").get(&self.ident) {
            Ok(val)
        } else {
            CashError::VariableNotFound(self.ident.clone()).boxed()
        }
    }
}

impl Ident {
    pub fn parse(pair: Pair<Rule>) -> Result<Box<dyn Node>, Box<dyn std::error::Error>> {
        Ok(Box::new(Self {
            ident: pair.as_span().as_str().to_owned(),
        }))
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ident '{}'", self.ident)
    }
}
