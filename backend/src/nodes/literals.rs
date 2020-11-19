use crate::ast::*;
use crate::context::Context;
use crate::error::CashError;
use crate::rules::Rule;
use crate::value::{Value, ValueResult};
use crate::values::*;
use pest::iterators::Pairs;
use std::sync::{Arc, RwLock};

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct BooleanLiteral {
    pub value: bool,
}

impl Node for BooleanLiteral {
    fn eval(&self, _ctx: Arc<RwLock<Context>>) -> ValueResult {
        BooleanValue::boxed(self.value)
    }
}
impl std::fmt::Display for BooleanLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BooleanLiteral '{}'", self.value)
    }
}

#[derive(Clone, Debug)]
pub struct IntegerLiteral {
    pub value: i64,
}

impl IntegerLiteral {
    pub fn parse_str(
        text: &str,
    ) -> Result<Arc<dyn Node>, Box<dyn std::error::Error + Sync + Send>> {
        let text = text.replace("_", "");
        Ok(Arc::new(if text.starts_with("0x") {
            IntegerLiteral {
                value: i64::from_str_radix(&text[2..], 16)?,
            }
        } else if text.starts_with("0o") {
            IntegerLiteral {
                value: i64::from_str_radix(&text[2..], 8)?,
            }
        } else if text.starts_with("0b") {
            IntegerLiteral {
                value: i64::from_str_radix(&text[2..], 2)?,
            }
        } else {
            IntegerLiteral {
                value: text.parse::<i64>()?,
            }
        }))
    }
}

impl Node for IntegerLiteral {
    fn eval(&self, _ctx: Arc<RwLock<Context>>) -> ValueResult {
        IntegerValue::boxed(self.value)
    }
}
impl std::fmt::Display for IntegerLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IntegerLiteral '{}'", self.value)
    }
}

#[derive(Clone, Debug)]
pub struct FloatLiteral {
    value: f64,
}

impl Node for FloatLiteral {
    fn eval(&self, _ctx: Arc<RwLock<Context>>) -> ValueResult {
        FloatValue::boxed(self.value)
    }
}
impl std::fmt::Display for FloatLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FloatLiteral '{}'", self.value)
    }
}

impl FloatLiteral {
    pub fn parse_str(
        text: &str,
    ) -> Result<Arc<dyn Node>, Box<dyn std::error::Error + Sync + Send>> {
        let text = text.replace("_", "");
        Ok(Arc::new(FloatLiteral {
            value: text.parse::<f64>()?,
        }))
    }
}

#[derive(Debug)]
pub struct RangeLiteral {
    lower: Arc<dyn Node>,
    upper: Arc<dyn Node>,
}

impl Node for RangeLiteral {
    fn eval(&self, ctx: Arc<RwLock<Context>>) -> ValueResult {
        let lower = self.lower.eval(ctx.clone())?;
        let lower = lower.downcast_ref::<IntegerValue>();
        let upper = self.upper.eval(ctx)?;
        let upper = upper.downcast_ref::<IntegerValue>();
        if let (Some(lower), Some(upper)) = (lower, upper) {
            RangeValue::boxed(lower.value, upper.value)
        } else {
            CashError::InvalidType("unknown".to_owned(), "range".to_owned()).boxed()
        }
    }
}
impl std::fmt::Display for RangeLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RangeLiteral '{}'..'{}'", self.lower, self.upper)
    }
}

impl RangeLiteral {
    pub fn parse_inner(
        mut pair: Pairs<Rule>,
    ) -> Result<Arc<dyn Node>, Box<dyn std::error::Error + Sync + Send>> {
        let lower_node = pair.next().expect("Could not happen - grammar!");
        let upper_node = pair.next().expect("Could not happen - grammar!");
        let lower = make_ast(lower_node)?;
        let upper = make_ast(upper_node)?;

        Ok(Arc::new(RangeLiteral { lower, upper }))
    }
}

#[derive(Debug)]
pub struct ListLiteral {
    vals: Vec<Arc<dyn Node>>,
}

impl Node for ListLiteral {
    fn eval(&self, ctx: Arc<RwLock<Context>>) -> ValueResult {
        let mut v: Vec<Box<dyn Value>> = Vec::with_capacity(self.vals.len());
        for val in &self.vals {
            v.push((*val).eval(ctx.clone())?);
        }
        ListValue::boxed(v)
    }
}

impl std::fmt::Display for ListLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("ListLiteral [");
        for value in &self.vals {
            s.push_str(&format!("{} ", value));
        }
        s.push_str("]");
        write!(f, "{}", s)
    }
}

impl ListLiteral {
    pub fn parse_inner(
        pair: Pairs<Rule>,
    ) -> Result<Arc<dyn Node>, Box<dyn std::error::Error + Sync + Send>> {
        let mut vals = Vec::new();
        for node in pair {
            vals.push(make_ast(node)?);
        }
        Ok(Arc::new(ListLiteral { vals }))
    }
}

#[derive(Debug)]
pub struct DictLiteral {
    vals: Vec<(Arc<dyn Node>, Arc<dyn Node>)>,
}

impl Node for DictLiteral {
    fn eval(&self, ctx: Arc<RwLock<Context>>) -> ValueResult {
        let mut v: HashMap<String, Box<dyn Value>> = HashMap::with_capacity(self.vals.len());
        for (key, val) in &self.vals {
            v.insert(
                (*key).eval(ctx.clone())?.to_string(),
                (*val).eval(ctx.clone())?,
            );
        }
        DictValue::boxed(v)
    }
}

impl std::fmt::Display for DictLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("DictLiteral {");
        for (key, value) in &self.vals {
            s.push_str(&format!("{}: {} ", key, value));
        }
        s.push_str("}");
        write!(f, "{}", s)
    }
}

impl DictLiteral {
    pub fn parse_inner(
        pair: Pairs<Rule>,
    ) -> Result<Arc<dyn Node>, Box<dyn std::error::Error + Sync + Send>> {
        let mut vals = Vec::new();
        for node in pair {
            let mut inner: Vec<_> = node.into_inner().collect();
            if inner.len() != 2 {
                return CashError::Bug("Dict value-pair must have length 2".to_owned()).boxed();
            } else {
                vals.push((make_ast(inner.remove(0))?, make_ast(inner.remove(0))?));
            }
        }
        Ok(Arc::new(DictLiteral { vals }))
    }
}

#[derive(Debug)]
pub struct StringLiteral {
    pub strings: Vec<String>,
    pub interpolations: Vec<Arc<dyn Node>>,
}

impl Node for StringLiteral {
    fn eval(&self, ctx: Arc<RwLock<Context>>) -> ValueResult {
        let mut text = self.strings[0].to_owned();
        for i in 0..self.interpolations.len() {
            let value = self.interpolations[i].eval(ctx.clone())?;
            text += &value.to_string();
            text += &self.strings[i + 1];
        }
        StringValue::boxed(text)
    }
}
impl std::fmt::Display for StringLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StringLiteral")
    }
}

impl StringLiteral {
    pub fn parse_inner(
        pair: Pairs<Rule>,
    ) -> Result<Arc<dyn Node>, Box<dyn std::error::Error + Sync + Send>> {
        let mut strings = Vec::new();
        let mut interpolations = Vec::new();
        let mut value = String::new();
        for node in pair {
            match node.as_rule() {
                Rule::Home => {
                    if let Some(path) = dirs::home_dir() {
                        value += path
                            .to_str()
                            .expect("Home directory contains invalid unicode symbols");
                    } else {
                        return CashError::Bug("Could not get home directory".to_owned()).boxed();
                    }
                }
                Rule::SingleQuoteText | Rule::DoubleQuoteText => {
                    value += node.as_span().as_str();
                }
                Rule::Escape => match &node.as_span().as_str()[1..] {
                    "n" => {
                        value += "\n";
                    }
                    "r" => {
                        value += "\r";
                    }
                    "t" => {
                        value += "\t";
                    }
                    "b" => {
                        value += "\x7f";
                    }
                    any => {
                        if any.starts_with('x') {
                            if any.len() > 1 {
                                if let Ok(number) = u32::from_str_radix(&any[1..], 16) {
                                    value += &std::char::from_u32(number)
                                        .expect("u32 should convert to an unicode char")
                                        .to_string();
                                } else {
                                    return CashError::InvalidInput(
                                        "'".to_owned() + any + "' for escape sequence",
                                    )
                                    .boxed();
                                }
                            }
                        } else {
                            value += any;
                        }
                    }
                },
                Rule::Interpolation => {
                    let content = node
                        .into_inner()
                        .next()
                        .expect("Could not happen - grammar!");
                    let result = make_ast(content)?;
                    strings.push(value);
                    value = String::new();
                    interpolations.push(result);
                }
                _ => {
                    return CashError::Bug(format!(
                        "StringLiteral should not contain node {:?}",
                        node.as_rule()
                    ))
                    .boxed();
                }
            }
        }
        strings.push(value);
        Ok(Arc::new(StringLiteral {
            strings,
            interpolations,
        }))
    }
}

#[derive(Debug)]
pub struct FunctionLiteral {
    pub node: Arc<dyn Node>,
    pub params: Vec<(String, Option<Arc<dyn Node>>)>,
}

impl Node for FunctionLiteral {
    fn eval(&self, ctx: Arc<RwLock<Context>>) -> ValueResult {
        let mut params = Vec::with_capacity(self.params.len());
        for (name, value) in &self.params {
            let mut optional = None;
            if let Some(value) = value {
                optional = Some(value.eval(ctx.clone())?);
            }
            params.push((name.clone(), optional));
        }
        FunctionValue::boxed(self.node.clone(), ctx, params)
    }
}
impl std::fmt::Display for FunctionLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FunctionLiteral")
    }
}

impl FunctionLiteral {
    pub fn parse_inner(
        mut pairs: Pairs<Rule>,
    ) -> Result<Arc<dyn Node>, Box<dyn std::error::Error + Sync + Send>> {
        let first = pairs.next().unwrap();
        let mut params = Vec::new();
        let block;
        if first.as_rule() == Rule::FunctionParams {
            block = make_ast(pairs.next().unwrap())?;
            for param in first.into_inner() {
                let mut inner = param.into_inner();
                let name = inner.next().unwrap().as_span().as_str().to_owned();
                let default_value = match inner.next() {
                    None => None,
                    Some(value) => Some(match value.as_rule() {
                        Rule::Literal => make_ast(
                            value
                                .into_inner()
                                .next()
                                .expect("Literal should contain node"),
                        )?,
                        Rule::Ident => make_ast(value)?,
                        _ => {
                            return CashError::Bug(
                                "Function marker should not contain other rule".to_owned(),
                            )
                            .boxed();
                        }
                    }),
                };
                params.push((name, default_value));
            }
        } else {
            block = make_ast(first)?;
        }

        Ok(Arc::new(FunctionLiteral {
            node: block,
            params,
        }))
    }
}

#[derive(Debug, Default)]
pub struct NoneLiteral;

impl Node for NoneLiteral {
    fn eval(&self, _ctx: Arc<RwLock<Context>>) -> ValueResult {
        NoneValue::boxed()
    }
}
impl std::fmt::Display for NoneLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NoneLiteral")
    }
}
