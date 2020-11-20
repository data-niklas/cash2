use crate::ast::*;
use crate::context::Context;
use crate::context::LockableContext;
use crate::error::CashError;
use crate::rules::Rule;
use crate::value::{Value, ValueResult};
use crate::values::{NoneValue, StringValue};
use pest::iterators::Pairs;
#[cfg(target_family = "unix")]
use std::os::unix::io::{AsRawFd, FromRawFd};
#[cfg(target_family = "windows")]
use std::os::windows::io::{FromRawHandle, IntoRawHandle};
use std::process::{Command, Stdio};
use std::sync::Arc;

#[derive(Debug)]
pub struct EnvCommand {
    pub name: String,
    pub args: Vec<Arc<dyn Node>>,
}

#[derive(Debug)]
pub struct Pipe {
    pub commands: Vec<EnvCommand>,
    pub capturing: bool,
}

impl Node for Pipe {
    fn eval(&self, ctx: LockableContext) -> ValueResult {
        // construct pipe
        let mut cmd = None;
        for (i, command) in self.commands.iter().enumerate() {
            let last = i == self.commands.len() - 1;
            let stdout = if !self.capturing && last {
                Stdio::inherit()
            } else {
                Stdio::piped()
            };
            let mut args = Vec::with_capacity(command.args.len());
            for arg in &command.args {
                args.push(format!("{}", arg.eval(ctx.clone())?));
            }
            if cmd.is_none() {
                cmd = Some(
                    Command::new(&command.name)
                        .args(&args)
                        .stdout(stdout)
                        .spawn()
                        .unwrap(),
                );
            } else {
                let prev_stdout = match cmd.expect("cannot happen, is checked").stdout {
                    Some(stdout) => stdout,
                    None => return CashError::Bug("No stdout for a command".to_owned()).boxed(),
                };

                #[cfg(target_family = "unix")]
                let stdin = unsafe { Stdio::from_raw_fd(prev_stdout.as_raw_fd()) };
                #[cfg(target_family = "windows")]
                let stdin = unsafe { Stdio::from_raw_handle(prev_stdout.into_raw_handle()) };

                cmd = Some(
                    Command::new(&command.name)
                        .args(&args)
                        .stdout(stdout)
                        .stdin(stdin)
                        .spawn()
                        .unwrap(),
                );
            }
        }

        // eval pipe
        if let Ok(res) = cmd.unwrap().wait_with_output() {
            if self.capturing {
                let mut text: String = String::from_utf8_lossy(&res.stdout).to_string();
                if text.ends_with("\n") {
                    text.remove(text.len() - 1);
                }
                StringValue::boxed(text)
            } else {
                NoneValue::boxed()
            }
        } else {
            CashError::Bug("".to_owned()).boxed()
        }
    }
}
impl std::fmt::Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pipe")
    }
}

impl Pipe {
    pub fn parse_inner(
        pairs: Pairs<Rule>,
        capturing: bool,
    ) -> Result<Arc<dyn Node>, Box<dyn std::error::Error + Sync + Send>> {
        let mut commands = Vec::new();

        for mut inner in pairs.map(|x| x.into_inner()) {
            let ident = inner.next().unwrap();
            let call = inner.next().unwrap();
            let name = ident.as_span().as_str().to_owned();

            let mut args = Vec::new();
            for arg in call.into_inner() {
                args.push(make_ast(arg)?);
            }

            commands.push(EnvCommand { name, args })
        }

        Ok(Arc::new(Self {
            commands,
            capturing,
        }))
    }
}
