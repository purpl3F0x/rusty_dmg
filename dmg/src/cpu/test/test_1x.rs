#[cfg(test)]
mod test_1x {
    use crate::cpu::test::run_test;
    use crate::cpu::*;

    #[test]
    fn test_10() {
        let machine = run_test(&[0x10], |_| {});
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.mode, CPUMode::Stop);
    }

    #[test]
    fn test_01() {
        let machine = run_test(&[0x11, 0x34, 0x12], |_| {});
        assert_eq!(machine.cpu.pc, 0x0003);
        assert_eq!(machine.cpu.de.value(), 0x1234);
        assert_eq!(machine.cpu.t_cycles, 12);
    }

    #[test]
    fn test_12() {
        let machine = run_test(&[0x12], |machine| {
            *machine.cpu.a_mut() = 0x42;
            *machine.cpu.de.value_mut() = 0xC000;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.mmu.borrow().read(0xC000), 0x42);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_13() {
        let machine = run_test(&[0x13], |machine| {
            *machine.cpu.de.value_mut() = 0xC000;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.de.value(), 0xC001);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_14() {
        let machine = run_test(&[0x14], |machine| {
            *machine.cpu.d_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.d(), 0x43);
        assert_eq!(machine.cpu.f(), 0);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_14_zero() {
        let machine = run_test(&[0x14], |machine| {
            *machine.cpu.d_mut() = 0xFF;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.d(), 0x00);
        assert_eq!(machine.cpu.f(), ZERO_FLAG | HALF_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_14_half_carry() {
        let machine = run_test(&[0x14], |machine| {
            *machine.cpu.d_mut() = 0x0F;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.d(), 0x10);
        assert_eq!(machine.cpu.f(), HALF_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_15() {
        let machine = run_test(&[0x15], |machine| {
            *machine.cpu.d_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.d(), 0x41);
        assert_eq!(machine.cpu.f(), SUB_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_15_zero() {
        let machine = run_test(&[0x15], |machine| {
            *machine.cpu.d_mut() = 0x01;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.d(), 0x00);
        assert_eq!(machine.cpu.f(), ZERO_FLAG | SUB_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_15_half_carry() {
        let machine = run_test(&[0x15], |machine| {
            *machine.cpu.d_mut() = 0x10;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.d(), 0x0F);
        assert_eq!(machine.cpu.f(), HALF_FLAG | SUB_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_16() {
        let machine = run_test(&[0x16, 0x42], |machine| {
            *machine.cpu.d_mut() = 0x00; // Initial value
        });
        assert_eq!(machine.cpu.pc, 0x0002);
        assert_eq!(machine.cpu.d(), 0x42);
        assert_eq!(machine.cpu.f(), 0);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_17() {
        let machine = run_test(&[0x17], |machine| {
            *machine.cpu.a_mut() = 0x55;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0xAA);
        assert_eq!(machine.cpu.f(), 0);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_17_carry() {
        let machine = run_test(&[0x17], |machine| {
            *machine.cpu.a_mut() = 0xAA;
            machine.cpu.set_flag(CARRY_FLAG);
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0x55);
        assert_eq!(machine.cpu.f(), CARRY_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_18() {
        let machine = run_test(&[0x18, 0x02, 0x00, 0xED], |_| {});
        assert_eq!(machine.cpu.pc, 0x0004);
        assert_eq!(machine.cpu.t_cycles, 12);
        assert_eq!(machine.instruction_counter, 1);
    }

    #[test]
    fn test_19() {
        let machine = run_test(&[0x19], |machine| {
            *machine.cpu.hl.value_mut() = 0x0001;
            *machine.cpu.de.value_mut() = 0x0002;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.hl.value(), 0x0003);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_0a() {
        let machine = run_test(&[0x1A, 0xFD, 0x42], |machine| {
            *machine.cpu.de.value_mut() = 0x0002;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_1b() {
        let machine = run_test(&[0x1B], |machine| {
            *machine.cpu.de.value_mut() = 0x0002;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.de.value(), 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_1c() {
        let machine = run_test(&[0x1C], |machine| {
            *machine.cpu.e_mut() = 0x01;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.e(), 0x02);
        assert_eq!(machine.cpu.f(), 0);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_1c_zero() {
        let machine = run_test(&[0x1C], |machine| {
            *machine.cpu.e_mut() = 0xFF;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.e(), 0x00);
        assert_eq!(machine.cpu.f(), ZERO_FLAG | HALF_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_1c_half_carry() {
        let machine = run_test(&[0x1C], |machine| {
            *machine.cpu.e_mut() = 0x0F;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.e(), 0x10);
        assert_eq!(machine.cpu.f(), HALF_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_1d() {
        let machine = run_test(&[0x1D], |machine| {
            *machine.cpu.e_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.e(), 0x41);
        assert_eq!(machine.cpu.f(), SUB_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_1d_zero() {
        let machine = run_test(&[0x1D], |machine| {
            *machine.cpu.e_mut() = 0x01;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.e(), 0x00);
        assert_eq!(machine.cpu.f(), ZERO_FLAG | SUB_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_1d_half_carry() {
        let machine = run_test(&[0x1D], |machine| {
            *machine.cpu.e_mut() = 0x10;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.e(), 0x0F);
        assert_eq!(machine.cpu.f(), HALF_FLAG | SUB_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_1e() {
        let machine = run_test(&[0x1E, 0x42], |machine| {
            *machine.cpu.e_mut() = 0x00; // Initial value
        });
        assert_eq!(machine.cpu.pc, 0x0002);
        assert_eq!(machine.cpu.e(), 0x42);
        assert_eq!(machine.cpu.f(), 0);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_1f() {
        let machine = run_test(&[0x1F], |machine| {
            *machine.cpu.a_mut() = 0xAA;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0x55);
        assert_eq!(machine.cpu.f(), 0);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_1f_carry() {
        let machine = run_test(&[0x1F], |machine| {
            *machine.cpu.a_mut() = 0x55;
            machine.cpu.set_flag(CARRY_FLAG);
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0xAA);
        assert_eq!(machine.cpu.f(), CARRY_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }
}
