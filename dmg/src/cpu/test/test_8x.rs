#[cfg(test)]
mod test_8x {
    use crate::cpu::test::run_test;
    use crate::cpu::*;

    use paste::paste;

    macro_rules! test_add {
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
                    assert_eq!(machine.cpu.a(), 0x43);
                    assert_eq!(machine.cpu.f(), 0);
                }

                #[test]
                fn [<test_ $instruction _half_carry>]() {
                    let machine = run_test(&[[<0 x $instruction>]], |machine| {
                        *machine.cpu.a_mut() = 0x42;
                        *machine.cpu.[<$rhs _mut>]() = 0x0F;
                    });

                    assert_eq!(machine.cpu.pc, 0x0001);
                    assert_eq!(machine.cpu.t_cycles, 4);
                    assert_eq!(machine.cpu.a(), 0x51);
                    assert_eq!(machine.cpu.f(), HALF_FLAG);
                    }

                #[test]
                fn [<test_ $instruction _carry>]() {
                   let machine = run_test(&[[<0 x $instruction>]], |machine| {
                       *machine.cpu.a_mut() = 0xF0;
                       *machine.cpu.[<$rhs _mut>]() = 0x11;
                    });

                    assert_eq!(machine.cpu.pc, 0x0001);
                    assert_eq!(machine.cpu.t_cycles, 4);
                    assert_eq!(machine.cpu.a(), 0x01);
                    assert_eq!(machine.cpu.f(), CARRY_FLAG);
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

    macro_rules! test_adc {
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
                    assert_eq!(machine.cpu.a(), 0x44);
                    assert_eq!(machine.cpu.f(), 0);
                }

                #[test]
                fn [<test_ $instruction _without_carry>]() {
                    let machine = run_test(&[[<0 x $instruction>]], |machine| {
                        *machine.cpu.a_mut() = 0x42;
                        *machine.cpu.[<$rhs _mut>]() = 0x01;
                    });

                    assert_eq!(machine.cpu.pc, 0x0001);
                    assert_eq!(machine.cpu.t_cycles, 4);
                    assert_eq!(machine.cpu.a(), 0x43);
                    assert_eq!(machine.cpu.f(), 0);
                }

            }
        };
    }

    // *Test ADD*

    test_add!(80, b);
    test_add!(81, c);
    test_add!(82, d);
    test_add!(83, e);
    test_add!(84, h);
    test_add!(85, l);

    // *Test ADC*

    test_adc!(88, b);
    test_adc!(89, c);
    test_adc!(8a, d);
    test_adc!(8b, e);
    test_adc!(8c, h);
    test_adc!(8d, l);

    #[test]
    fn test_86() {
        let machine = run_test(&[0x86], |machine| {
            *machine.cpu.hl.value_mut() = 0xC000;
            machine.cpu.mmu.borrow_mut().write(0xC000, 0x01);
            *machine.cpu.a_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
        assert_eq!(machine.cpu.a(), 0x43);
        assert_eq!(machine.cpu.f(), 0);
    }

    #[test]
    fn test_87() {
        let machine = run_test(&[0x87], |machine| {
            *machine.cpu.a_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.a(), 0x84);
        assert_eq!(machine.cpu.f(), 0);
    }

    #[test]
    fn test_8e_with_carry() {
        let machine = run_test(&[0x8E], |machine| {
            *machine.cpu.hl.value_mut() = 0xC000;
            machine.cpu.mmu.borrow_mut().write(0xC000, 0x01);
            *machine.cpu.a_mut() = 0x42;
            machine.cpu.set_flag(CARRY_FLAG);
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
        assert_eq!(machine.cpu.a(), 0x44);
        assert_eq!(machine.cpu.f(), 0);
    }

    #[test]
    fn test_8e_without_carry() {
        let machine = run_test(&[0x8E], |machine| {
            *machine.cpu.hl.value_mut() = 0xC000;
            machine.cpu.mmu.borrow_mut().write(0xC000, 0x01);
            *machine.cpu.a_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 8);
        assert_eq!(machine.cpu.a(), 0x43);
        assert_eq!(machine.cpu.f(), 0);
    }

    #[test]
    fn test_8f_with_carry() {
        let machine = run_test(&[0x8f], |machine| {
            *machine.cpu.a_mut() = 0x42;
            machine.cpu.set_flag(CARRY_FLAG);
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.a(), 0x85);
        assert_eq!(machine.cpu.f(), 0);
    }

    #[test]
    fn test_8f_without_carry() {
        let machine = run_test(&[0x8f], |machine| {
            *machine.cpu.a_mut() = 0x42;
        });
        assert_eq!(machine.cpu.pc, 0x0001);
        assert_eq!(machine.cpu.t_cycles, 4);
        assert_eq!(machine.cpu.a(), 0x84);
        assert_eq!(machine.cpu.f(), 0);
    }

    

}
