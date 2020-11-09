//By default, eval async and wait for completion
//#BeDefault
use std::sync::{Arc, RwLock};

mod ast;
mod cashstd;
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
    pub fn new() -> Self {
        let mut runtime = Runtime {
            ctx: Arc::new(RwLock::new(Context::root())),
        };
        {
            let mut ctx = runtime.ctx.write().unwrap();
            ctx.set(
                "PI",
                Box::new(values::FloatValue {
                    value: std::f64::consts::PI,
                }),
            );
            ctx.set(
                "E",
                Box::new(values::FloatValue {
                    value: std::f64::consts::E,
                }),
            );
        }
        runtime
    }

    pub fn interpret(&mut self, text: String) {
        //A is used for debugging purposes
        {
            self.ctx
                .write()
                .unwrap()
                .set("a", Box::new(values::BooleanValue { value: false }));
        }
        let text = text.trim();
        let parse_result = rules::Language::parse(rules::Rule::Main, &text);
        if parse_result.is_err() {
            println!("Error occured while parsing input: {:?}", parse_result);
            return;
        }
        let parse_tree = parse_result.expect("Cannot happen: tested by if");
        let mut root_nodes = parse_tree.collect::<Vec<_>>();
        if root_nodes.len() != 1 {
            println!("Error occured while parsing input: Not exactly one root node");
            return;
        }
        let root_node = root_nodes.remove(0);
        let root_span = root_node.as_span();
        if root_span.start() != 0 || root_span.end() != text.len() {
            println!("Error occured while parsing input: Not all of the input was consumed");
            println!(
                "Consumed from {} to {}: {}",
                root_span.start(),
                root_span.end(),
                root_span.as_str()
            );
            return;
        }
        let tree_result = ast::make_ast(
            root_node
                .into_inner()
                .next()
                .expect("Main should have a Block"),
        );
        //println!("{:?}",tree_result);
        if tree_result.is_err() {
            println!("Error occured while parsing input: {:?}", tree_result);
            return;
        }
        let tree = tree_result.expect("Cannot happen: tested by if");
        println!(
            "{}",
            tree.eval(self.ctx.clone()).expect("Could not eval value")
        );
        println!("{:?}", self.ctx);
        // for (key, value) in std::env::vars() {
        //     println!("{}: {}", key, value);
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple() {
        let mut runtime = Runtime::new();
        //runtime.interpret("$ echo('asd')".to_owned());
        //runtime.interpret("\"~hello\\nworld\\x134\"".to_owned());
        //runtime.interpret("[false, false, false]".to_owned());
        //runtime.interpret("a = [1,[0,1],[0,1]];a[1..3][0] = 3".to_owned());
        //runtime.interpret("".to_owned());
        runtime.interpret(
            r#"#!/bin/env cash
        a = 4
        if a >= 5 | a < 3{
            7
        }
        elif a in 3..10{
            42
        }
        else{
            6
        }
        i = 0
        a=0
        for b in 1..101{
            a+=b
        }
        "#
            .to_owned(),
        );
    }
}
