use crate::ast::*;
use crate::context::Context;
use crate::context::LockableContext;
use crate::error::CashError;
use crate::rules::Rule;
use crate::value::ValueResult;
use crate::values::{BooleanValue, NoneValue};
use pest::iterators::Pairs;
use std::sync::Arc;

type If = (Arc<dyn Node>, Arc<dyn Node>);

#[derive(Debug)]
pub struct Conditional {
    pub ifblocks: Vec<If>,
    pub elseblock: Option<Arc<dyn Node>>,
}

impl Node for Conditional {
    fn eval(&self, ctx: LockableContext) -> ValueResult {
        let ctx = Context::from_parent(ctx);
        for (expr, block) in &self.ifblocks {
            let val = expr.eval(ctx.clone())?;
            if let Some(val) = val.downcast_ref::<BooleanValue>() {
                if val.value {
                    return Ok(block.eval(ctx)?);
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
        if let Some(elseblock) = &self.elseblock {
            Ok(elseblock.eval(ctx)?)
        } else {
            NoneValue::boxed()
        }
    }
}

impl Conditional {
    pub fn parse(
        pairs: Pairs<Rule>,
    ) -> Result<Arc<dyn Node>, Box<dyn std::error::Error + Sync + Send>> {
        let mut ifblocks = Vec::new();
        let mut elseblock = None;
        for pair in pairs {
            match pair.as_rule() {
                Rule::If | Rule::Elif => {
                    let mut inner = pair.into_inner();
                    let expr = make_ast(inner.next().unwrap())?;
                    let block = make_ast(inner.next().unwrap())?;
                    ifblocks.push((expr, block));
                }
                Rule::Else => {
                    let block = make_ast(pair.into_inner().next().unwrap())?;
                    elseblock = Some(block);
                }
                _ => {
                    return CashError::Bug("Conditional may not contain this".to_owned()).boxed();
                }
            }
        }
        Ok(Arc::new(Self {
            ifblocks,
            elseblock,
        }))
    }
}

impl std::fmt::Display for Conditional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = String::new();
        for ifblock in &self.ifblocks {
            text.push_str(&format!("{} {}\n", ifblock.0, ifblock.1));
        }
        if let Some(elseblock) = &self.elseblock {
            text.push_str(&format!("{}", elseblock));
        }
        write!(f, "Conditional '{}'", text)
    }
}
