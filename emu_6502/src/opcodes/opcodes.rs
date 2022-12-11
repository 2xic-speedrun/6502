use crate::machine::register::Register;
use crate::machine::memory::Memory;
use std::cell::Cell;

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
        panic!("Argument is out of bounds. Have you set the correct size ? ")
    }    
}

pub fn push(state: SimpleMachineState, value: i32) -> SimpleMachineState {
    let mut register = state.register.get();

    state.memory.write(
        (0x0100 + register.sp) as usize,
        value
    );

    register.sp -= 1;
    if register.sp < 0{
        register.sp = 0xff;
    }

    state.register.set(register);

    return state;
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
    // BPL
    else if opcode == 0x10 {
        return SimpleOpcode{
            length: 2,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {
                let mut register = state.register.get();
                
                let unwrap_arg_1 = unwrap_argument(arg_1);
                
                if register.n() == false {
                    register.pc = (register.pc + unwrap_arg_1) % 256; // (256 - unwrap_arg_1 );
                }
                
                state.register.set(register);
                state.register.set(register);
                return state;
            },
        };    
    }
    // CLC
    else if opcode == 0x18 {
        return SimpleOpcode{
            length: 1,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {
                let mut register = state.register.get();
                register = register.set_c(
                    false
                );

                state.register.set(register);
                return state;
            },
        };    
    }
    // JSR
    else if opcode == 0x20 {
        return SimpleOpcode{
            length: 3,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {
                let pc_address = state.register.get().clone().pc;
                // TODO: This is wrong, since the pc is 16 bits
                //      it should be split in two
                let new_state = push(state, pc_address + 3);

                let mut register = new_state.register.get();
                let unwrap_arg_1 = unwrap_argument(arg_1);
                let unwrap_arg_2 = unwrap_argument(arg_2);

                let x: i32 = 256;                
                let pc_address = (unwrap_arg_2 * x + unwrap_arg_1) - 3;

                // TODO: We should load into the correct memory slot instead of doing this
                register.pc = (pc_address - 0x0600);

                new_state.register.set(register);

                return new_state;
            },
        };
    } 
    // BIT
    else if opcode == 0x24 {
        return SimpleOpcode{
            length: 2,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {
                let mut register = state.register.get();
                let unwrap_arg_1 = unwrap_argument(arg_1);

                register = register.set_z(
                    (register.ac & unwrap_arg_1) == 0
                );

                state.register.set(register);

                return state;
            },
        };
    } 
    // AND
    else if opcode == 0x29 {
        return SimpleOpcode{
            length: 2,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {
                let unwrap_arg_1 = unwrap_argument(arg_1);

                let mut register = state.register.get();

                register.ac = (register.ac & unwrap_arg_1);
                register = register.set_z(
                    register.ac == 0
                );

                state.register.set(register);
                return state;
            },
        };    
    }
    // PHA
    else if opcode == 0x48 {
        return SimpleOpcode{
            length: 1,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {
                let mut register = state.register.get();

                return push(state, register.ac);
            },
        };
    } 
    // JMP oper
    else if opcode == 0x4c {
        return SimpleOpcode{
            length: 3,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {
                let mut register = state.register.get();
                let unwrap_arg_1 = unwrap_argument(arg_1);
                let unwrap_arg_2 = unwrap_argument(arg_2);

                let x: i32 = 256;                
                let pc_address = unwrap_arg_2 * x + unwrap_arg_1;
                
                // TODO: We should load into the correct memory slot instead of doing this
                register.pc = (pc_address - 0x0600) - 3;

                state.register.set(register);

                return state;
            },
        };
    } 
    // RTS
    else if opcode == 0x60 {
        return SimpleOpcode{
            length: 1,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {
                // TODO: This should also be behind a separate function
                let mut register = state.register.get();
                // TODO: this is dirty
                let stack_value = state.memory.read((0x0100 + (register.sp + 1)) as usize);
                register.sp = (register.sp + 1) % (0xff + 1);

                register.pc = stack_value - 1;

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
    //PLA
    else if opcode == 0x68 {
        return SimpleOpcode{
            length: 1,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {
                let mut register = state.register.get();

                register.ac = state.memory.read((0x0100 + register.sp) as usize);
                register.sp = (register.sp + 1) % (0xff + 1);

                state.register.set(register);
                return state;
            },
        };    
    } 
    // ADC
    else if opcode == 0x69 {
        return SimpleOpcode{
            length: 2,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {
                let unwrap_arg_1 = unwrap_argument(arg_1);

                let mut register = state.register.get();

                register.ac += unwrap_arg_1 + (register.z() as i32);
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
    // LDA oper
    else if opcode == 0xa5 {
        return SimpleOpcode{
            length: 2,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {
                let unwrap_arg_1 = unwrap_argument(arg_1);

                let mut register = state.register.get();
                register.ac = state.memory.read((unwrap_arg_1 as usize));

                state.register.set(register);
                return state;
            },
        };    
    }
    // LDX oper
    else if opcode == 0xa6 {
        return SimpleOpcode{
            length: 2,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {
                let unwrap_arg_1 = unwrap_argument(arg_1);

                let mut register = state.register.get();
                register.x = state.memory.read((unwrap_arg_1 as usize));

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
    // LDA oper
    else if opcode == 0xb5 {
        return SimpleOpcode{
            length: 2,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {
                let unwrap_arg_1 = unwrap_argument(arg_1);

                let mut register = state.register.get();
                register.ac = state.memory.read((unwrap_arg_1 + register.x)as usize );

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
    // TXA
    else if opcode == 0x8a {
        return SimpleOpcode{
            length: 1,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {                
                let mut register = state.register.get();
                register.ac = register.x;

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
    // STA oper,X
    else if opcode == 0x95 {
        return SimpleOpcode{
            length: 2,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {                
                let unwrap_arg_1 = unwrap_argument(arg_1);
                let x: i32 = 256;
                
                let mut register = state.register.get();
                let address = unwrap_arg_1 + register.x;
                (state.memory).write((address as usize), register.ac);

                return state;
            },
        };    
    }
    // STA oper,Y
    else if opcode == 0x99 {
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
    // LDA
    else if opcode == 0xad {
        return SimpleOpcode{
            length: 3,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {                                
                let mut register = state.register.get();

                let unwrap_arg_1 = unwrap_argument(arg_1);
                let unwrap_arg_2 = unwrap_argument(arg_2);
                let x: i32 = 256;
                
                let address = unwrap_arg_2 * x + unwrap_arg_1;
                let mut register = state.register.get();

                register.ac = state.memory.read(address as usize);

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
    // CPY
    else if opcode == 0xc0 {
        return SimpleOpcode{
            length: 2,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {                                
                let mut register = state.register.get();
                let value = unwrap_argument(arg_1);
                let mut register = state.register.get();

                register = register.set_z(
                    value == register.y
                );

                state.register.set(register);

                return state;
            },
        };    
    }  
    // INY
    else if opcode == 0xc8 {
        return SimpleOpcode{
            length: 1,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {                                
                let mut register = state.register.get();

                register.y = (register.y + 1) % 256;
                state.register.set(register);

                return state;
            },
        };    
    }  
    // CMP
    else if opcode == 0xc5 {
        return SimpleOpcode{
            length: 2,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {                                
                let mut register = state.register.get();

                let address = state.memory.read(unwrap_argument(arg_1) as usize);
                register = register.set_z(
                    register.ac == address
                );
                register = register.set_n(
                    (register.ac - address) < 0 && register.ac != address
                );
                register = register.set_c(
                    (register.ac > address) 
                );
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
                register = register.set_n(
                    (register.ac - address) < 0 && register.ac != address
                );
                register = register.set_c(
                    (register.ac > address) 
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
                let value = unwrap_arg_1;

                register = register.set_z(
                    register.x == value
                );
                register = register.set_n(
                    (register.x - value) < 0 && register.x != value
                );
                register = register.set_c(
                    (register.x > value) 
                );

                state.register.set(register);
                return state;
            },
        };    
    }
    // CPX #oper
    else if opcode == 0xe4 {
        return SimpleOpcode{
            length: 2,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {                                
                let mut register = state.register.get();
                
                let value = state.memory.read(unwrap_argument(arg_1) as usize);

                register = register.set_z(
                    register.x == value
                );
                register = register.set_n(
                    register.x < value
                );
                register = register.set_c(
                    register.x > value
                );

                state.register.set(register);
                return state;
            },
        };    
    }
    // INC oper
    else if opcode == 0xe6 {
        return SimpleOpcode{
            length: 2,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {                                
                let mut register = state.register.get();
                
                let value = state.memory.read(unwrap_argument(arg_1) as usize);

                state.memory.write(unwrap_argument(arg_1) as usize, value + 1);

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

                register.x = (register.x + 1) % 256;

                state.register.set(register);
                return state;
            },
        };    
    }
    // BEQ
    else if opcode == 0xf0 {
        return SimpleOpcode{
            length: 2,
            func: |state: SimpleMachineState, arg_1: Option<u8>, arg_2: Option<u8>| -> SimpleMachineState {                                
                let mut register = state.register.get();
                
                let unwrap_arg_1 = unwrap_argument(arg_1);
                
                if register.z() == true {
                    register.pc = (register.pc + unwrap_arg_1) % 256; // (256 - unwrap_arg_1 );
                }
                
                state.register.set(register);
                
                return state;
            },
        };    
    }
    panic!("Unknown opcode {:#02x}", opcode);
}
