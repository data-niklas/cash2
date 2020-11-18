use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::cashstd;
use crate::value::Value;
use crate::values::StringValue;

#[derive(Debug)]
pub struct Context {
    parent: Option<Arc<RwLock<Context>>>,
    vars: HashMap<String, Box<dyn Value>>,
}

impl Context {
    pub fn root() -> Context {
        Context {
            parent: None,
            vars: HashMap::new(),
        }
    }
    pub fn from_parent(parent: Arc<RwLock<Context>>) -> Arc<RwLock<Context>> {
        Arc::new(RwLock::new(Context {
            parent: Some(parent),
            vars: HashMap::new(),
        }))
    }

    pub fn get(&self, key: &str) -> Option<Box<dyn Value>> {
        if key.starts_with('$') {
            if let Ok(val) = std::env::var(&key[1..]) {
                return Some(Box::new(StringValue { value: val }));
            } else {
                return None;
            }
        }
        if self.vars.contains_key(key) {
            return Some((*self.vars.get(key).unwrap()).clone());
        }
        if let Some(parent) = &self.parent {
            if let Some(val) = parent
                .read()
                .expect("COULD NOT READ VAR OUT OF HASHMAP!!!")
                .get(key)
            {
                return Some(val);
            }
        }
        // None
        cashstd::get_stdlib_function(key)
    }

    pub fn set(&mut self, key: &str, value: Box<dyn Value>) {
        if key.starts_with('$') {
            std::env::set_var(&key[1..], &value.to_string());
            return;
        }
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
        self.set_self(key, value);
    }

    pub fn exists(&self, key: &str) -> bool {
        if key.starts_with('$') {
            if std::env::var(&key[1..]).is_ok() {
                return true;
            } else {
                return false;
            }
        }
        if self.vars.contains_key(key) {
            return true;
        }
        if let Some(parent) = &self.parent {
            if parent
                .read()
                .expect("COULD NOT READ VAR OUT OF HASHMAP!!!")
                .exists(key)
            {
                return true;
            }
        }
        // None
        cashstd::get_stdlib_function(key).is_some()
    }

    pub fn set_self(&mut self, key: &str, value: Box<dyn Value>) {
        self.vars.insert(key.to_owned(), value);
    }
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("Ctx: ");
        s.push_str(&format!("Parent: {:?}", self.parent));
        s.push_str(&format!("Vars: {:?}", self.vars));
        write!(f, "{}", s)
    }
}
