
#[cfg(test)]
mod machine_test {
    #[test]
    fn it_should_move_pc() {
        use crate::machine::machine::Machine;
        let input = "a9018d";
        let mut machine = Machine::new(input);
        assert_eq!(machine.program.len(), 3);

        machine = machine.tick();
        assert_eq!(machine.register.pc, 3);
    }
}

