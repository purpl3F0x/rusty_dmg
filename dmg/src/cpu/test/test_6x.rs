#[cfg(test)]
mod test_6x {
    use crate::cpu::test::run_test;

    #[test]
    fn test_60() {
        let machine = run_test(&[0x60], |machine| {
            *machine.cpu.b_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.h(), 0x42);
    }

    #[test]
    fn test_61() {
        let machine = run_test(&[0x61], |machine| {
            *machine.cpu.c_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.h(), 0x42);
    }

    #[test]
    fn test_62() {
        let machine = run_test(&[0x62], |machine| {
            *machine.cpu.d_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.h(), 0x42);
    }
    #[test]
    fn test_63() {
        let machine = run_test(&[0x63], |machine| {
            *machine.cpu.e_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.h(), 0x42);
    }

    #[test]
    fn test_64() {
        let machine = run_test(&[0x64], |machine| {
            *machine.cpu.h_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.h(), 0x42);
    }

    #[test]
    fn test_65() {
        let machine = run_test(&[0x65], |machine| {
            *machine.cpu.l_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.h(), 0x42);
    }

    #[test]
    fn test_66() {
        let machine = run_test(&[0x66], |machine| {
            machine.cpu.mmu.borrow_mut().write(0xC000, 0x42);
            *machine.cpu.hl.value_mut() = 0xC000;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
        assert_eq!(machine.cpu.h(), 0x42);
    }

    #[test]
    fn test_67() {
        let machine = run_test(&[0x67], |machine| {
            *machine.cpu.a_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.h(), 0x42);
    }

    #[test]
    fn test_68() {
        let machine = run_test(&[0x68], |machine| {
            *machine.cpu.b_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.l(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_69() {
        let machine = run_test(&[0x69], |machine| {
            *machine.cpu.c_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.l(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_6a() {
        let machine = run_test(&[0x6A], |machine| {
            *machine.cpu.d_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.l(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_6b() {
        let machine = run_test(&[0x6B], |machine| {
            *machine.cpu.e_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.l(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_6c() {
        let machine = run_test(&[0x6C], |machine| {
            *machine.cpu.h_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.l(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_6d() {
        let machine = run_test(&[0x6D], |machine| {
            *machine.cpu.l_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.l(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_6e() {
        let machine = run_test(&[0x6E], |machine| {
            machine.cpu.mmu.borrow_mut().write(0xC000, 0x42);
            *machine.cpu.hl.value_mut() = 0xC000;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
        assert_eq!(machine.cpu.l(), 0x42);
    }

    #[test]
    fn test_6f() {
        let machine = run_test(&[0x6F], |machine| {
            *machine.cpu.a_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.l(), 0x42);
    }
}
