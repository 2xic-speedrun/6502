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

pub fn get_opcode(opcode: u8) -> SimpleOpcode {

    if opcode == 0x00{
        return SimpleOpcode{
            length: 1,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {
                // TODO: machine should have is running state ? 
                let mut register = state.register.get();

                register.terminated = true;

                state.register.set(register);

                return state;
            },
        };
    } 
    // JMP (oper)
    else if opcode == 0x6c {
        return SimpleOpcode{
            length: 3,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {
                let unwrap_arg_1 = unwrap_argument(arg_1);
                let unwrap_arg_2 = unwrap_argument(arg_2);

                let x: i32 = 256;                
                let address = unwrap_arg_2 * x + unwrap_arg_1;
                
                let address_lsb = state.memory.read((address as usize));
                let address_msb = state.memory.read(((address + 1) as usize));
                let pc_address = (address_msb * x + address_lsb) - 3;

                let mut register = state.register.get();

                register.pc = pc_address;

                state.register.set(register);

                return state;
            },
        };    
    } 
    // LDA #oper
    else if opcode == 0x69 {
        return SimpleOpcode{
            length: 2,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {
                let unwrap_arg_1 = unwrap_argument(arg_1);

                let mut register = state.register.get();

                register.ac += unwrap_arg_1 ;
                register.ac = register.ac % 256;

                state.register.set(register);
                return state;
            },
        };    
    } 
    // LDY #oper
    else if opcode == 0xa0 {
        return SimpleOpcode{
            length: 2,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {
                let unwrap_arg_1 = unwrap_argument(arg_1);

                let mut register = state.register.get();
                register.y = unwrap_arg_1;

                state.register.set(register);
                return state;
            },
        };    
    }
    // LDA (oper,X)
    else if opcode == 0xa1 {
        return SimpleOpcode{
            length: 2,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {
                let unwrap_arg_1 = unwrap_argument(arg_1);

                let mut register = state.register.get();
                let address = unwrap_arg_1 + register.x;

                let address_lsb = state.memory.read((address as usize));
                let address_msb = state.memory.read(((address + 1) as usize));
               
                let memory_address = (address_msb * 256 + address_lsb) ;
                register.ac = state.memory.read((memory_address) as usize);

                state.register.set(register);
                return state;
            },
        };    
    }
    // LDX #oper
    else if opcode == 0xa2 {
        return SimpleOpcode{
            length: 2,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {
                let unwrap_arg_1 = unwrap_argument(arg_1);

                let mut register = state.register.get();
                register.x = unwrap_arg_1;

                state.register.set(register);
                return state;
            },
        };    
    }
    // LDA #oper
    else if opcode == 0xa9 {
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
    // LDA (oper),Y
    else if opcode == 0xb1 {
        return SimpleOpcode{
            length: 2,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {
                let unwrap_arg_1 = unwrap_argument(arg_1);

                let mut register = state.register.get();

                let address_lsb = state.memory.read((unwrap_arg_1 as usize));
                let address_msb = state.memory.read(((unwrap_arg_1 + 1) as usize));
                
                let memory_address = (address_msb * 256 + address_lsb) + register.y;
                register.ac = state.memory.read((memory_address) as usize);

                state.register.set(register);
                return state;
            },
        };    
    }
    // STA oper
    else if opcode == 0x85 {
        return SimpleOpcode{
            length: 2,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {                
                let unwrap_arg_1 = unwrap_argument(arg_1);
                
                let address = unwrap_arg_1;
                let mut register = state.register.get();
                (state.memory).write((address as usize), register.ac);

                return state;
            },
        };    
    }
    // STX oper
    else if opcode == 0x8e {
        return SimpleOpcode{
            length: 3,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {                
                let unwrap_arg_1 = unwrap_argument(arg_1);
                let unwrap_arg_2 = unwrap_argument(arg_2);

                let x: i32 = 256;                
                let address = unwrap_arg_2 * x + unwrap_arg_1;

                let mut register = state.register.get();
                (state.memory).write((address as usize), register.x);

                return state;
            },
        };    
    }
    // STA oper
    else if opcode == 0x8d {
        return SimpleOpcode{
            length: 3,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {                
                let unwrap_arg_1 = unwrap_argument(arg_1);
                let unwrap_arg_2 = unwrap_argument(arg_2);
                let x: i32 = 256;
                
                let address = unwrap_arg_2 * x + unwrap_arg_1;
                let mut register = state.register.get();
                (state.memory).write((address as usize), register.ac);

                return state;
            },
        };    
    }
    // STA oper
    else if opcode == 0x8c {
        return SimpleOpcode{
            length: 3,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {                
                let unwrap_arg_1 = unwrap_argument(arg_1);
                let unwrap_arg_2 = unwrap_argument(arg_2);
                let x: i32 = 256;
                
                let address = unwrap_arg_2 * x + unwrap_arg_1;
                let mut register = state.register.get();
                (state.memory).write((address as usize), register.y);

                return state;
            },
        };    
    }
    // TAX
    else if opcode == 0xaa {
        return SimpleOpcode{
            length: 1,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {                                
                let mut register = state.register.get();

                register.x = register.ac;

                state.register.set(register);
                return state;
            },
        };    
    }    
    // DEX
    else if opcode == 0xca {
        return SimpleOpcode{
            length: 1,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {                                
                let mut register = state.register.get();

                register.x = (register.x - 1) % 256;
                if register.x < 0 {
                    register.x = 255 - register.x;
                }

                state.register.set(register);
                return state;
            },
        };    
    }    
    // CMP
    else if opcode == 0xc9 {
        return SimpleOpcode{
            length: 2,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {                                
                let mut register = state.register.get();

                let address = unwrap_argument(arg_1);
                register = register.set_z(
                    register.ac == address
                );
                state.register.set(register);

                return state;
            },
        };    
    }    
    // BNE
    else if opcode == 0xd0 {
        return SimpleOpcode{
            length: 2,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {                                
                let mut register = state.register.get();
                
                let unwrap_arg_1 = unwrap_argument(arg_1);
                
                if register.z() == false {
                    register.pc = (register.pc + unwrap_arg_1) % 256; // (256 - unwrap_arg_1 );
                }
                
                state.register.set(register);
                
                return state;
            },
        };    
    }
    // CPX #oper
    else if opcode == 0xe0 {
        return SimpleOpcode{
            length: 2,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {                                
                let mut register = state.register.get();
                
                let unwrap_arg_1 = unwrap_argument(arg_1);
                let address = unwrap_arg_1;

                register = register.set_z(
                    register.x == address //state.memory.read((address as usize))
                );

                state.register.set(register);
                return state;
            },
        };    
    }
    // INX
    else if opcode == 0xe8 {
        return SimpleOpcode{
            length: 1,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {                                
                let mut register = state.register.get();

                register.x += 1;

                state.register.set(register);
                return state;
            },
        };    
    }
    panic!("Unknown opcode {:#02x}", opcode);
}
