use crate::ast::*;
use crate::context::Context;
use crate::error::CashError;
use crate::rules::Rule;
use crate::value::Value;
use pest::iterators::Pairs;
use std::sync::{Arc, RwLock};

use super::{Expr, Infix, Postfix};

#[derive(Debug)]
pub struct Assignment {
    pub ident: String,
    pub indexes: Vec<Postfix>,
    pub infix: Option<Infix>,
    pub expr: Box<dyn Node>,
}

impl Node for Assignment {
    fn eval(
        &self,
        ctx: Arc<RwLock<Context>>,
    ) -> Result<Box<dyn Value>, Box<dyn std::error::Error>> {
        let mut result = self.expr.eval(ctx.clone())?;
        if let Some(infix) = &self.infix {
            if let Some(mut val) = ctx.read().expect("could not read value").get(&self.ident) {
                // handle indexed assignment
                for index in &self.indexes {
                    match index {
                        Postfix::Indexing(index_node) => {
                            // inside of indexing it can only read from the context
                            let index = index_node.eval(ctx.clone())?;
                            val = val.index(&index)?;
                        }
                        _ => unreachable!(),
                    }
                }

                // compute result
                result = Expr::compute_infix(val, &result, &infix)?;
            } else {
                return CashError::VariableNotFound(self.ident.clone()).boxed();
            }
        }
        if self.indexes.len() == 0 {
            // only place that will ever write to the context
            ctx.write()
                .expect("could not write value")
                .set(&self.ident, (*result).clone());
            Ok(result)
        } else {
            let mut indexes = Vec::with_capacity(self.indexes.len());
            for index in &self.indexes {
                match index {
                    Postfix::Indexing(index_node) => {
                        let index = index_node.eval(ctx.clone())?;
                        indexes.push(index);
                    }
                    _ => unreachable!(),
                }
            }

            let mut lock = ctx.write().expect("could not read value");
            if let Some(mut val) = lock.get(&self.ident) {
                val.indexed_set((*result).clone(), &indexes)?;
                lock.set(&self.ident, val);
                Ok(result)
            } else {
                return CashError::VariableNotFound(self.ident.clone()).boxed();
            }
        }
    }
}

impl Assignment {
    pub fn parse_inner(
        mut pairs: Pairs<Rule>,
    ) -> Result<Box<dyn Node>, Box<dyn std::error::Error>> {
        let ident = pairs
            .next()
            .expect("Did not find identifier node")
            .as_span()
            .as_str()
            .to_owned();
        let mut indexes = Vec::new();
        let mut infix = None;
        let mut expr = None;
        for pair in pairs {
            match pair.as_rule() {
                Rule::Indexing => indexes.push(Postfix::parse(pair)),
                Rule::Infix => infix = Some(Infix::parse(pair)),
                Rule::Expr => {
                    expr = Some(make_ast(pair)?);
                    break;
                }
                _ => {
                    return CashError::Bug("Unexpected node in assignment parsing".to_owned())
                        .boxed()
                }
            }
        }
        Ok(Box::new(Assignment {
            ident,
            indexes,
            infix,
            expr: expr.expect("Did not find expression to assign to"),
        }))
    }
}

impl std::fmt::Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("Assignment: ");
        s.push_str(&self.ident);
        for index in &self.indexes {
            s.push_str(&format!("{:?}", index));
        }
        s.push_str(" ");
        if let Some(infix) = &self.infix {
            s.push_str(&format!("{:?}", infix))
        }
        s.push_str(" ");
        s.push_str(&format!("{}", self.expr));
        write!(f, "{}", s)
    }
}
