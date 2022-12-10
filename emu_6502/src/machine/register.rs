#[derive(Clone, Copy)]
pub struct Register {
    // TODO i16
    pub pc: i32,
    // accumulator
    pub ac: i32,
    pub x: i32,
    pub y: i32,
    // Status register
    sr: i8,
    //  Stack pointer
    sp: i32,
    // Not actually a register, but convenient to have it here
    pub terminated: bool,
}

static z_bit: i8 = 1;

impl Register {
    pub fn new() -> Register {
        Register {
            pc: 0,
            ac: 0,
            x: 0,
            y: 0,
            sr: 0,
            sp: 0,
            terminated: false,
        }
    }

    pub fn z(self) -> bool {
        return ((self.sr >> z_bit) & 1) == 1;
    }

    pub fn set_z(mut self, value: bool) -> Register {
        if (value) {
            self.sr = self.sr | (1 << z_bit);
            println!("I set it now");
            println!("{}", self.z());
        } else {
            self.sr = self.sr & !(1 << z_bit);
        }

        return self;
    }
}
