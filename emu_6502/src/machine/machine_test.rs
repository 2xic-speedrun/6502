
#[cfg(test)]
mod machine_test {
    #[test]
    fn it_should_move_pc() {
        use crate::machine::machine::Machine;
        let input = "a901";
        let mut machine = Machine::new(input);
        assert_eq!(machine.program.len(), 2);

        machine = machine.tick();
        assert_eq!(machine.register.pc, 2);
        assert_eq!(machine.register.ac, 1);

        machine = machine.tick();
        assert_eq!(machine.register.pc, 2);
    }
}

