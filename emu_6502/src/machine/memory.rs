#[derive(Clone)]
pub struct Memory {
    storage: Vec<i8>
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            storage: Vec::new(),
        }
    }
}