//By default, eval async and wait for completion
//#BeDefault
use std::sync::{Arc, RwLock};

mod ast;
mod context;
mod error;
mod nodes;
mod rules;
mod value;

use context::Context;
use pest::Parser;

pub struct Runtime {
    ctx: Arc<RwLock<Context>>,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            ctx: Arc::new(RwLock::new(Context::root())),
        }
    }

    pub fn interpret(&mut self, text: String) {
        let mut parse_tree =
            rules::Language::parse(rules::Rule::Bool, &text).expect("Could not create parse tree");
        let tree = ast::make_ast(parse_tree.next().unwrap()).expect("Could not create ast");
        println!(
            "{}",
            tree.eval(self.ctx.clone()).expect("Could not eval value")
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple() {
        let mut runtime = Runtime::new();
        //runtime.interpret("$ echo('asd')".to_owned());
        runtime.interpret("false".to_owned());
        assert_eq!(2 + 2, 4);
    }
}
