use crate::ast::Node;
use crate::context::Context;
use crate::error::CashError;
use crate::value::ValueResult;
use num_cpus;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use threadpool::ThreadPool;

#[derive(Debug)]
pub struct Executor {
    threadpool: Mutex<ThreadPool>,
    results: Arc<RwLock<HashMap<usize, ValueResult>>>,
    counter: usize,
}

impl Default for Executor {
    fn default() -> Executor {
        let threadpool = Mutex::new(ThreadPool::new(num_cpus::get()));
        let results = Arc::new(RwLock::new(HashMap::new()));
        let counter = 0;
        Executor {
            threadpool,
            results,
            counter,
        }
    }
}

impl Executor {
    pub fn register_job(&mut self, node: Arc<dyn Node>, ctx: Arc<RwLock<Context>>) -> usize {
        let id = self.counter;
        let results = self.results.clone();
        self.threadpool
            .lock()
            .expect("Could not lock mutex")
            .execute(move || {
                let result = node.eval(ctx);
                results
                    .write()
                    .expect("Could not write to HashMap")
                    .insert(id, result);
            });
        self.counter = self.counter.wrapping_add(1);
        id
    }

    pub fn get_result(&self, id: usize) -> ValueResult {
        while !self
            .results
            .read()
            .expect("Could not read from HashMap")
            .contains_key(&id)
        {
            thread::sleep(std::time::Duration::from_millis(1));
        }
        self.results
            .write()
            .expect("Could not write to HashMap")
            .remove(&id)
            .expect("Can not happen, checked before")
    }
}
