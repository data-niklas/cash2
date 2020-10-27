use pest_derive::*;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Language;