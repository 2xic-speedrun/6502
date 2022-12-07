#[derive(Clone, Copy)]
pub struct Register {
    pub pc: i8,
    ac: i8,
    x: i8,
    y: i8,
    sr: i8,
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