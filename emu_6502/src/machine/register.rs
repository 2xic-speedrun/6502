#[derive(Clone, Copy)]
pub struct Register {
    // TODO i16
    pub pc: i32,
    // accumulator
    pub ac: i32,
    x: i32,
    y: i32,
    // Status register
    sr: i32,
    //  Stack pointer
    sp: i32
}

impl Register {
    pub fn new() -> Register {
        Register {
            pc: 0,
            ac: 0,
            x: 0,
            y: 0,
            sr: 0,
            sp: 0
        }
    }
}