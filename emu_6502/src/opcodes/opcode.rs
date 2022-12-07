#[cfg(test)]
#[path = "./opcode_test.rs"]
mod opcode_test;

use crate::machine::register::Register;

pub trait Opcode {
    fn execute(register: Register) -> Register;
}

