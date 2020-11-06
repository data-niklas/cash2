use std::collections::HashMap;
use std::sync::{Arc, RwLock};

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
    pub fn from_parent(parent: Arc<RwLock<Context>>) -> Context {
        Context {
            parent: Some(parent),
            vars: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<Box<dyn Value>> {
        if key.starts_with("$") {
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
            return parent
                .read()
                .expect("COULD NOT READ VAR OUT OF HASHMAP!!!")
                .get(key);
        }
        None
    }

    pub fn set(&mut self, key: &str, value: Box<dyn Value>) {
        if key.starts_with("$") {
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
