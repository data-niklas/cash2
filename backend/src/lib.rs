//By default, eval async and wait for completion
//#BeDefault
use std::sync::{Arc, RwLock};

mod ast;
mod context;
mod error;
mod nodes;
mod rules;
mod value;
mod values;

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
            rules::Language::parse(rules::Rule::Int, &text.trim()).expect("Could not create parse tree");
        let root_node = parse_tree.next().unwrap();
        let root_span = root_node.as_span();
        if root_span.start() != 0 || root_span.end() != text.len(){
            println!("ERROR OCCURED");
        }
        let tree = ast::make_ast(root_node).expect("Could not create ast");
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
        runtime.interpret("0x1100".to_owned());
    }
}
