use crate::ast::Node;
use crate::context::Context;
use crate::executor::Executor;
use crate::value::{Value, ValueResult};
use std::sync::{Arc, Mutex, RwLock};

#[derive(Debug, Clone)]
pub struct FutureValue {
    pub id: usize,
    pub executor: Arc<Mutex<Executor>>,
}

impl FutureValue {
    pub fn boxed(node: Arc<dyn Node>, ctx: Arc<RwLock<Context>>) -> ValueResult {
        let executor = ctx
            .read()
            .expect("Could not read from RwLock")
            .get_executor();
        let id = executor
            .lock()
            .expect("Could not lock Executor")
            .register_job(node, ctx);
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
        self.executor
            .lock()
            .expect("Could not lock Executor")
            .get_result(self.id)
    }
}

impl std::fmt::Display for FutureValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FutureValue {}", self.id)
    }
}
