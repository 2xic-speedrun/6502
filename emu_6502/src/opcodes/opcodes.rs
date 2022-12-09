use crate::machine::register::Register;
use crate::machine::memory::Memory;
use std::cell::Cell;

//#[derive(Clone)]
pub struct SimpleMachineState<'a> {
    pub register: Cell<Register>,
    pub memory: &'a mut Memory,
}

pub struct SimpleOpcode {
    pub length: i32,
    pub func: fn(state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>) -> SimpleMachineState,
}

fn unwrap_argument(arg: Option<u8>) -> i32 {
    if let Some(arg) = arg {
        return arg.try_into().unwrap()
    } else {
        panic!("Out of bounds")
    }    
}

pub fn get_opcode(index: u8) -> SimpleOpcode {

    // LDA #oper
    if index == 0xa9 {
        return SimpleOpcode{
            length: 2,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {
                let unwrap_arg_1 = unwrap_argument(arg_1);

                let mut register = state.register.get();
                register.ac = unwrap_arg_1;

                state.register.set(register);
                return state;
            },
        };    
    } 
    // STA oper
    else if index == 0x8d {
        return SimpleOpcode{
            length: 3,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {                
                let unwrap_arg_1 = unwrap_argument(arg_1);
                let unwrap_arg_2 = unwrap_argument(arg_2);
                let x: i32 = 100;
                
                let address = unwrap_arg_2 * x + unwrap_arg_1;
                let mut register = state.register.get();
                (state.memory).write((address as usize), register.ac);

                return state;
            },
        };    
    }
    panic!("Unknown opcode");
}
