#[cfg(test)]
mod test_0x {
    use crate::cpu::test::run_test;
    use crate::cpu::*;

    #[test]
    fn test_00() {
        let machine = run_test(&[0x00], |_| {});
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_01() {
        let machine = run_test(&[0x01, 0x34, 0x12], |_| {});
        assert_eq!(machine.cpu.pc, 0x0003);
        assert_eq!(machine.cpu.bc.value(), 0x1234);
        assert_eq!(machine.cpu.t_cycles, 12);
    }

    #[test]
    fn test_02() {
        let machine = run_test(&[0x02], |machine| {
            *machine.cpu.a_mut() = 0x42;
            *machine.cpu.bc.value_mut() = 0xC000;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.mmu.borrow().read(0xC000), 0x42);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_03() {
        let machine = run_test(&[0x03], |machine| {
            *machine.cpu.bc.value_mut() = 0xC000;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.bc.value(), 0xC001);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_03_overflow() {
        let machine = run_test(&[0x03], |machine| {
            *machine.cpu.bc.value_mut() = 0xFFFF;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.bc.value(), 0x0000);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_04() {
        let machine = run_test(&[0x04], |machine| {
            *machine.cpu.b_mut() = 0x01;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.b(), 0x02);
        assert_eq!(machine.cpu.f(), 0);
        assert_eq!(machine.cpu.t_cycles, 4);
    }
    #[test]
    fn test_04_zero() {
        let machine = run_test(&[0x04], |machine| {
            *machine.cpu.b_mut() = 0xFF;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.b(), 0x00);
        assert_eq!(machine.cpu.f(), ZERO_FLAG | HALF_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_04_half_carry() {
        let machine = run_test(&[0x04], |machine| {
            *machine.cpu.b_mut() = 0x0F;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.b(), 0x10);
        assert_eq!(machine.cpu.f(), HALF_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_05() {
        let machine = run_test(&[0x05], |machine| {
            *machine.cpu.b_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.b(), 0x41);
        assert_eq!(machine.cpu.f(), SUB_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_05_zero() {
        let machine = run_test(&[0x05], |machine| {
            *machine.cpu.b_mut() = 0x01;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.b(), 0x00);
        assert_eq!(machine.cpu.f(), ZERO_FLAG | SUB_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_05_half_carry() {
        let machine = run_test(&[0x05], |machine| {
            *machine.cpu.b_mut() = 0x00;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.b(), 0xFF);
        assert_eq!(machine.cpu.f(), HALF_FLAG | SUB_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_06() {
        let machine = run_test(&[0x06, 0x42], |machine| {
            *machine.cpu.b_mut() = 0x00;
        });
        assert_eq!(machine.cpu.pc, 0x0002);
        assert_eq!(machine.cpu.b(), 0x42);
        assert_eq!(machine.cpu.f(), 0);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_07() {
        let machine = run_test(&[0x07], |machine| {
            *machine.cpu.a_mut() = 0x77;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0xEE);
        assert_eq!(machine.cpu.f(), 0);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_07_carry() {
        let machine = run_test(&[0x07], |machine| {
            *machine.cpu.a_mut() = 0xF7;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0xEF);
        assert_eq!(machine.cpu.f(), CARRY_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_08() {
        let machine = run_test(&[0x08, 0x00, 0xC0], |machine| {
            machine.cpu.sp = 0x4217;
        });
        assert_eq!(machine.cpu.pc, 0x0003);
        assert_eq!(machine.cpu.mmu.borrow().read(0xC000), 0x17);
        assert_eq!(machine.cpu.mmu.borrow().read(0xC001), 0x42);
        assert_eq!(machine.cpu.t_cycles, 20);
    }

    #[test]
    fn test_09() {
        let machine = run_test(&[0x09], |machine| {
            *machine.cpu.hl.value_mut() = 0x0101;
            *machine.cpu.bc.value_mut() = 0x0101;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.hl.value(), 0x0101 + 0x0101);
        assert_eq!(machine.cpu.f(), 0);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_0a() {
        let machine = run_test(&[0x0A, 0xFD, 0x42], |machine| {
            *machine.cpu.bc.value_mut() = 0x0002;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_0b() {
        let machine = run_test(&[0x0B], |machine| {
            *machine.cpu.bc.value_mut() = 0x0002;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.bc.value(), 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_0c() {
        let machine = run_test(&[0x0C], |machine| {
            *machine.cpu.c_mut() = 0x01;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.c(), 0x02);
        assert_eq!(machine.cpu.f(), 0);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_0c_zero() {
        let machine = run_test(&[0x0C], |machine| {
            *machine.cpu.c_mut() = 0xFF;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.c(), 0x00);
        assert_eq!(machine.cpu.f(), ZERO_FLAG | HALF_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_0c_half_carry() {
        let machine = run_test(&[0x0C], |machine| {
            *machine.cpu.c_mut() = 0x0F;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.c(), 0x10);
        assert_eq!(machine.cpu.f(), HALF_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_0d() {
        let machine = run_test(&[0x0D], |machine| {
            *machine.cpu.c_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.c(), 0x41);
        assert_eq!(machine.cpu.f(), SUB_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_0d_zero() {
        let machine = run_test(&[0x0D], |machine| {
            *machine.cpu.c_mut() = 0x01;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.c(), 0x00);
        assert_eq!(machine.cpu.f(), ZERO_FLAG | SUB_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_0d_half_carry() {
        let machine = run_test(&[0x0D], |machine| {
            *machine.cpu.c_mut() = 0x00;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.c(), 0xFF);
        assert_eq!(machine.cpu.f(), HALF_FLAG | SUB_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_0e() {
        let machine = run_test(&[0x0E, 0x42], |machine| {
            *machine.cpu.c_mut() = 0x00;
        });
        assert_eq!(machine.cpu.pc, 0x0002);
        assert_eq!(machine.cpu.c(), 0x42);
        assert_eq!(machine.cpu.f(), 0);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_0f() {
        let machine = run_test(&[0x0F], |machine| {
            *machine.cpu.a_mut() = 0xEE;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0x77);
        assert_eq!(machine.cpu.f(), 0);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_0f_carry() {
        let machine = run_test(&[0x0F], |machine| {
            *machine.cpu.a_mut() = 0xEF;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0xF7);
        assert_eq!(machine.cpu.f(), CARRY_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }
}
