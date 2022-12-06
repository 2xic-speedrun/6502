
#[cfg(test)]
mod machine_test {
    #[test]
    fn it_works() {
        use crate::machine::machine::Machine;
        let input = "a9018d";
        let machine = Machine::new(input);
        assert_eq!(machine.program.len(), 3);
    }
}

