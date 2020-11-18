use crate::ast::*;
use crate::context::Context;
use crate::error::CashError;
use crate::nodes::NoneLiteral;
use crate::rules::Rule;
use crate::value::Value;
use crate::values::{BreakValue, ContinueValue, ReturnValue};
use pest::iterators::Pairs;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub enum KeywordType {
    Return,
    Break,
    Continue,
}

#[derive(Debug)]
pub struct KeywordStatement {
    pub statement: Arc<dyn Node>,
    pub keyword: KeywordType,
}

impl Node for KeywordStatement {
    fn eval(
        &self,
        ctx: Arc<RwLock<Context>>,
    ) -> Result<Box<dyn Value>, Box<dyn std::error::Error>> {
        let val = self.statement.eval(ctx)?;
        match self.keyword {
            KeywordType::Return => ReturnValue::boxed(val),
            KeywordType::Break => BreakValue::boxed(val),
            KeywordType::Continue => ContinueValue::boxed(val),
        }
    }
}

impl KeywordStatement {
    pub fn parse_inner(
        mut inner: Pairs<Rule>,
    ) -> Result<Arc<dyn Node>, Box<dyn std::error::Error>> {
        let first = inner.next().expect("I need to find this");
        let keyword = match first.as_span().as_str() {
            "return" => KeywordType::Return,
            "break" => KeywordType::Break,
            "continue" => KeywordType::Continue,
            a => {
                return CashError::Bug(format!(
                    "Cannot have a keyword statement with a keyword {}",
                    a
                ))
                .boxed()
            }
        };
        let statement;
        if let Some(pair) = inner.next() {
            statement = make_ast(pair)?;
        } else {
            statement = Arc::new(NoneLiteral::default());
        }
        Ok(Arc::new(Self { statement, keyword }))
    }
}

impl std::fmt::Display for KeywordStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KeywordStatement{:?} '{}'", self.keyword, self.statement)
    }
}
