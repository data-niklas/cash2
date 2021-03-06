use crate::ast::*;
use crate::context::LockableContext;
use crate::error::CashError;
use crate::rules::Rule;
use crate::value::ValueResult;
use pest::iterators::Pair;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Ident {
    pub ident: String,
}

impl Node for Ident {
    fn eval(&self, ctx: LockableContext) -> ValueResult {
        if let Some(val) = ctx.read().get(&self.ident) {
            Ok(val)
        } else {
            CashError::VariableNotFound(self.ident.clone()).boxed()
        }
    }
}

impl Ident {
    pub fn parse(
        pair: Pair<Rule>,
    ) -> Result<Arc<dyn Node>, Box<dyn std::error::Error + Sync + Send>> {
        Ok(Arc::new(Self {
            ident: pair.as_span().as_str().to_owned(),
        }))
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ident '{}'", self.ident)
    }
}
