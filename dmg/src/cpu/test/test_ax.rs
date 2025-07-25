#[cfg(test)]
mod test_ax {
    use crate::cpu::test::run_test;
    use crate::cpu::*;

    use paste::paste;

    macro_rules! test_and {
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
                        assert_eq!(machine.cpu.a(), 0x02);
                        assert_eq!(machine.cpu.f(), HALF_FLAG);
                    }

                    #[test]
                    fn [<test_ $instruction _zero>]() {
                        let machine = run_test(&[[<0 x $instruction>]], |machine| {
                            *machine.cpu.a_mut() = 0x42;
                            *machine.cpu.[<$rhs _mut>]() = 0xBD;
                        });

                        assert_eq!(machine.cpu.pc, 0x0001);
                        assert_eq!(machine.cpu.t_cycles, 4);
                        assert_eq!(machine.cpu.a(), 0x00);
                        assert_eq!(machine.cpu.f(), HALF_FLAG | ZERO_FLAG);
                    }


            }
        };
    }

    macro_rules! test_xor {
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
                        assert_eq!(machine.cpu.a(), 0x55);
                        assert_eq!(machine.cpu.f(), 0);
                    }

                    #[test]
                    fn [<test_ $instruction _zero>]() {
                        let machine = run_test(&[[<0 x $instruction>]], |machine| {
                            *machine.cpu.a_mut() = 0x42;
                            *machine.cpu.[<$rhs _mut>]() = 0x42;
                        });

                        assert_eq!(machine.cpu.pc, 0x0001);
                        assert_eq!(machine.cpu.t_cycles, 4);
                        assert_eq!(machine.cpu.a(), 0x00);
                        assert_eq!(machine.cpu.f(), ZERO_FLAG);
                    }


            }
        };
    }

    // *Test AND*

    test_and!(a0, b);
    test_and!(a1, c);
    test_and!(a2, d);
    test_and!(a3, e);
    test_and!(a4, h);
    test_and!(a5, l);

    // * Test XOR *
    test_xor!(a8, b);
    test_xor!(a9, c);
    test_xor!(aa, d);
    test_xor!(ab, e);
    test_xor!(ac, h);
    test_xor!(ad, l);

    #[test]
    fn test_a6() {
        let machine = run_test(&[0xA6], |machine| {
            *machine.cpu.a_mut() = 0x42;
            *machine.cpu.hl.value_mut() = 0xC000;
            machine.cpu.mmu.borrow_mut().write(0xC000, 0x17);
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
        assert_eq!(machine.cpu.a(), 0x02);
        assert_eq!(machine.cpu.f(), HALF_FLAG);
    }

        #[test]
    fn test_a6_zero() {
        let machine = run_test(&[0xA6], |machine| {
            *machine.cpu.a_mut() = 0x42;
            *machine.cpu.hl.value_mut() = 0xC000;
            machine.cpu.mmu.borrow_mut().write(0xC000, 0xBD);
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
        assert_eq!(machine.cpu.a(), 0x00);
        assert_eq!(machine.cpu.f(), HALF_FLAG | ZERO_FLAG);
    }

    #[test]
    fn test_a7() {
        let machine = run_test(&[0xA7], |machine| {
            *machine.cpu.a_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.a(), 0x42);
        assert_eq!(machine.cpu.f(), HALF_FLAG);
    }

    #[test]
    fn test_ae() {
        let machine = run_test(&[0xAE], |machine| {
            *machine.cpu.a_mut() = 0x42;
            *machine.cpu.hl.value_mut() = 0xC000;
            machine.cpu.mmu.borrow_mut().write(0xC000, 0x17);
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
        assert_eq!(machine.cpu.a(), 0x55);
        assert_eq!(machine.cpu.f(), 0);
    }

    #[test]
    fn test_ae_zero() {
        let machine = run_test(&[0xAE], |machine| {
            *machine.cpu.a_mut() = 0x42;
            *machine.cpu.hl.value_mut() = 0xC000;
            machine.cpu.mmu.borrow_mut().write(0xC000, 0x42);
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
        assert_eq!(machine.cpu.a(), 0x00);
        assert_eq!(machine.cpu.f(), ZERO_FLAG);
    }

    #[test]
    fn test_af() {
        let machine = run_test(&[0xAF], |machine| {
            *machine.cpu.a_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.a(), 0x00);
        assert_eq!(machine.cpu.f(), ZERO_FLAG);
    }
}
