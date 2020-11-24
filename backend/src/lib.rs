//By default, eval async and wait for completion
//#BeDefault
use parking_lot::{const_rwlock, RwLock};
use std::collections::HashMap;
use std::sync::Arc;

use glob::glob;

mod ast;
mod cashstd;
mod context;
mod error;
mod executor;
mod nodes;
mod rules;
mod value;
mod values;

use crate::context::LockableContext;
use context::Context;
use error::CashError;
use pest::Parser;
use value::{Value, ValueResult};

pub struct Runtime {
    ctx: LockableContext,
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl Runtime {
    pub fn new() -> Self {
        let runtime = Runtime {
            ctx: Arc::new(const_rwlock(Context::root())),
        };
        {
            let mut ctx = runtime.ctx.write();
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

        #[cfg(feature = "deadlock_detection")]
        {
            // only for #[cfg]
            use parking_lot::deadlock;
            use std::thread;
            use std::time::Duration;

            // Create a background thread which checks for deadlocks every 10s
            thread::spawn(move || loop {
                thread::sleep(Duration::from_secs(10));
                let deadlocks = deadlock::check_deadlock();
                if deadlocks.is_empty() {
                    continue;
                }

                println!("{} deadlocks detected", deadlocks.len());
                for (i, threads) in deadlocks.iter().enumerate() {
                    println!("Deadlock #{}", i);
                    for t in threads {
                        println!("Thread Id {:#?}", t.thread_id());
                        println!("{:#?}", t.backtrace());
                    }
                }
            });
        }

        runtime
    }

    fn preprocess(mut text: String) -> String {
        let mut replacements = HashMap::new();
        let cwd = std::env::current_dir().expect("Cannot find current working directory");
        for line in text.lines() {
            if line.starts_with("include! ") {
                let (_, path) = line.split_at(9);
                let mut all_contents = String::new();
                if replacements.contains_key(path) {
                    continue;
                }
                for entry in glob(path).expect("Failed to read glob pattern") {
                    match entry {
                        Ok(path) => {
                            let path =
                                std::fs::canonicalize(path).expect("Cannot canonicalize path");
                            let contents = std::fs::read_to_string(&path)
                                .expect("Errored while reading file!");
                            assert!(std::env::set_current_dir(
                                &path.parent().expect("Cannot find parent of file")
                            )
                            .is_ok());
                            let contents = Self::preprocess(contents);
                            assert!(std::env::set_current_dir(&cwd).is_ok());

                            all_contents.push_str("\n");
                            all_contents.push_str(&contents);
                        }
                        Err(e) => println!("{:?}", e),
                    }
                }
                replacements.insert(path.to_owned(), all_contents);
            }
        }

        for (key, value) in replacements {
            text = text.replace(&format!("include! {}", key), &value);
        }
        text
    }

    pub fn interpret(&mut self, text: String) -> ValueResult {
        //A is used for debugging purposes

        let text = text.trim().to_owned();

        let text = Self::preprocess(text);

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
        // println!("{:?}",tree_result);
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
