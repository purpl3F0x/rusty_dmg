#[cfg(test)]
mod test_7x {
    use crate::cpu::test::run_test;
    use crate::cpu::*;

    #[test]
    fn test_70() {
        let machine = run_test(&[0x70], |machine| {
            *machine.cpu.b_mut() = 0x42;
            *machine.cpu.hl.value_mut() = 0xC000;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
        assert_eq!(machine.cpu.mmu.borrow().read(0xC000), 0x42);
    }

    #[test]
    fn test_71() {
        let machine = run_test(&[0x71], |machine| {
            *machine.cpu.c_mut() = 0x42;
            *machine.cpu.hl.value_mut() = 0xC000;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
        assert_eq!(machine.cpu.mmu.borrow().read(0xC000), 0x42);
    }

    #[test]
    fn test_72() {
        let machine = run_test(&[0x72], |machine| {
            *machine.cpu.d_mut() = 0x42;
            *machine.cpu.hl.value_mut() = 0xC000;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
        assert_eq!(machine.cpu.mmu.borrow().read(0xC000), 0x42);
    }
    #[test]
    fn test_73() {
        let machine = run_test(&[0x73], |machine| {
            *machine.cpu.e_mut() = 0x42;
            *machine.cpu.hl.value_mut() = 0xC000;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
        assert_eq!(machine.cpu.mmu.borrow().read(0xC000), 0x42);
    }

    #[test]
    fn test_74() {
        let machine = run_test(&[0x74], |machine| {
            *machine.cpu.hl.value_mut() = 0xC000;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
        assert_eq!(machine.cpu.mmu.borrow().read(0xC000), 0xC0);
    }

    #[test]
    fn test_75() {
        let machine = run_test(&[0x75], |machine| {
            *machine.cpu.hl.value_mut() = 0xC042;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
        assert_eq!(machine.cpu.mmu.borrow().read(0xC042), 0x42);
    }

    #[test]
    fn test_76() {
        let machine = run_test(&[0x76], |_| {});
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.mode, CPUMode::Halt);
    }

    #[test]
    fn test_77() {
        let machine = run_test(&[0x77], |machine| {
            *machine.cpu.a_mut() = 0x42;
            *machine.cpu.hl.value_mut() = 0xC000;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
        assert_eq!(machine.cpu.mmu.borrow().read(0xC000), 0x42);
    }

    #[test]
    fn test_78() {
        let machine = run_test(&[0x78], |machine| {
            *machine.cpu.b_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_79() {
        let machine = run_test(&[0x79], |machine| {
            *machine.cpu.c_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_7a() {
        let machine = run_test(&[0x7A], |machine| {
            *machine.cpu.d_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_7b() {
        let machine = run_test(&[0x7B], |machine| {
            *machine.cpu.e_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_7c() {
        let machine = run_test(&[0x7C], |machine| {
            *machine.cpu.h_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_7d() {
        let machine = run_test(&[0x7D], |machine| {
            *machine.cpu.l_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.a(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_7e() {
        let machine = run_test(&[0x7E], |machine| {
            machine.cpu.mmu.borrow_mut().write(0xC000, 0x42);
            *machine.cpu.hl.value_mut() = 0xC000;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
        assert_eq!(machine.cpu.a(), 0x42);
    }

    #[test]
    fn test_7f() {
        let machine = run_test(&[0x7F], |machine| {
            *machine.cpu.a_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.a(), 0x42);
    }
}
