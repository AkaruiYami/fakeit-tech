use crate::engine::FakeModule;
use std::sync::Mutex;

// Central registry (global)
lazy_static::lazy_static! {
    pub static ref MODULES: Mutex<Vec<Box<dyn FakeModule + Send>>> = Mutex::new(Vec::new());
}

// Function modules call to register themselves
pub fn register(module: Box<dyn FakeModule + Send>) {
    MODULES.lock().unwrap().push(module);
}

// Function to retrieve registered modules
pub fn get_registered() -> Vec<Box<dyn FakeModule + Send>> {
    MODULES.lock().unwrap().drain(..).collect()
}
