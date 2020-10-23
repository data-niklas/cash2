//By default, eval async and wait for completion
//#BeDefault
use std::sync::{Arc, RwLock};

mod context;
mod value;
use context::Context;

pub struct Runtime {
    ctx: Arc<RwLock<Context>>,
}

impl Runtime {
    pub fn interpret(&mut self, text: String) {
        
    }
}
