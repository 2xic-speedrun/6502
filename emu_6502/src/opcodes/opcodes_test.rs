use crate::machine::register::Register;
use crate::opcodes::opcodes::get_data;
use crate::opcodes::opcodes::get_data;

#[cfg(test)]
mod opcodes_test {
    #[test]
    fn it_should_execute_lda_correctly() {
        let register = Register::new();
        let opcode = get_data(0);
        opcode.func(register);

        assert_eq!(register.pc);
    }
}

