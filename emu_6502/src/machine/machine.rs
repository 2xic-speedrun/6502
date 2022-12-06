#[cfg(test)]
#[path = "./machine_test.rs"]
mod machine_test;

use crate::machine::register::Register;

pub struct Machine {
    program: Vec<u8>,
    register: Register
}

impl Machine {
    pub fn new (input: &str) -> Machine {
        let decoded = hex::decode(input).expect("Decoding failed");

        Machine {
            register: Register::new(),
            program: decoded,
        }
    }
}

