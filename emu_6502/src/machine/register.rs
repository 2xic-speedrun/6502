#[derive(Clone, Copy)]
pub struct Register {
    // TODO i16
    pub pc: i8,
    // accumulator
    pub ac: i8,
    x: i8,
    y: i8,
    // Status register
    sr: i8,
    //  Stack pointer
    sp: i8
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