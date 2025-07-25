#[cfg(test)]
mod test_5x {
    use crate::cpu::test::run_test;

    #[test]
    fn test_50() {
        let machine = run_test(&[0x50], |machine| {
            *machine.cpu.b_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.d(), 0x42);
    }

    #[test]
    fn test_51() {
        let machine = run_test(&[0x51], |machine| {
            *machine.cpu.c_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.d(), 0x42);
    }

    #[test]
    fn test_52() {
        let machine = run_test(&[0x52], |machine| {
            *machine.cpu.d_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.d(), 0x42);
    }
    #[test]
    fn test_53() {
        let machine = run_test(&[0x53], |machine| {
            *machine.cpu.e_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.d(), 0x42);
    }

    #[test]
    fn test_54() {
        let machine = run_test(&[0x54], |machine| {
            *machine.cpu.h_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.d(), 0x42);
    }

    #[test]
    fn test_55() {
        let machine = run_test(&[0x55], |machine| {
            *machine.cpu.l_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.d(), 0x42);
    }

    #[test]
    fn test_56() {
        let machine = run_test(&[0x56], |machine| {
            machine.cpu.mmu.borrow_mut().write(0xC000, 0x42);
            *machine.cpu.hl.value_mut() = 0xC000;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
        assert_eq!(machine.cpu.d(), 0x42);
    }

    #[test]
    fn test_57() {
        let machine = run_test(&[0x57], |machine| {
            *machine.cpu.a_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.d(), 0x42);
    }

    #[test]
    fn test_58() {
        let machine = run_test(&[0x58], |machine| {
            *machine.cpu.b_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.e(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_59() {
        let machine = run_test(&[0x59], |machine| {
            *machine.cpu.c_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.e(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_5a() {
        let machine = run_test(&[0x5A], |machine| {
            *machine.cpu.d_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.e(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_5b() {
        let machine = run_test(&[0x5B], |machine| {
            *machine.cpu.e_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.e(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_5c() {
        let machine = run_test(&[0x5C], |machine| {
            *machine.cpu.h_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.e(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_5d() {
        let machine = run_test(&[0x5D], |machine| {
            *machine.cpu.l_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.e(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_5e() {
        let machine = run_test(&[0x5E], |machine| {
            machine.cpu.mmu.borrow_mut().write(0xC000, 0x42);
            *machine.cpu.hl.value_mut() = 0xC000;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
        assert_eq!(machine.cpu.e(), 0x42);
    }

    #[test]
    fn test_5f() {
        let machine = run_test(&[0x5F], |machine| {
            *machine.cpu.a_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.e(), 0x42);
    }
}
