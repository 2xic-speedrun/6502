#[cfg(test)]
#[path = "./machine_test.rs"]
mod machine_test;

use crate::machine::register::Register;
use crate::opcodes::opcodes::get_data;

#[derive(Clone)]
pub struct Machine {
    program: Vec<u8>,
    pub register: Register
}

impl Machine {
    pub fn new (input: &str) -> Machine {
        let decoded = hex::decode(input).expect("Decoding failed");

        Machine {
            register: Register::new(),
            program: decoded,
        }
    }

    pub fn tick(self: Machine) -> Machine {
        let opcodeNumber = self.getOpcode();
        let opcode = get_data(opcodeNumber);
        let arg_1: Option<u8> = None;
        let arg_2: Option<u8> = None;

        if 2 == opcode.length {
            arg_1 = Some(self.program_read(self.register.pc + 1));
        } else if 3 == opcode.length {
            arg_1 = Some(self.program_read(self.register.pc + 1));
            arg_2 = Some(self.program_read(self.register.pc + 2));
        }

        let mut register = self.register;
        
        (opcode.func)(&mut register);
        register.pc += opcode.length;

        return Machine {
            register: register,
            program: self.program,
        };
    }

    fn getOpcode(&self) -> u8 {
        return self.program_read(self.register.pc);
    }

    fn program_read(&self, location: i8 ) -> u8{
        let x = self.program.get(location as usize);
        if let Some(x) = x {
            return *x;
        } else {
            panic!("Out of bounds")
        }
    }
}

