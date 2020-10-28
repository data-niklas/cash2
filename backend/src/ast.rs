use std::sync::{Arc, RwLock};

use crate::context::Context;
use crate::nodes::*;
use crate::rules::Rule;
use crate::value::Value;
use pest::iterators::{Pair, Pairs};

pub trait Node: ToString {
    fn eval(&self, ctx: Arc<RwLock<Context>>)
        -> Result<Box<dyn Value>, Box<dyn std::error::Error>>;
}

pub fn make_ast(root: Pair<Rule>) -> Result<Box<dyn Node>, Box<dyn std::error::Error>> {
    match root.as_rule() {
        Rule::Bool => Ok(Box::new(BooleanLiteral {
            value: root.as_span().as_str().parse::<bool>()?,
        })),
        _ => {
            unimplemented!();
        }
    }
}
