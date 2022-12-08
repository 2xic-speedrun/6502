use crate::machine::register::Register;

pub struct SimpleOpcode {
    pub length: i8,
    pub func: fn(&mut Register, arg_1: Option<u8>, arg_2: Option<u8>) -> Register,
}

fn unwrap_argument(arg: Option<u8>) -> i8 {
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
            func: |i: &mut Register, arg_1: Option<u8>, arg_2: Option<u8>| -> Register {                
                let unwrap_arg_1 = unwrap_argument(arg_1);
                i.ac = unwrap_arg_1;
                return *i;
            },
        };    
    } 
    // STA oper
    else if (index == 0x8d) {
        return SimpleOpcode{
            length: 2,
            func: |i: &mut Register, arg_1: Option<u8>, arg_2: Option<u8>| -> Register {                
                let unwrap_arg_1 = unwrap_argument(arg_1);
                i.ac = unwrap_arg_1;
                return *i;
            },
        };    
    }
    panic!("Unknown opcode");
}
