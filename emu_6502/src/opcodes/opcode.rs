use crate::machine::register::Register;

pub trait Opcode {
    fn execute(register: Register) -> Register;
}

