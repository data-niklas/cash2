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
use error::CashError;
use pest::Parser;
use value::Value;

pub struct Runtime {
    ctx: Arc<RwLock<Context>>,
}

impl Runtime {
    pub fn new() -> Self {
        let runtime = Runtime {
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

    pub fn interpret(
        &mut self,
        text: String,
    ) -> Result<Box<dyn Value>, Box<dyn std::error::Error>> {
        //A is used for debugging purposes

        let text = text.trim();
        let parse_result = rules::Language::parse(rules::Rule::Main, &text);
        if parse_result.is_err() {
            return CashError::ParseError(format!(
                "Error occured while parsing input: {:?}",
                parse_result
            ))
            .boxed();
        }
        let parse_tree = parse_result.expect("Cannot happen: tested by if");
        let mut root_nodes = parse_tree.collect::<Vec<_>>();
        if root_nodes.len() != 1 {
            return CashError::ParseError(
                "Error occured while parsing input: Not exactly one root node".to_owned(),
            )
            .boxed();
        }
        let root_node = root_nodes.remove(0);
        let root_span = root_node.as_span();
        if root_span.start() != 0 || root_span.end() != text.len() {
            return CashError::ParseError(
                format!("Error occured while parsing input: Not all of the input was consumed\nConsumed from {} to {}: {}",
                    root_span.start(),
                    root_span.end(),
                    root_span.as_str()
                )
            ).boxed();
        }
        let tree_result = ast::make_ast(
            root_node
                .into_inner()
                .next()
                .expect("Main should have a Block"),
        );
        //println!("{:?}",tree_result);
        if tree_result.is_err() {
            return CashError::ParseError(format!(
                "Error occured while parsing input: {:?}",
                tree_result
            ))
            .boxed();
        }
        let tree = tree_result.expect("Cannot happen: tested by if");
        tree.eval(self.ctx.clone())
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
        a = [1,2,3]
        b = map(a, print)
        print(b)
        print(type(false))
        print({"a": [1,2,3],"b":{"asd":false}})

        times = (n)->{
            (a)->{
                a*n
            }
        }
        three = times(3)
        three(9)

        "#
            .to_owned(),
        );
    }
}
