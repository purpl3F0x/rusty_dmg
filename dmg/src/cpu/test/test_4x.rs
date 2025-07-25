#[cfg(test)]
mod test_4x {
    use crate::cpu::test::run_test;

    #[test]
    fn test_40() {
        let machine = run_test(&[0x40], |machine| {
            *machine.cpu.b_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.b(), 0x42);
    }

    #[test]
    fn test_41() {
        let machine = run_test(&[0x41], |machine| {
            *machine.cpu.c_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.b(), 0x42);
    }

    #[test]
    fn test_42() {
        let machine = run_test(&[0x42], |machine| {
            *machine.cpu.d_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.b(), 0x42);
    }
    #[test]
    fn test_43() {
        let machine = run_test(&[0x43], |machine| {
            *machine.cpu.e_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.b(), 0x42);
    }

    #[test]
    fn test_44() {
        let machine = run_test(&[0x44], |machine| {
            *machine.cpu.h_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.b(), 0x42);
    }

    #[test]
    fn test_45() {
        let machine = run_test(&[0x45], |machine| {
            *machine.cpu.l_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.b(), 0x42);
    }

    #[test]
    fn test_46() {
        let machine = run_test(&[0x46], |machine| {
            machine.cpu.mmu.borrow_mut().write(0xC000, 0x42);
            *machine.cpu.hl.value_mut() = 0xC000;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
        assert_eq!(machine.cpu.b(), 0x42);
    }

    #[test]
    fn test_47() {
        let machine = run_test(&[0x47], |machine| {
            *machine.cpu.a_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.b(), 0x42);
    }

    #[test]
    fn test_48() {
        let machine = run_test(&[0x48], |machine| {
            *machine.cpu.b_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.c(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_49() {
        let machine = run_test(&[0x49], |machine| {
            *machine.cpu.c_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.c(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_4a() {
        let machine = run_test(&[0x4A], |machine| {
            *machine.cpu.d_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.c(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_4b() {
        let machine = run_test(&[0x4B], |machine| {
            *machine.cpu.e_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.c(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_4c() {
        let machine = run_test(&[0x4C], |machine| {
            *machine.cpu.h_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.c(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_4d() {
        let machine = run_test(&[0x4D], |machine| {
            *machine.cpu.l_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.c(), 0x42);
        assert_eq!(machine.cpu.t_cycles, 4);
    }

    #[test]
    fn test_4e() {
        let machine = run_test(&[0x4E], |machine| {
            machine.cpu.mmu.borrow_mut().write(0xC000, 0x42);
            *machine.cpu.hl.value_mut() = 0xC000;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
        assert_eq!(machine.cpu.c(), 0x42);
    }

    #[test]
    fn test_4f() {
        let machine = run_test(&[0x4F], |machine| {
            *machine.cpu.a_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.c(), 0x42);
    }
}
