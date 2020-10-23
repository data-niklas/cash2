use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::value::Value;

pub struct Context {
    parent: Option<Arc<RwLock<Context>>>,
    vars: HashMap<String, Arc<dyn Value>>,
}

impl Context {
    pub fn root() -> Context {
        Context {
            parent: None,
            vars: HashMap::new(),
        }
    }
    pub fn from_parent(parent: Arc<RwLock<Context>>) -> Context {
        Context {
            parent: Some(parent),
            vars: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<Arc<dyn Value>> {
        if self.vars.contains_key(key) {
            return Some(self.vars.get(key).unwrap().clone());
        }
        if let Some(parent) = &self.parent {
            return parent
                .read()
                .expect("COULD NOT READ VAR OUT OF HASHMAP!!!")
                .get(key);
        }
        None
    }

    pub fn set(&mut self, key: &str, value: Arc<dyn Value>) {
        if let Some(parent) = &self.parent {
            if parent
                .read()
                .expect("COULD NOT READ VAR OUT OF HASHMAP!!!")
                .get(key)
                .is_some()
            {
                parent
                    .write()
                    .expect("WEIRD ERROR MESSAGE TO BE CHANGED")
                    .set(key, value);
                return;
            }
        }
        self.vars.insert(key.to_owned(), value);
    }

    pub fn set_self(&mut self, key: &str, value: Arc<dyn Value>) {
        self.vars.insert(key.to_owned(), value);
    }
}
