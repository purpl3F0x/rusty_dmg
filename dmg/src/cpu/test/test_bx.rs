#[cfg(test)]
mod test_bx {
    use crate::cpu::test::run_test;
    use crate::cpu::*;

    use paste::paste;

    macro_rules! test_or {
        ($instruction:expr, $rhs:expr) => {
            paste! {
                    #[test]
                    fn [<test_ $instruction>]() {
                        let machine = run_test(&[[<0 x $instruction>]], |machine| {
                            *machine.cpu.a_mut() = 0x42;
                            *machine.cpu.[<$rhs _mut>]() = 0x17;
                        });

                        assert_eq!(machine.cpu.pc, 0x0001);
                        assert_eq!(machine.cpu.t_cycles, 4);
                        assert_eq!(machine.cpu.a(), 0x57);
                        assert_eq!(machine.cpu.f(), 0);
                    }

                    #[test]
                    fn [<test_ $instruction _zero>]() {
                        let machine = run_test(&[[<0 x $instruction>]], |machine| {
                            *machine.cpu.a_mut() = 0x00;
                            *machine.cpu.[<$rhs _mut>]() = 0x00;
                        });

                        assert_eq!(machine.cpu.pc, 0x0001);
                        assert_eq!(machine.cpu.t_cycles, 4);
                        assert_eq!(machine.cpu.a(), 0x00);
                        assert_eq!(machine.cpu.f(), ZERO_FLAG);
                    }


            }
        };
    }

    macro_rules! test_cp {
        ($instruction:expr, $rhs:expr) => {
            paste! {
                    #[test]
                    fn [<test_ $instruction>]() {
                        let machine = run_test(&[[<0 x $instruction>]], |machine| {
                            *machine.cpu.a_mut() = 0x42;
                            *machine.cpu.[<$rhs _mut>]() = 0x17;
                        });

                        assert_eq!(machine.cpu.pc, 0x0001);
                        assert_eq!(machine.cpu.t_cycles, 4);
                        assert_eq!(machine.cpu.a(), 0x42);
                        assert_eq!(machine.cpu.f(), SUB_FLAG | HALF_FLAG);
                    }

                    #[test]
                    fn [<test_ $instruction _zero>]() {
                        let machine = run_test(&[[<0 x $instruction>]], |machine| {
                            *machine.cpu.a_mut() = 0x42;
                            *machine.cpu.[<$rhs _mut>]() = 0x42;
                        });

                        assert_eq!(machine.cpu.pc, 0x0001);
                        assert_eq!(machine.cpu.t_cycles, 4);
                        assert_eq!(machine.cpu.a(), 0x42);
                        assert_eq!(machine.cpu.f(), SUB_FLAG | ZERO_FLAG);
                    }


            }
        };
    }

    // *Test OR*

    test_or!(b0, b);
    test_or!(b1, c);
    test_or!(b2, d);
    test_or!(b3, e);
    test_or!(b4, h);
    test_or!(b5, l);

    // * Test CP *
    test_cp!(b8, b);
    test_cp!(b9, c);
    test_cp!(ba, d);
    test_cp!(bb, e);
    test_cp!(bc, h);
    test_cp!(bd, l);

    #[test]
    fn test_b6() {
        let machine = run_test(&[0xB6], |machine| {
            *machine.cpu.a_mut() = 0x42;
            *machine.cpu.hl.value_mut() = 0xC000;
            machine.cpu.mmu.borrow_mut().write(0xC000, 0x17);
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
        assert_eq!(machine.cpu.a(), 0x57);
        assert_eq!(machine.cpu.f(), 0);
    }

    #[test]
    fn test_b6_zero() {
        let machine = run_test(&[0xB6], |machine| {
            *machine.cpu.a_mut() = 0x00;
            *machine.cpu.hl.value_mut() = 0xC000;
            machine.cpu.mmu.borrow_mut().write(0xC000, 0x00);
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
        assert_eq!(machine.cpu.a(), 0x00);
        assert_eq!(machine.cpu.f(), ZERO_FLAG);
    }

    #[test]
    fn test_b7() {
        let machine = run_test(&[0xB7], |machine| {
            *machine.cpu.a_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.a(), 0x42);
        assert_eq!(machine.cpu.f(), 0);
    }

    #[test]
    fn test_be() {
        let machine = run_test(&[0xBE], |machine| {
            *machine.cpu.a_mut() = 0x42;
            *machine.cpu.hl.value_mut() = 0xC000;
            machine.cpu.mmu.borrow_mut().write(0xC000, 0x17);
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
        assert_eq!(machine.cpu.a(), 0x42);
        assert_eq!(machine.cpu.f(), SUB_FLAG | HALF_FLAG);
    }

    #[test]
    fn test_be_zero() {
        let machine = run_test(&[0xBE], |machine| {
            *machine.cpu.a_mut() = 0x42;
            *machine.cpu.hl.value_mut() = 0xC000;
            machine.cpu.mmu.borrow_mut().write(0xC000, 0x42);
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
        assert_eq!(machine.cpu.a(), 0x42);
        assert_eq!(machine.cpu.f(), ZERO_FLAG | SUB_FLAG);
    }

    #[test]
    fn test_bf() {
        let machine = run_test(&[0xBF], |machine| {
            *machine.cpu.a_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.a(), 0x42);
        assert_eq!(machine.cpu.f(), ZERO_FLAG | SUB_FLAG);
    }
}
