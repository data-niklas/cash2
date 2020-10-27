//By default, eval async and wait for completion
//#BeDefault
use std::sync::{Arc, RwLock};

mod context;
mod value;
mod rules;
use context::Context;
use pest::Parser;

pub struct Runtime {
    ctx: Arc<RwLock<Context>>,
}

impl Runtime {

    pub fn new() -> Runtime{
        Runtime{
            ctx: Arc::new(RwLock::new(Context::root()))
        }
    }

    pub fn interpret(&mut self, text: String) {
        let parse_tree = rules::Language::parse(rules::Rule::Block, &text);
        println!("{:?}",parse_tree);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple() {
        let mut runtime = Runtime::new();
        runtime.interpret("$ echo('asd')".to_owned());
        assert_eq!(2 + 2, 4);
    }
}