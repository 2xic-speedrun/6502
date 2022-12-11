
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

        assert_eq!(machine.memory.read(0x200), 1);
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

        assert_eq!(machine.register.ac, 0x0a);
    }

    #[test]
    fn it_should_run_example_program_8() {
        use crate::machine::machine::Machine;
        let input = "a200a0008a99000248e8c8c010d0f568990002c8c020d0f7";
        let mut machine = Machine::new(input);

        while !machine.register.terminated {
            machine = machine.tick();
        }

        assert_eq!(machine.register.sp, 0xff);
    }

    #[test]
    fn it_should_run_example_program_9() {
        use crate::machine::machine::Machine;
        let input = "a9034c08060000008d0002";
        let mut machine = Machine::new(input);

        while !machine.register.terminated {
            machine = machine.tick();
        }

        assert_eq!(machine.register.sp, 0xff);
        assert_eq!(machine.register.ac, 3);
        assert_eq!(machine.register.pc, 11);
    }

    #[test]
    fn it_should_run_example_program_10() {
        use crate::machine::machine::Machine;
        let input = "200906200c06201206a20060e8e005d0fb6000";
        let mut machine = Machine::new(input);

        while !machine.register.terminated {
            machine = machine.tick();
        }

        assert_eq!(machine.register.sp, 0xfd - 1);
        assert_eq!(machine.register.ac, 0);
        assert_eq!(machine.register.x, 5);
        assert_eq!(machine.register.y, 0);
    }

    #[test]
    fn it_should_run_snake() {
        use crate::machine::machine::Machine;
        let input = "200606203806200d06202a0660a9028502a9048503a9118510a9108512a90f8514a90485118513851560a5fe8500a5fe2903186902850160204d06208d0620c306201907202007202d074c3806a5ffc977f00dc964f014c973f01bc961f02260a9042402d026a901850260a9082402d01ba902850260a9012402d010a904850260a9022402d005a9088502606020940620a80660a500c510d00da501c511d007e603e603202a0660a202b510c510d006b511c511f009e8e8e403f0064caa064c350760a603ca8ab5109512ca10f9a5024ab0094ab0194ab01f4ab02fa51038e9208510900160c611a901c511f02860e610a91f2410f01f60a5101869208510b00160e611a906c511f00c60c610a510291fc91ff001604c3507a000a5fe910060a603a9008110a200a901811060a200eaeacad0fb60";
        let mut machine = Machine::new(input);

        while !machine.register.terminated {
            machine = machine.tick();
        }
    }
}

