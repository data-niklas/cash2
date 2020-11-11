use std::sync::{Arc, RwLock};

use crate::context::Context;
use crate::nodes::*;
use crate::rules::Rule;
use crate::value::Value;
use downcast_rs::{impl_downcast, DowncastSync};
use pest::iterators::Pair;

pub trait Node: std::fmt::Display + std::fmt::Debug + DowncastSync {
    fn eval(&self, ctx: Arc<RwLock<Context>>)
        -> Result<Box<dyn Value>, Box<dyn std::error::Error>>;
}

impl_downcast!(sync Node);

pub fn make_ast(root: Pair<Rule>) -> Result<Arc<dyn Node>, Box<dyn std::error::Error>> {
    match root.as_rule() {
        Rule::Bool => Ok(Arc::new(BooleanLiteral {
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
        Rule::Block => Block::parse(root.into_inner()),
        Rule::Conditional => Conditional::parse(root.into_inner()),
        Rule::WhileLoop => While::parse(root.into_inner()),
        Rule::ForLoop => For::parse(root.into_inner()),
        Rule::Function => FunctionLiteral::parse_inner(root.into_inner()),
        _ => {
            unimplemented!();
        }
    }
}
