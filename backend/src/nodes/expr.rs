use crate::ast::*;
use crate::context::Context;
use crate::error::CashError;
use crate::rules::Rule;
use crate::value::Value;
use pest::iterators::{Pair, Pairs};
use std::iter::Peekable;
use std::slice::Iter;
use std::sync::{Arc, RwLock};

type Primary = (Vec<Prefix>, Arc<dyn Node>, Vec<Postfix>);

#[derive(Debug)]
pub struct Expr {
    pub values: Vec<Primary>,
    pub infixes: Vec<Infix>,
}

impl Node for Expr {
    fn eval(
        &self,
        ctx: Arc<RwLock<Context>>,
    ) -> Result<Box<dyn Value>, Box<dyn std::error::Error>> {
        let mut values = Vec::new();
        for value in &self.values {
            values.push(self.eval_primary(&value, ctx.clone())?);
        }
        Self::climb_ops(
            &mut values.into_iter(),
            &mut self.infixes.iter().peekable(),
            0,
        )
    }
}

impl Expr {
    fn climb_ops(
        values: &mut impl std::iter::Iterator<Item = Box<dyn Value>>,
        infixes: &mut Peekable<Iter<Infix>>,
        min_precedence: usize,
    ) -> Result<Box<dyn Value>, Box<dyn std::error::Error>> {
        let mut result = values.next().expect("A value should exist");
        while let Some(next) = infixes.peek() {
            let mut prec = next.precedence();
            if prec < min_precedence {
                break;
            }
            let next = infixes.next().expect("Peeked, so should have next");
            if next.left_to_right() {
                prec += 1;
            }
            let rhs = Self::climb_ops(values, infixes, prec)?;
            result = Self::compute_infix(result, &rhs, next)?;
        }
        Ok(result)
    }

    pub fn compute_infix(
        lhs: Box<dyn Value>,
        rhs: &Box<dyn Value>,
        infix: &Infix,
    ) -> Result<Box<dyn Value>, Box<dyn std::error::Error>> {
        match infix {
            Infix::Exponentiation => lhs.power(rhs),
            Infix::Multiply => lhs.multiply(rhs),
            Infix::Divide => lhs.division(rhs),
            Infix::Modulo => lhs.modulo(rhs),
            Infix::Add => lhs.add(rhs),
            Infix::Subtract => lhs.subtract(rhs),
            Infix::BitShiftLeft => lhs.bit_shift_l(rhs),
            Infix::BitShiftRight => lhs.bit_shift_r(rhs),
            Infix::In => rhs.contains(&lhs),
            Infix::Lt => lhs.lt(rhs),
            Infix::Gt => lhs.gt(rhs),
            Infix::Lte => lhs.lte(rhs),
            Infix::Gte => lhs.gte(rhs),
            Infix::Ne => lhs.ne(rhs),
            Infix::Equality => lhs.eq(rhs),
            Infix::And => lhs.and(rhs),
            Infix::Xor => lhs.xor(rhs),
            Infix::Or => lhs.or(rhs),
        }
    }

    fn eval_primary(
        &self,
        primary: &(Vec<Prefix>, Arc<dyn Node>, Vec<Postfix>),
        ctx: Arc<RwLock<Context>>,
    ) -> Result<Box<dyn Value>, Box<dyn std::error::Error>> {
        let (prefixes, value, postfixes) = primary;
        let mut value = value.eval(ctx.clone())?;
        for postfix in postfixes {
            match postfix {
                Postfix::FunctionCall(nodes) => {
                    let mut args: Vec<Box<dyn Value>> = Vec::new();
                    for node in nodes {
                        args.push(node.eval(ctx.clone())?);
                    }
                    value = value.call(args, ctx.clone())?;
                }
                Postfix::Indexing(node) => {
                    let arg: Box<dyn Value> = node.eval(ctx.clone())?;
                    value = value.index(&arg)?;
                }
            }
        }
        for prefix in prefixes.iter().rev() {
            match prefix {
                Prefix::UnaryPlus => {
                    value = value.uplus()?;
                }
                Prefix::UnaryMinus => {
                    value = value.uminus()?;
                }
                Prefix::Await => {
                    value = value.r#await()?;
                }
                Prefix::Async => {
                    value = value.r#async()?;
                }
                Prefix::Not => {
                    value = value.not()?;
                }
            }
        }

        Ok(value)
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("Expr: ");
        for (i, (prefixes, primary, postfixes)) in self.values.iter().enumerate() {
            for prefix in prefixes {
                s.push_str(&format!("{:?} ", prefix));
            }
            s.push_str(&format!("{} ", primary));
            for postfix in postfixes {
                s.push_str(&format!("{:?} ", postfix));
            }
            if i < self.infixes.len() {
                s.push_str(&format!("{:?} ", self.infixes[i]));
            }
        }
        write!(f, "{}", s)
    }
}

impl Expr {
    pub fn parse_inner(pairs: Pairs<Rule>) -> Result<Arc<dyn Node>, Box<dyn std::error::Error>> {
        let mut values = Vec::new();
        let mut infixes = Vec::new();
        let mut prefixes = Vec::new();
        let mut postfixes = Vec::new();
        let mut primary = None;
        for pair in pairs {
            match pair.as_rule() {
                Rule::Prefix => {
                    prefixes.push(Prefix::parse(pair));
                }
                Rule::Expr | Rule::Capture | Rule::Block => {
                    primary = Some(make_ast(pair)?);
                }
                Rule::Literal => {
                    primary = Some(make_ast(
                        pair.into_inner()
                            .next()
                            .expect("Literal should contain node"),
                    )?);
                }
                Rule::Postfix => {
                    postfixes.push(Postfix::parse(
                        pair.into_inner()
                            .next()
                            .expect("Did not find node in postfix?"),
                    ));
                }
                Rule::Infix => {
                    infixes.push(Infix::parse(pair));
                    values.push((prefixes, primary.expect("Primary should exist"), postfixes));
                    prefixes = Vec::new();
                    postfixes = Vec::new();
                    primary = None;
                }
                Rule::Ident => {
                    primary = Some(make_ast(pair)?);
                }
                _ => {
                    return CashError::Bug("Expr should not contains other rule".to_owned()).boxed()
                }
            }
        }
        values.push((prefixes, primary.expect("Primary should exist"), postfixes));

        Ok(Arc::new(Expr { values, infixes }))
    }
}

#[derive(Debug)]
pub enum Prefix {
    UnaryPlus,
    UnaryMinus,
    Await,
    Async,
    Not,
}

impl Prefix {
    pub fn parse(pair: Pair<Rule>) -> Self {
        match pair.as_span().as_str() {
            "!" => Self::Not,
            "+" => Self::UnaryPlus,
            "-" => Self::UnaryMinus,
            "await" => Self::Await,
            "async" => Self::Async,
            _ => panic!("Unrecognized prefix operator"),
        }
    }
}

#[derive(Debug)]
pub enum Infix {
    Exponentiation,
    Multiply,
    Divide,
    Modulo,
    Add,
    Subtract,
    BitShiftLeft,
    BitShiftRight,
    In,
    Lt,
    Gt,
    Lte,
    Gte,
    Ne,
    Equality,
    And,
    Xor,
    Or,
}

impl Infix {
    pub fn parse(pair: Pair<Rule>) -> Self {
        match pair.as_span().as_str() {
            "**" => Self::Exponentiation,
            "*" => Self::Multiply,
            "/" => Self::Divide,
            "%" => Self::Modulo,
            "+" => Self::Add,
            "-" => Self::Subtract,
            "<<" => Self::BitShiftLeft,
            ">>" => Self::BitShiftRight,
            "in" => Self::In,
            "<" => Self::Lt,
            ">" => Self::Gt,
            "<=" => Self::Lte,
            ">=" => Self::Gte,
            "!=" => Self::Ne,
            "==" => Self::Equality,
            "&" => Self::And,
            "^" => Self::Xor,
            "|" => Self::Or,
            _ => panic!("Unrecognized infix operator"),
        }
    }

    pub fn precedence(&self) -> usize {
        match self {
            Self::Exponentiation => 9,
            Self::Multiply => 8,
            Self::Divide => 8,
            Self::Modulo => 8,
            Self::Add => 7,
            Self::Subtract => 7,
            Self::BitShiftLeft => 6,
            Self::BitShiftRight => 6,
            Self::In => 5,
            Self::Lt => 5,
            Self::Gt => 5,
            Self::Lte => 5,
            Self::Gte => 5,
            Self::Ne => 4,
            Self::Equality => 4,
            Self::And => 3,
            Self::Xor => 2,
            Self::Or => 1,
        }
    }
    pub fn left_to_right(&self) -> bool {
        match self {
            Self::Exponentiation => false,
            Self::Multiply => true,
            Self::Divide => true,
            Self::Modulo => true,
            Self::Add => true,
            Self::Subtract => true,
            Self::BitShiftLeft => true,
            Self::BitShiftRight => true,
            Self::In => true,
            Self::Lt => true,
            Self::Gt => true,
            Self::Lte => true,
            Self::Gte => true,
            Self::Ne => true,
            Self::Equality => true,
            Self::And => true,
            Self::Xor => true,
            Self::Or => true,
        }
    }
}

#[derive(Debug)]
pub enum Postfix {
    FunctionCall(Vec<Arc<dyn Node>>),
    Indexing(Arc<dyn Node>),
}

impl Postfix {
    pub fn parse(inner: Pair<Rule>) -> Self {
        match inner.as_rule() {
            Rule::FunctionCall => {
                let mut nodes = Vec::new();
                for node in inner.into_inner() {
                    nodes.push(make_ast(node).expect("Could not create node from function call"));
                }
                Self::FunctionCall(nodes)
            }
            Rule::Indexing => {
                let node = make_ast(
                    inner
                        .into_inner()
                        .next()
                        .expect("Indexing should contain a node"),
                )
                .expect("Could not create node from indexing");
                Self::Indexing(node)
            }
            _ => panic!("Should not contain other rule"),
        }
    }
}
