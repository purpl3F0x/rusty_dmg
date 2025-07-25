#[cfg(test)]
mod test_2x {
    use crate::cpu::test::run_test;
    use crate::cpu::*;

    #[test]
    fn test_20() {
        let machine = run_test(&[0x20, 0x01, 0xFD, 0xFD], |_| {});
        assert_eq!(machine.cpu.pc, 0x0003);
        assert_eq!(machine.cpu.t_cycles, 12);
    }

    #[test]
    fn test_20_negative() {
        let machine = run_test(&[0x00, 0xFD, 0x20, -3i8 as u8], |machine| {
            machine.cpu.pc = 0x0002;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 12);
    }

    #[test]
    fn test_20_no_jump() {
        let machine = run_test(&[0x20, 0x01, 0xFD, 0x00], |machine| {
            machine.cpu.set_flag(ZERO_FLAG);
        });
        assert_eq!(machine.cpu.pc, 0x0002);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_21() {
        let machine = run_test(&[0x21, 0x34, 0x12], |_| {});
        assert_eq!(machine.cpu.pc, 0x0003);
        assert_eq!(machine.cpu.hl.value(), 0x1234);
        assert_eq!(machine.cpu.t_cycles, 12);
    }

    #[test]
    fn test_22() {
        let machine = run_test(&[0x22], |machine| {
            *machine.cpu.a_mut() = 0x42;
            *machine.cpu.hl.value_mut() = 0xC000;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.mmu.borrow().read(0xC000), 0x42);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_23() {
        let machine = run_test(&[0x23], |machine| {
            *machine.cpu.hl.value_mut() = 0xC000;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.hl.value(), 0xC001);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_24() {
        let machine = run_test(&[0x24], |machine| {
            *machine.cpu.h_mut() = 0x01;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.h(), 0x02);
        assert_eq!(machine.cpu.f(), 0);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_24_zero() {
        let machine = run_test(&[0x24], |machine| {
            *machine.cpu.h_mut() = 0xFF;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.h(), 0x00);
        assert_eq!(machine.cpu.f(), ZERO_FLAG | HALF_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_24_half_carry() {
        let machine = run_test(&[0x24], |machine| {
            *machine.cpu.h_mut() = 0x0F;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.h(), 0x10);
        assert_eq!(machine.cpu.f(), HALF_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_25() {
        let machine = run_test(&[0x25], |machine| {
            *machine.cpu.h_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.h(), 0x41);
        assert_eq!(machine.cpu.f(), SUB_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_25_zero() {
        let machine = run_test(&[0x25], |machine| {
            *machine.cpu.h_mut() = 0x01;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.h(), 0x00);
        assert_eq!(machine.cpu.f(), ZERO_FLAG | SUB_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_25_half_carry() {
        let machine = run_test(&[0x25], |machine| {
            *machine.cpu.h_mut() = 0x00;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.h(), 0xFF);
        assert_eq!(machine.cpu.f(), HALF_FLAG | SUB_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_26() {
        let machine = run_test(&[0x26, 0x42], |_| {});
        assert_eq!(machine.cpu.pc, 0x0002);
        assert_eq!(machine.cpu.h(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_27_1() {
        /*
        From the Game Pack Programming Manual, page 122
        Examples:  When A = 45h and B = 38h,
            ADD   A, B  ;  A ← 7Dh, N ← 0
            DAA         ;  A ←7Dh + 06h (83h), CY ← 0
            SUB   A, B  ;  A ← 83h – 38h (4Bh), N ← 1
            DAA         ;  A ← 4Bh + FAh (45h)
        */
        let machine = run_test(&[0x27], |machine| {
            *machine.cpu.a_mut() = 0x7D;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0x83);
        assert_eq!(machine.cpu.f(), 0);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_27_2() {
        let machine = run_test(&[0x27], |machine| {
            *machine.cpu.a_mut() = 0x4B;
            machine.cpu.set_flag(SUB_FLAG | HALF_FLAG);
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0x45);
        assert_eq!(machine.cpu.f(), SUB_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_28() {
        let machine = run_test(&[0x28, 0x01, 0xFD, 0xFD], |machine| {
            machine.cpu.set_flag(ZERO_FLAG);
        });
        assert_eq!(machine.cpu.pc, 0x0003);
        assert_eq!(machine.cpu.t_cycles, 12);
    }

    #[test]
    fn test_28_negative() {
        let machine = run_test(&[0x00, 0xFD, 0x28, -3i8 as u8], |machine| {
            machine.cpu.pc = 0x0002;
            machine.cpu.set_flag(ZERO_FLAG);
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 12);
    }

    #[test]
    fn test_28_no_jump() {
        let machine = run_test(&[0x28, 0x01, 0xFD, 0x00], |machine| {
            machine.cpu.set_flag(ZERO_FLAG);
            machine.cpu.clear_flag(ZERO_FLAG);
        });
        assert_eq!(machine.cpu.pc, 0x0002);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_29() {
        let machine = run_test(&[0x29], |machine| {
            *machine.cpu.hl.value_mut() = 0x0001;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.hl.value(), 0x0002);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_29_half_carry() {
        let machine = run_test(&[0x29], |machine| {
            *machine.cpu.hl.value_mut() = 0x0FFF;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.hl.value(), 0x1FFE);
        assert_eq!(machine.cpu.f(), HALF_FLAG);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_29_carry() {
        let machine = run_test(&[0x29], |machine| {
            *machine.cpu.hl.value_mut() = 0xF000;
            machine.cpu.set_flag(CARRY_FLAG);
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.hl.value(), 0xE000);
        assert_eq!(machine.cpu.f(), CARRY_FLAG);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_29_gb_manual() {
        let machine = run_test(&[0x29], |machine| {
            *machine.cpu.hl.value_mut() = 0x8A23;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.hl.value(), 0x1446);
        assert_eq!(machine.cpu.f(), CARRY_FLAG | HALF_FLAG);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_2a() {
        let machine = run_test(&[0x2A], |machine| {
            *machine.cpu.hl.value_mut() = 0xC000;
            machine.cpu.mmu.borrow_mut().write(0xC000, 0x42);
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_2b() {
        let machine = run_test(&[0x2B], |machine| {
            *machine.cpu.hl.value_mut() = 0xC001;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.hl.value(), 0xC000);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_2c() {
        let machine = run_test(&[0x2C], |machine| {
            *machine.cpu.l_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.l(), 0x43);
        assert_eq!(machine.cpu.f(), 0);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_2c_zero() {
        let machine = run_test(&[0x2C], |machine| {
            *machine.cpu.l_mut() = 0xFF;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.l(), 0x00);
        assert_eq!(machine.cpu.f(), ZERO_FLAG | HALF_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_2c_half_carry() {
        let machine = run_test(&[0x2C], |machine| {
            *machine.cpu.l_mut() = 0x0F;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.l(), 0x10);
        assert_eq!(machine.cpu.f(), HALF_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_2d() {
        let machine = run_test(&[0x2D], |machine| {
            *machine.cpu.l_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.l(), 0x41);
        assert_eq!(machine.cpu.f(), SUB_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_2d_zero() {
        let machine = run_test(&[0x2D], |machine| {
            *machine.cpu.l_mut() = 0x01;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.l(), 0x00);
        assert_eq!(machine.cpu.f(), ZERO_FLAG | SUB_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_2d_half_carry() {
        let machine = run_test(&[0x2D], |machine| {
            *machine.cpu.l_mut() = 0x00;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.l(), 0xFF);
        assert_eq!(machine.cpu.f(), HALF_FLAG | SUB_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_2e() {
        let machine = run_test(&[0x2E, 0x42], |_| {});
        assert_eq!(machine.cpu.pc, 0x0002);
        assert_eq!(machine.cpu.l(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_2f() {
        let machine = run_test(&[0x2F], |machine| {
            *machine.cpu.a_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0xBD);
        assert_eq!(machine.cpu.f(), HALF_FLAG | SUB_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }
}
