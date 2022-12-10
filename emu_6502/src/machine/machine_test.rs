
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

    #[test]
    fn it_should_write_to_screen() {
        use crate::machine::machine::Machine;
        let input = "a9018d0002";
        let mut machine = Machine::new(input);
        assert_eq!(machine.program.len(), 5);

        machine = machine.tick();
        assert_eq!(machine.register.pc, 2);
        assert_eq!(machine.register.ac, 1);

        machine = machine.tick();
        assert_eq!(machine.register.pc, 5);
        assert_eq!(machine.register.ac, 1);
        machine = machine.tick();

        assert_eq!(machine.memory.read(200), 1);
    }

    #[test]
    fn it_should_run_example_program_2() {
        use crate::machine::machine::Machine;
        let input = "a9c0aae869c400";
        let mut machine = Machine::new(input);

        machine = machine.tick();
        assert_eq!(machine.register.ac, 0xc0);

        machine = machine.tick();
        assert_eq!(machine.register.x, 0xc0);

        machine = machine.tick();
        assert_eq!(machine.register.x, 0xc1);

        machine = machine.tick();
        assert_eq!(machine.register.ac, 0x84);

        machine = machine.tick();
    }

    #[test]
    fn it_should_run_example_program_3() {
        use crate::machine::machine::Machine;
        let input = "a208ca8e0002e003d0f88e010200";
        let mut machine = Machine::new(input);

        machine = machine.tick();
        assert_eq!(machine.register.x, 0x08);

        machine = machine.tick();
        assert_eq!(machine.register.x, 0x07);

        machine = machine.tick();

        machine = machine.tick();

        // BNE
        machine = machine.tick();
        assert_eq!(machine.register.pc, 0x02);

        while !machine.register.terminated {
            machine = machine.tick();
        }

        assert_eq!(machine.register.terminated, true);
   
    }

    #[test]
    fn it_should_run_example_program_4() {
        use crate::machine::machine::Machine;
        let input = "a901c902d002852200";
        let mut machine = Machine::new(input);

        machine = machine.tick();
        assert_eq!(machine.register.ac, 0x01);

        machine = machine.tick();
        machine = machine.tick();

        assert_eq!(machine.register.pc, 0x08);
        machine = machine.tick();
        assert_eq!(machine.register.pc, 0x09);

        while !machine.register.terminated {
            machine = machine.tick();
        }
        assert_eq!(machine.register.ac, 0x01);
    }

    #[test]
    fn it_should_run_example_program_5() {
        use crate::machine::machine::Machine;
        let input = "a90185f0a9cc85f16cf000";
        let mut machine = Machine::new(input);

        machine = machine.tick();
        assert_eq!(machine.register.ac, 0x01);

        machine = machine.tick();
        assert_eq!(machine.register.ac, 0x1);

        machine = machine.tick();
        assert_eq!(machine.register.ac, 0xcc);

        machine = machine.tick();
        assert_eq!(machine.register.ac, 0xcc);

        machine = machine.tick();
        assert_eq!(machine.register.ac, 0xcc);

        machine = machine.tick();
        assert_eq!(machine.register.pc, 0xcc01);
    }

    #[test]
    fn it_should_run_example_program_6() {
        use crate::machine::machine::Machine;
        let input = "a201a9058501a9078502a00a8c0507a100";
        let mut machine = Machine::new(input);

        while !machine.register.terminated {
            machine = machine.tick();
        }

        assert_eq!(machine.memory.read(0x0705 as usize), 0x0a);
        assert_eq!(machine.register.ac, 0x0a);
    }

    #[test]
    fn it_should_run_example_program_7() {
        use crate::machine::machine::Machine;
        let input = "a001a9038501a9078502a20a8e0407b101";
        let mut machine = Machine::new(input);

        while !machine.register.terminated {
            machine = machine.tick();
        }

        assert_eq!(machine.memory.read(1 as usize), 3);
        assert_eq!(machine.memory.read(2 as usize), 7);

        assert_eq!(machine.register.ac, 0xa);
    }
}

