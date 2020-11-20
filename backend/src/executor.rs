use crate::ast::Node;
use crate::context::{Context, LockableContext};
use crate::error::CashError;
use crate::value::ValueResult;
use num_cpus;
use parking_lot::{const_mutex, const_rwlock, Mutex, RwLock};
use std::collections::HashMap;
use std::sync::Arc;
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
        let threadpool = const_mutex(ThreadPool::new(num_cpus::get()));
        let results = Arc::new(const_rwlock(HashMap::new()));
        let counter = 0;
        Executor {
            threadpool,
            results,
            counter,
        }
    }
}

impl Executor {
    pub fn register_job(&mut self, node: Arc<dyn Node>, ctx: LockableContext) -> usize {
        let id = self.counter;
        let results = self.results.clone();
        self.threadpool.lock().execute(move || {
            let result = node.eval(ctx);
            results.write().insert(id, result);
        });
        self.counter = self.counter.wrapping_add(1);
        id
    }

    pub fn get_result(&self, id: usize) -> ValueResult {
        loop {
            {
                if self.results.read().contains_key(&id) {
                    break;
                }
            }
            thread::sleep(std::time::Duration::from_millis(1));
        }
        self.results
            .write()
            .remove(&id)
            .expect("Can not happen, checked before")
    }
}
