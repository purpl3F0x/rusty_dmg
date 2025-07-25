#[cfg(test)]
mod test_9x {
    use crate::cpu::test::run_test;
    use crate::cpu::*;

    use paste::paste;

    macro_rules! test_sub {
        ($instruction:expr, $rhs:expr) => {
            paste! {
                #[test]
                fn [<test_ $instruction>]() {
                    let machine = run_test(&[[<0 x $instruction>]], |machine| {
                        *machine.cpu.a_mut() = 0x42;
                        *machine.cpu.[<$rhs _mut>]() = 0x01;
                    });

                    assert_eq!(machine.cpu.pc, 0x0001);
                    assert_eq!(machine.cpu.t_cycles, 4);
                    assert_eq!(machine.cpu.a(), 0x41);
                    assert_eq!(machine.cpu.f(), SUB_FLAG);
                }

                #[test]
                fn [<test_ $instruction _half_carry>]() {
                    let machine = run_test(&[[<0 x $instruction>]], |machine| {
                        *machine.cpu.a_mut() = 0x15;
                        *machine.cpu.[<$rhs _mut>]() = 0x06;
                    });

                    assert_eq!(machine.cpu.pc, 0x0001);
                    assert_eq!(machine.cpu.t_cycles, 4);
                    assert_eq!(machine.cpu.a(), 0x0F);
                    assert_eq!(machine.cpu.f(), HALF_FLAG | SUB_FLAG);
                    }

                #[test]
                fn [<test_ $instruction _carry>]() {
                   let machine = run_test(&[[<0 x $instruction>]], |machine| {
                       *machine.cpu.a_mut() = 0x05;
                       *machine.cpu.[<$rhs _mut>]() = 0x10;
                    });

                    assert_eq!(machine.cpu.pc, 0x0001);
                    assert_eq!(machine.cpu.t_cycles, 4);
                    assert_eq!(machine.cpu.a(), 0xF5);
                    assert_eq!(machine.cpu.f(), CARRY_FLAG | SUB_FLAG);
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
                    assert_eq!(machine.cpu.f(), ZERO_FLAG | SUB_FLAG);
                }

            }
        };
    }

    macro_rules! test_sbc {
        ($instruction:expr, $rhs:expr) => {
            paste! {
                #[test]
                fn [<test_ $instruction _with_carry>]() {
                    let machine = run_test(&[[<0 x $instruction>]], |machine| {
                        *machine.cpu.a_mut() = 0x42;
                        *machine.cpu.[<$rhs _mut>]() = 0x01;
                        machine.cpu.set_flag(CARRY_FLAG);
                    });

                    assert_eq!(machine.cpu.pc, 0x0001);
                    assert_eq!(machine.cpu.t_cycles, 4);
                    assert_eq!(machine.cpu.a(), 0x40);
                    assert_eq!(machine.cpu.f(), SUB_FLAG);
                }

                #[test]
                fn [<test_ $instruction _without_carry>]() {
                    let machine = run_test(&[[<0 x $instruction>]], |machine| {
                        *machine.cpu.a_mut() = 0x42;
                        *machine.cpu.[<$rhs _mut>]() = 0x01;
                    });

                    assert_eq!(machine.cpu.pc, 0x0001);
                    assert_eq!(machine.cpu.t_cycles, 4);
                    assert_eq!(machine.cpu.a(), 0x41);
                    assert_eq!(machine.cpu.f(), SUB_FLAG);
                }

            }
        };
    }

    // *Test SUB*

    test_sub!(90, b);
    test_sub!(91, c);
    test_sub!(92, d);
    test_sub!(93, e);
    test_sub!(94, h);
    test_sub!(95, l);

    // *Test SBC*

    test_sbc!(98, b);
    test_sbc!(99, c);
    test_sbc!(9a, d);
    test_sbc!(9b, e);
    test_sbc!(9c, h);
    test_sbc!(9d, l);

    #[test]
    fn test_96() {
        let machine = run_test(&[0x96], |machine| {
            *machine.cpu.hl.value_mut() = 0xC000;
            machine.cpu.mmu.borrow_mut().write(0xC000, 0x01);
            *machine.cpu.a_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
        assert_eq!(machine.cpu.a(), 0x41);
        assert_eq!(machine.cpu.f(), SUB_FLAG);
    }

    #[test]
    fn test_97() {
        let machine = run_test(&[0x97], |machine| {
            *machine.cpu.a_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.a(), 0);
        assert_eq!(machine.cpu.f(), ZERO_FLAG | SUB_FLAG);
    }

    #[test]
    fn test_9e_with_carry() {
        let machine = run_test(&[0x9E], |machine| {
            *machine.cpu.hl.value_mut() = 0xC000;
            machine.cpu.mmu.borrow_mut().write(0xC000, 0x01);
            *machine.cpu.a_mut() = 0x42;
            machine.cpu.set_flag(CARRY_FLAG);
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
        assert_eq!(machine.cpu.a(), 0x40);
        assert_eq!(machine.cpu.f(), SUB_FLAG);
    }

    #[test]
    fn test_9e_without_carry() {
        let machine = run_test(&[0x9E], |machine| {
            *machine.cpu.hl.value_mut() = 0xC000;
            machine.cpu.mmu.borrow_mut().write(0xC000, 0x01);
            *machine.cpu.a_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
        assert_eq!(machine.cpu.a(), 0x41);
        assert_eq!(machine.cpu.f(), SUB_FLAG);
    }

    #[test]
    fn test_9f_with_carry() {
        let machine = run_test(&[0x9F], |machine| {
            *machine.cpu.a_mut() = 0x42;
            machine.cpu.set_flag(CARRY_FLAG);
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.a(), -1i8 as u8);
        assert_eq!(machine.cpu.f(), SUB_FLAG | HALF_FLAG | CARRY_FLAG);
    }

    #[test]
    fn test_9f_without_carry() {
        let machine = run_test(&[0x9F], |machine| {
            *machine.cpu.a_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.a(), 0);
        assert_eq!(machine.cpu.f(), SUB_FLAG | ZERO_FLAG);
    }
}
