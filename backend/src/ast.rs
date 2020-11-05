use std::sync::{Arc, RwLock};

use crate::context::Context;
use crate::nodes::*;
use crate::rules::Rule;
use crate::value::Value;
use downcast_rs::{impl_downcast, Downcast};
use pest::iterators::Pair;

pub trait Node: std::fmt::Display + std::fmt::Debug + Downcast {
    fn eval(&self, ctx: Arc<RwLock<Context>>)
        -> Result<Arc<dyn Value>, Box<dyn std::error::Error>>;
}
impl_downcast!(Node);

pub fn make_ast(root: Pair<Rule>) -> Result<Box<dyn Node>, Box<dyn std::error::Error>> {
    match root.as_rule() {
        Rule::Bool => Ok(Box::new(BooleanLiteral {
            value: root.as_span().as_str().parse::<bool>()?,
        })),
        Rule::Int => IntegerLiteral::parse_str(root.as_span().as_str()),
        Rule::Float => FloatLiteral::parse_str(root.as_span().as_str()),
        Rule::Range => RangeLiteral::parse_inner(root.into_inner()),
        Rule::String => StringLiteral::parse_inner(root.into_inner()),
        Rule::List => ListLiteral::parse_inner(root.into_inner()),
        Rule::Dict => DictLiteral::parse_inner(root.into_inner()),
        Rule::Expr => Expr::parse_inner(root.into_inner()),
        Rule::Assignment => Assignment::parse_inner(root.into_inner()),
        Rule::Ident => Ident::parse(root),
        _ => {
            unimplemented!();
        }
    }
}
