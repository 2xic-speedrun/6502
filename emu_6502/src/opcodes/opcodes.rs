#[cfg(test)]
#[path = "./opcodes_test.rs"]
mod opcodes_test;

use std::collections::HashMap;
use std::sync::Mutex;
use crate::machine::register::Register;

pub struct SimpleOpcode {
    pub length: i8,
    pub func: fn(&mut Register) -> Register,
}


pub fn get_data(index: u8) -> SimpleOpcode {
    return SimpleOpcode{
        length: 3,
        func: |i: &mut Register| -> Register {
            //i.pc += 1;
            return *i;
        },
    };
}
