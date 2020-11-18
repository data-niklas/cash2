use crate::value::{Value, ValueResult};

pub struct ReturnValue {
    pub value: Box<dyn Value>,
}

impl ReturnValue {
    pub fn boxed(value: Box<dyn Value>) -> ValueResult {
        Ok(Box::new(Self { value }))
    }
}

impl Value for ReturnValue {
    fn get_type_name(&self) -> &'static str {
        "return"
    }

    fn clone(&self) -> Box<dyn Value> {
        Box::new(Self {
            value: (*self.value).clone(),
        })
    }
}

impl std::fmt::Debug for ReturnValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "returnvalue")
    }
}

impl std::fmt::Display for ReturnValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "returnvalue")
    }
}

pub struct BreakValue {
    pub value: Box<dyn Value>,
}

impl BreakValue {
    pub fn boxed(value: Box<dyn Value>) -> ValueResult {
        Ok(Box::new(Self { value }))
    }
}

impl Value for BreakValue {
    fn get_type_name(&self) -> &'static str {
        "break"
    }

    fn clone(&self) -> Box<dyn Value> {
        Box::new(Self {
            value: (*self.value).clone(),
        })
    }
}

impl std::fmt::Debug for BreakValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "breakvalue")
    }
}

impl std::fmt::Display for BreakValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "breakvalue")
    }
}

pub struct ContinueValue {
    pub value: Box<dyn Value>,
}

impl ContinueValue {
    pub fn boxed(value: Box<dyn Value>) -> ValueResult {
        Ok(Box::new(Self { value }))
    }
}

impl Value for ContinueValue {
    fn get_type_name(&self) -> &'static str {
        "continue"
    }

    fn clone(&self) -> Box<dyn Value> {
        Box::new(Self {
            value: (*self.value).clone(),
        })
    }
}

impl std::fmt::Debug for ContinueValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "continuevalue")
    }
}

impl std::fmt::Display for ContinueValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "continuevalue")
    }
}
