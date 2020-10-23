use std::sync::{Arc, RwLock};

use crate::context::Context;
use crate::value::Value;

#[derive(Debug, Clone)]
pub trait Node {
    async fn eval(&self, ctx: Arc<RwLock<Context>>) -> Result<Box<dyn Value>, Box<dyn Error>>;
}