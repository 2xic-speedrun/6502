#[cfg(test)]
#[path = "./machine_test.rs"]
mod machine_test;

use crate::machine::register::Register;
use crate::opcodes::opcodes::get_opcode;
use crate::opcodes::opcodes::SimpleMachineState;
use crate::machine::memory::Memory;
use std::cell::Cell;

#[derive(Clone)]
pub struct Machine {
    program: Vec<u8>,
    pub register: Register,
    pub memory: Memory,
    
}

impl Machine {
    pub fn new (input: &str) -> Machine {
        let decoded = hex::decode(input).expect("Decoding failed");

        Machine {
            register: Register::new(),
            program: decoded,
            memory: Memory::new(),
        }
    }

    pub fn tick(self: Machine) -> Machine {
        if !(self.register.pc < (self.program.len() as i32)) {
            return self;
        }

        let opcode_number = self.get_opcode();
        let opcode = get_opcode(opcode_number);
        let mut arg_1: Option<u8> = None;
        let mut arg_2: Option<u8> = None;

        if 2 == opcode.length {
            arg_1 = Some(self.program_read(self.register.pc + 1));
        } else if 3 == opcode.length {
            arg_1 = Some(self.program_read(self.register.pc + 1));
            arg_2 = Some(self.program_read(self.register.pc + 2));
        }

        let mut register = self.register;
        let mut memory = self.memory;
        let mut machine = SimpleMachineState{
            register: Cell::new(register),
            memory: &mut memory,
        };
        let machine_updated = (opcode.func)(machine, arg_1, arg_2);

        register = machine_updated.register.get();
        register.pc += opcode.length;

        return Machine {
            register: register,
            memory: memory,
            program: self.program,
        };
    }

    fn get_opcode(&self) -> u8 {
        return self.program_read(self.register.pc);
    }

    fn program_read(&self, location: i32 ) -> u8{
        let x = self.program.get(location as usize);
        if let Some(x) = x {
            return *x;
        } else {
            panic!("Out of bounds")
        }
    }
}

