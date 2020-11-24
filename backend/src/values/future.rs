use crate::ast::Node;
use crate::context::LockableContext;
use crate::executor::Executor;
use crate::value::{Value, ValueResult};
use parking_lot::Mutex;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct FutureValue {
    pub id: usize,
    pub executor: Arc<Mutex<Executor>>,
}

impl FutureValue {
    pub fn boxed(node: Arc<dyn Node>, ctx: LockableContext) -> ValueResult {
        let executor = ctx.read().get_executor();
        let id = executor.lock().register_job(node, ctx);
        Ok(Box::new(FutureValue { id, executor }))
    }
}

impl Value for FutureValue {
    fn clone(&self) -> Box<dyn Value> {
        Box::new(std::clone::Clone::clone(self))
    }
    fn get_type_name(&self) -> &'static str {
        "future"
    }
    fn r#await(self: Box<Self>) -> ValueResult {
        self.executor.lock().get_result(self.id)
    }
}

impl std::fmt::Display for FutureValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FutureValue {}", self.id)
    }
}
