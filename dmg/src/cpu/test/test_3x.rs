#[cfg(test)]
mod test_3x {
    use crate::cpu::test::run_test;
    use crate::cpu::*;

    #[test]
    fn test_30() {
        let machine = run_test(&[0x30, 0x01, 0xFD, 0xFD], |_| {});
        assert_eq!(machine.cpu.pc, 0x0003);
        assert_eq!(machine.cpu.t_cycles, 12);
    }

    #[test]
    fn test_30_negative() {
        let machine = run_test(&[0x00, 0xFD, 0x30, -3i8 as u8], |machine| {
            machine.cpu.pc = 0x0002;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 12);
    }

    #[test]
    fn test_30_no_jump() {
        let machine = run_test(&[0x30, 0x01, 0xFD, 0xFD], |machine| {
            machine.cpu.set_flag(CARRY_FLAG);
        });
        assert_eq!(machine.cpu.pc, 0x0002);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_31() {
        let machine = run_test(&[0x31, 0x34, 0x12], |_| {});
        assert_eq!(machine.cpu.pc, 0x0003);
        assert_eq!(machine.cpu.sp, 0x1234);
        assert_eq!(machine.cpu.t_cycles, 12);
    }

    #[test]
    fn test_32() {
        let machine = run_test(&[0x32], |machine| {
            *machine.cpu.a_mut() = 0x42;
            *machine.cpu.hl.value_mut() = 0xC000;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.mmu.borrow().read(0xC000), 0x42);
        assert_eq!(machine.cpu.hl.value(), 0xC000 - 1);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_33() {
        let machine = run_test(&[0x33], |machine| {
            machine.cpu.sp = 0x4217;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.sp, 0x4218);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_34() {
        let machine = run_test(&[0x34], |machine| {
            machine.cpu.mmu.borrow_mut().write(0xC000, 0x42);
            *machine.cpu.hl.value_mut() = 0xC000;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.mmu.borrow().read(0xC000), 0x43);
        assert_eq!(machine.cpu.t_cycles, 12);
    }
    #[test]
    fn test_34_zero() {
        let machine = run_test(&[0x34], |machine| {
            machine.cpu.mmu.borrow_mut().write(0xC000, 0xFF);
            *machine.cpu.hl.value_mut() = 0xC000;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.mmu.borrow().read(0xC000), 0x00);
        assert_eq!(machine.cpu.f(), ZERO_FLAG | HALF_FLAG);
        assert_eq!(machine.cpu.t_cycles, 12);
    }

    #[test]
    fn test_34_half_carry() {
        let machine = run_test(&[0x34], |machine| {
            machine.cpu.mmu.borrow_mut().write(0xC000, 0x0F);
            *machine.cpu.hl.value_mut() = 0xC000;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.mmu.borrow().read(0xC000), 0x10);
        assert_eq!(machine.cpu.f(), HALF_FLAG);
        assert_eq!(machine.cpu.t_cycles, 12);
    }

    #[test]
    fn test_35() {
        let machine = run_test(&[0x35], |machine| {
            machine.cpu.mmu.borrow_mut().write(0xC000, 0x42);
            *machine.cpu.hl.value_mut() = 0xC000;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.mmu.borrow().read(0xC000), 0x41);
        assert_eq!(machine.cpu.f(), SUB_FLAG);
        assert_eq!(machine.cpu.t_cycles, 12);
    }

    #[test]
    fn test_35_zero() {
        let machine = run_test(&[0x35], |machine| {
            machine.cpu.mmu.borrow_mut().write(0xC000, 0x01);
            *machine.cpu.hl.value_mut() = 0xC000;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.mmu.borrow().read(0xC000), 0x00);
        assert_eq!(machine.cpu.f(), ZERO_FLAG | SUB_FLAG);
        assert_eq!(machine.cpu.t_cycles, 12);
    }

    #[test]
    fn test_35_half_carry() {
        let machine = run_test(&[0x35], |machine| {
            machine.cpu.mmu.borrow_mut().write(0xC000, 0x00);
            *machine.cpu.hl.value_mut() = 0xC000;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.mmu.borrow().read(0xC000), 0xFF);
        assert_eq!(machine.cpu.f(), HALF_FLAG | SUB_FLAG);
        assert_eq!(machine.cpu.t_cycles, 12);
    }

    #[test]
    fn test_36() {
        let machine = run_test(&[0x36, 0x42], |machine| {
            *machine.cpu.hl.value_mut() = 0xC000;
        });
        assert_eq!(machine.cpu.pc, 0x0002);
        assert_eq!(machine.cpu.mmu.borrow().read(0xC000), 0x42);
        assert_eq!(machine.cpu.t_cycles, 12);
    }

    #[test]
    fn test_37() {
        let machine = run_test(&[0x37], |machine| {
            *machine.cpu.a_mut() = 0x42;
            machine.cpu.set_flag(SUB_FLAG | HALF_FLAG);
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0x42);
        assert_eq!(machine.cpu.f(), CARRY_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_38() {
        let machine = run_test(&[0x38, 0x01, 0xFD, 0xFD], |machine| {
            *machine.cpu.a_mut() = 0x42;
            machine.cpu.set_flag(CARRY_FLAG);
        });
        assert_eq!(machine.cpu.pc, 0x0003);
        assert_eq!(machine.cpu.t_cycles, 12);
    }

    #[test]
    fn test_38_negative() {
        let machine = run_test(&[0x00, 0xFD, 0x38, -3i8 as u8], |machine| {
            machine.cpu.pc = 0x0002;
            machine.cpu.set_flag(CARRY_FLAG);
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 12);
    }

    #[test]
    fn test_38_no_jump() {
        let machine = run_test(&[0x38, 0x01, 0xFD, 0xFD], |machine| {
            *machine.cpu.a_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0002);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_39() {
        let machine = run_test(&[0x39], |machine| {
            *machine.cpu.hl.value_mut() = 0x0101;
            machine.cpu.sp = 0x0101;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.hl.value(), 0x0202);
        assert_eq!(machine.cpu.f(), 0);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_3a() {
        let machine = run_test(&[0x3A], |machine| {
            *machine.cpu.hl.value_mut() = 0xC000;
            machine.cpu.mmu.borrow_mut().write(0xC000, 0x42);
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_3b() {
        let machine = run_test(&[0x3B], |machine| {
            machine.cpu.sp = 0xC000;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.sp, 0xC000 - 1);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_3c() {
        let machine = run_test(&[0x3C], |machine| {
            *machine.cpu.a_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0x43);
        assert_eq!(machine.cpu.f(), 0);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_3c_zero() {
        let machine = run_test(&[0x3C], |machine| {
            *machine.cpu.a_mut() = 0xFF;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0x00);
        assert_eq!(machine.cpu.f(), ZERO_FLAG | HALF_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_3c_half_carry() {
        let machine = run_test(&[0x3C], |machine| {
            *machine.cpu.a_mut() = 0x0F;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0x10);
        assert_eq!(machine.cpu.f(), HALF_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_3d() {
        let machine = run_test(&[0x3D], |machine| {
            *machine.cpu.a_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0x41);
        assert_eq!(machine.cpu.f(), SUB_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_3d_zero() {
        let machine = run_test(&[0x3D], |machine| {
            *machine.cpu.a_mut() = 0x01;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0x00);
        assert_eq!(machine.cpu.f(), ZERO_FLAG | SUB_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_3d_half_carry() {
        let machine = run_test(&[0x3D], |machine| {
            *machine.cpu.a_mut() = 0x00;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0xFF);
        assert_eq!(machine.cpu.f(), HALF_FLAG | SUB_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_3e() {
        let machine = run_test(&[0x3E, 0x42], |_| {});
        assert_eq!(machine.cpu.pc, 0x0002);
        assert_eq!(machine.cpu.a(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 8);
    }

    #[test]
    fn test_3f() {
        let machine = run_test(&[0x3F], |machine| {
            *machine.cpu.a_mut() = 0x42;
            machine.cpu.set_flag(SUB_FLAG | HALF_FLAG);
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0x42);
        assert_eq!(machine.cpu.f(), CARRY_FLAG);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_3f_carry() {
        let machine = run_test(&[0x3F], |machine| {
            *machine.cpu.a_mut() = 0x42;
            machine.cpu.set_flag(SUB_FLAG | HALF_FLAG | CARRY_FLAG);
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0x42);
        assert_eq!(machine.cpu.f(), 0);
        assert_eq!(machine.cpu.t_cycles, 4);
    }
}
