#[cfg(test)]
mod test_cb {
    use crate::cpu::test::run_test;
    use crate::cpu::*;

    use paste::paste;

    macro_rules! read_register {
        ($machine:expr, $reg:expr) => {
            match $reg {
                Register8::A => $machine.cpu.a(),
                Register8::B => $machine.cpu.b(),
                Register8::C => $machine.cpu.c(),
                Register8::D => $machine.cpu.d(),
                Register8::E => $machine.cpu.e(),
                Register8::H => $machine.cpu.h(),
                Register8::L => $machine.cpu.l(),
                Register8::IndirectHL => $machine.cpu.mmu.borrow_mut().read(0xC000),
            }
        };
        () => {};
    }

    macro_rules! write_register {
        ($machine:expr, $reg:expr, $value:expr) => {
            match $reg {
                Register8::A => *$machine.cpu.a_mut() = $value,
                Register8::B => *$machine.cpu.b_mut() = $value,
                Register8::C => *$machine.cpu.c_mut() = $value,
                Register8::D => *$machine.cpu.d_mut() = $value,
                Register8::E => *$machine.cpu.e_mut() = $value,
                Register8::H => *$machine.cpu.h_mut() = $value,
                Register8::L => *$machine.cpu.l_mut() = $value,
                Register8::IndirectHL => {
                    $machine.cpu.mmu.borrow_mut().write(0xC000, $value);
                    *$machine.cpu.hl.value_mut() = 0xC000;
                }
            }
        };
        () => {};
    }

    macro_rules! test_rrc {
        ($instruction:expr, $reg:expr) => {
            paste! {
                    #[test]
                    fn [<test_ $instruction>]() {
                        let machine = run_test(&[0xCB, [< $instruction>]], |machine| {
                            write_register!(machine, $reg, 0x42);
                        });

                        assert_eq!(machine.cpu.pc, 0x0002);
                        assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 16 });
                        assert_eq!(read_register!(machine, $reg), 0x21);
                        assert_eq!(machine.cpu.f(), 0);
                    }

                    #[test]
                    fn [<test_ $instruction _zero>]() {
                        let machine = run_test(&[0xCB, [<$instruction>]], |machine| {
                            write_register!(machine, $reg, 0x00);
                        });

                        assert_eq!(machine.cpu.pc, 0x0002);
                        assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 16 });
                        assert_eq!(read_register!(machine, $reg), 0x00);
                        assert_eq!(machine.cpu.f(), ZERO_FLAG);
                    }

                    #[test]
                    fn [<test_ $instruction _carry>]() {
                        let machine = run_test(&[0xCB, [< $instruction>]], |machine| {
                            write_register!(machine, $reg, 0x01);
                        });

                        assert_eq!(machine.cpu.pc, 0x0002);
                        assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 16 });
                        assert_eq!(read_register!(machine, $reg), 0x80);
                        assert_eq!(machine.cpu.f(), CARRY_FLAG);
                    }
            }
        };
    }

    macro_rules! test_rlc {
        ($instruction:expr, $reg:expr) => {
            paste! {
                    #[test]
                    fn [<test_ $instruction>]() {
                        let machine = run_test(&[0xCB, [<$instruction>]], |machine| {
                            write_register!(machine, $reg, 0x42);
                        });

                        assert_eq!(machine.cpu.pc, 0x0002);
                        assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 16 });
                        assert_eq!(read_register!(machine, $reg), 0x84);
                        assert_eq!(machine.cpu.f(), 0);
                    }

                    #[test]
                    fn [<test_ $instruction _zero>]() {
                        let machine = run_test(&[0xCB, [< $instruction>]], |machine| {
                            write_register!(machine, $reg, 0x00);
                        });

                        assert_eq!(machine.cpu.pc, 0x0002);
                        assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 16 });
                        assert_eq!(read_register!(machine, $reg), 0x00);
                        assert_eq!(machine.cpu.f(), ZERO_FLAG);
                    }

                    #[test]
                    fn [<test_ $instruction _carry>]() {
                        let machine = run_test(&[0xCB, [< $instruction>]], |machine| {
                            write_register!(machine, $reg, 0x80);
                        });

                        assert_eq!(machine.cpu.pc, 0x0002);
                        assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 16 });
                        assert_eq!(read_register!(machine, $reg), 0x01);
                        assert_eq!(machine.cpu.f(), CARRY_FLAG);
                    }
            }
        };
    }

    macro_rules! test_rr {  
        ($instruction:expr, $reg:expr) => {
            paste! {
                    #[test]
                    fn [<test_ $instruction>]() {
                        let machine = run_test(&[0xCB, [< $instruction>]], |machine| {
                            write_register!(machine, $reg, 0x42);
                        });

                        assert_eq!(machine.cpu.pc, 0x0002);
                        assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 16 });
                        assert_eq!(read_register!(machine, $reg), 0x21);
                        assert_eq!(machine.cpu.f(), 0);
                    }

                    #[test]
                    fn [<test_ $instruction _zero>]() {
                        let machine = run_test(&[0xCB, [< $instruction>]], |machine| {
                            write_register!(machine, $reg, 0x00);
                        });

                        assert_eq!(machine.cpu.pc, 0x0002);
                        assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 16 });
                        assert_eq!(read_register!(machine, $reg), 0x00);
                        assert_eq!(machine.cpu.f(), ZERO_FLAG);
                    }

                    #[test]
                    fn [<test_ $instruction _carry>]() {
                        let machine = run_test(&[0xCB, [< $instruction>]], |machine| {
                            write_register!(machine, $reg, 0x01);
                        });

                        assert_eq!(machine.cpu.pc, 0x0002);
                        assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 16 });
                        assert_eq!(read_register!(machine, $reg), 0x00);
                        assert_eq!(machine.cpu.f(), CARRY_FLAG | ZERO_FLAG);

                        let machine = run_test(&[0xCB, [<$instruction>]], |machine| {
                            write_register!(machine, $reg, 0x00);
                            machine.cpu.set_flag(CARRY_FLAG);
                        });

                        assert_eq!(machine.cpu.pc, 0x0002);
                        assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 16 });
                        assert_eq!(read_register!(machine, $reg), 0x80);
                        assert_eq!(machine.cpu.f(), 0);
                    }
            }
        };
    }

    macro_rules! test_rl {
        ($instruction:expr, $reg:expr) => {
            paste! {
                    #[test]
                    fn [<test_ $instruction>]() {
                        let machine = run_test(&[0xCB, [<$instruction>]], |machine| {
                            write_register!(machine, $reg, 0x42);
                        });

                        assert_eq!(machine.cpu.pc, 0x0002);
                        assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 16 });
                        assert_eq!(read_register!(machine, $reg), 0x84);
                        assert_eq!(machine.cpu.f(), 0);
                    }

                    #[test]
                    fn [<test_ $instruction _zero>]() {
                        let machine = run_test(&[0xCB, [<$instruction>]], |machine| {
                            write_register!(machine, $reg, 0x00);
                        });

                        assert_eq!(machine.cpu.pc, 0x0002);
                        assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 16 });
                        assert_eq!(read_register!(machine, $reg), 0x00);
                        assert_eq!(machine.cpu.f(), ZERO_FLAG);
                    }

                    #[test]
                    fn [<test_ $instruction _carry>]() {
                        let machine = run_test(&[0xCB, [<$instruction>]], |machine| {
                            write_register!(machine, $reg, 0x80);
                        });

                        assert_eq!(machine.cpu.pc, 0x0002);
                        assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 16 });
                        assert_eq!(read_register!(machine, $reg), 0x00);
                        assert_eq!(machine.cpu.f(), CARRY_FLAG | ZERO_FLAG);

                        let machine = run_test(&[0xCB, [<$instruction>]], |machine| {
                            write_register!(machine, $reg, 0x00);
                            machine.cpu.set_flag(CARRY_FLAG);
                        });

                        assert_eq!(machine.cpu.pc, 0x0002);
                        assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 16 });
                        assert_eq!(read_register!(machine, $reg), 0x01);
                        assert_eq!(machine.cpu.f(), 0);
                    }
            }
        };
    }

    macro_rules! test_sla {
        ($instruction:expr, $reg:expr) => {
            paste! {
                    #[test]
                    fn [<test_ $instruction>]() {
                        let machine = run_test(&[0xCB, [<$instruction>]], |machine| {
                            write_register!(machine, $reg, 0x42);
                        });

                        assert_eq!(machine.cpu.pc, 0x0002);
                        assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 16 });
                        assert_eq!(read_register!(machine, $reg), 0x84);
                        assert_eq!(machine.cpu.f(), 0);
                    }

                    #[test]
                    fn [<test_ $instruction _zero>]() {
                        let machine = run_test(&[0xCB, [<$instruction>]], |machine| {
                            write_register!(machine, $reg, 0x00);
                        });

                        assert_eq!(machine.cpu.pc, 0x0002);
                        assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 16 });
                        assert_eq!(read_register!(machine, $reg), 0x00);
                        assert_eq!(machine.cpu.f(), ZERO_FLAG);
                    }

                    #[test]
                    fn [<test_ $instruction _carry>]() {
                        let machine = run_test(&[0xCB, [<$instruction>]], |machine| {
                            write_register!(machine, $reg, 0x80);
                        });

                        assert_eq!(machine.cpu.pc, 0x0002);
                        assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 16 });
                        assert_eq!(read_register!(machine, $reg), 0x00);
                        assert_eq!(machine.cpu.f(), CARRY_FLAG | ZERO_FLAG);

                        let machine = run_test(&[0xCB, [<$instruction>]], |machine| {
                            write_register!(machine, $reg, 0x00);
                            machine.cpu.set_flag(CARRY_FLAG);
                        });

                        assert_eq!(machine.cpu.pc, 0x0002);
                        assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 16 });
                        assert_eq!(read_register!(machine, $reg), 0x00);
                        assert_eq!(machine.cpu.f(), ZERO_FLAG);
                    }
                }
            }
    }

    macro_rules! test_sra {
        ($instruction:expr, $reg:expr) => {
            paste! {
                    #[test]
                    fn [<test_ $instruction>]() {
                        let machine = run_test(&[0xCB, [<$instruction>]], |machine| {
                            write_register!(machine, $reg, 0x42);
                        });

                        assert_eq!(machine.cpu.pc, 0x0002);
                        assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 16 });
                        assert_eq!(read_register!(machine, $reg), 0x21);
                        assert_eq!(machine.cpu.f(), 0);
                    }

                    #[test]
                    fn [<test_ $instruction _zero>]() {
                        let machine = run_test(&[0xCB, [<$instruction>]], |machine| {
                            write_register!(machine, $reg, 0x00);
                        });

                        assert_eq!(machine.cpu.pc, 0x0002);
                        assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 16 });
                        assert_eq!(read_register!(machine, $reg), 0x00);
                        assert_eq!(machine.cpu.f(), ZERO_FLAG);
                    }

                    #[test]
                    fn [<test_ $instruction _carry>]() {
                        let machine = run_test(&[0xCB, [<$instruction>]], |machine| {
                            write_register!(machine, $reg, 0x01);
                        });

                        assert_eq!(machine.cpu.pc, 0x0002);
                        assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 16 });
                        assert_eq!(read_register!(machine, $reg), 0x00);
                        assert_eq!(machine.cpu.f(), CARRY_FLAG | ZERO_FLAG);
                    }
            }
        };

    }

    macro_rules! test_swap {
        ($instruction:expr, $reg:expr) => {
            paste! {
                    #[test]
                    fn [<test_ $instruction>]() {
                        let machine = run_test(&[0xCB, [<$instruction>]], |machine| {
                            write_register!(machine, $reg, 0x42);
                        });

                        assert_eq!(machine.cpu.pc, 0x0002);
                        assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 16 });
                        assert_eq!(read_register!(machine, $reg), 0x24);
                        assert_eq!(machine.cpu.f(), 0);
                    }

                    #[test]
                    fn [<test_ $instruction _zero>]() {
                        let machine = run_test(&[0xCB, [<$instruction>]], |machine| {
                            write_register!(machine, $reg, 0x00);
                        });

                        assert_eq!(machine.cpu.pc, 0x0002);
                        assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 16 });
                        assert_eq!(read_register!(machine, $reg), 0x00);
                        assert_eq!(machine.cpu.f(), ZERO_FLAG);
                    }
            }
        };
    }

    macro_rules! test_srl {
        ($instruction:expr, $reg:expr) => {
            paste! {
                    #[test]
                    fn [<test_ $instruction>]() {
                        let machine = run_test(&[0xCB, [<$instruction>]], |machine| {
                            write_register!(machine, $reg, 0x42);
                        });

                        assert_eq!(machine.cpu.pc, 0x0002);
                        assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 16 });
                        assert_eq!(read_register!(machine, $reg), 0x21);
                        assert_eq!(machine.cpu.f(), 0);
                    }

                    #[test]
                    fn [<test_ $instruction _zero>]() {
                        let machine = run_test(&[0xCB, [<$instruction>]], |machine| {
                            write_register!(machine, $reg, 0x00);
                        });

                        assert_eq!(machine.cpu.pc, 0x0002);
                        assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 16 });
                        assert_eq!(read_register!(machine, $reg), 0x00);
                        assert_eq!(machine.cpu.f(), ZERO_FLAG);
                    }

                    #[test]
                    fn [<test_ $instruction _carry>]() {
                        let machine = run_test(&[0xCB, [<$instruction>]], |machine| {
                            write_register!(machine, $reg, 0x01);
                        });

                        assert_eq!(machine.cpu.pc, 0x0002);
                        assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 16 });
                        assert_eq!(read_register!(machine, $reg), 0x00);
                        assert_eq!(machine.cpu.f(), CARRY_FLAG | ZERO_FLAG);
                    }
            }
        };
    }

    macro_rules! test_bit {
        ($instruction:expr, $bit:expr, $reg:expr) => {
            paste! {
                #[test]
                fn [<test_ $instruction _ $bit>]() {
                    let machine = run_test(&[0xCB, [<$instruction>]], |machine| {
                        write_register!(machine, $reg, 1u8 << $bit);
                    });

                    assert_eq!(machine.cpu.pc, 0x0002);
                    assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 12 });
                    assert_eq!(read_register!(machine, $reg), 1u8 << $bit);
                    assert_eq!(machine.cpu.f(), HALF_FLAG);

                    let machine = run_test(&[0xCB, [<$instruction>]], |machine| {
                        write_register!(machine, $reg,  !(1u8 << $bit) );
                    });

                    assert_eq!(machine.cpu.pc, 0x0002);
                    assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 12 });
                    assert_eq!(read_register!(machine, $reg), !(1u8 << $bit));
                    assert_eq!(machine.cpu.f(), ZERO_FLAG | HALF_FLAG);
                }
            }
        };
    }

    macro_rules! test_res {
        ($instruction:expr, $bit:expr, $reg:expr) => {
            paste! {
                #[test]
                fn [<test_ $instruction _ $bit>]() {
                    let machine = run_test(&[0xCB, [<$instruction>]], |machine| {
                        write_register!(machine, $reg, 1u8 << $bit);
                    });

                    assert_eq!(machine.cpu.pc, 0x0002);
                    assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 16 });
                    assert_eq!(read_register!(machine, $reg), 0x00);
                    assert_eq!(machine.cpu.f(), 0);
                }
            }
        };
    }

    macro_rules! test_set {
        ($instruction:expr, $bit:expr, $reg:expr) => {
            paste! {
                #[test]
                fn [<test_ $instruction _ $bit>]() {
                    let machine = run_test(&[0xCB, [<$instruction>]], |machine| {
                        write_register!(machine, $reg, !(1u8 << $bit));
                    });

                    assert_eq!(machine.cpu.pc, 0x0002);
                    assert_eq!(machine.cpu.t_cycles, if $reg != Register8::IndirectHL { 8 } else { 16 });
                    assert_eq!(read_register!(machine, $reg), 0xFF);
                    assert_eq!(machine.cpu.f(), 0);
                }
            }
        };
    }

    test_rlc!(0x00, Register8::B);
    test_rlc!(0x01, Register8::C);
    test_rlc!(0x02, Register8::D);
    test_rlc!(0x03, Register8::E);
    test_rlc!(0x04, Register8::H);
    test_rlc!(0x05, Register8::L);
    test_rlc!(0x06, Register8::IndirectHL);
    test_rlc!(0x07, Register8::A);

    test_rrc!(0x08, Register8::B);
    test_rrc!(0x09, Register8::C);
    test_rrc!(0x0a, Register8::D);
    test_rrc!(0x0b, Register8::E);
    test_rrc!(0x0c, Register8::H);
    test_rrc!(0x0d, Register8::L);
    test_rrc!(0x0e, Register8::IndirectHL);
    test_rrc!(0x0f, Register8::A);

    test_rl!(0x10, Register8::B);
    test_rl!(0x11, Register8::C);
    test_rl!(0x12, Register8::D);
    test_rl!(0x13, Register8::E);
    test_rl!(0x14, Register8::H);
    test_rl!(0x15, Register8::L);
    test_rl!(0x16, Register8::IndirectHL);
    test_rl!(0x17, Register8::A);

    test_rr!(0x18, Register8::B);
    test_rr!(0x19, Register8::C);
    test_rr!(0x1a, Register8::D);
    test_rr!(0x1b, Register8::E);
    test_rr!(0x1c, Register8::H);
    test_rr!(0x1d, Register8::L);
    test_rr!(0x1e, Register8::IndirectHL);
    test_rr!(0x1f, Register8::A);

    test_sla!(0x20, Register8::B);
    test_sla!(0x21, Register8::C);
    test_sla!(0x22, Register8::D);
    test_sla!(0x23, Register8::E);
    test_sla!(0x24, Register8::H);
    test_sla!(0x25, Register8::L);
    test_sla!(0x26, Register8::IndirectHL);
    test_sla!(0x27, Register8::A);

    test_sra!(0x28, Register8::B);
    test_sra!(0x29, Register8::C);
    test_sra!(0x2a, Register8::D);
    test_sra!(0x2b, Register8::E);
    test_sra!(0x2c, Register8::H);
    test_sra!(0x2d, Register8::L);
    test_sra!(0x2e, Register8::IndirectHL);
    test_sra!(0x2f, Register8::A);

    test_swap!(0x30, Register8::B);
    test_swap!(0x31, Register8::C);
    test_swap!(0x32, Register8::D);
    test_swap!(0x33, Register8::E);
    test_swap!(0x34, Register8::H);
    test_swap!(0x35, Register8::L);
    test_swap!(0x36, Register8::IndirectHL);
    test_swap!(0x37, Register8::A);

    test_srl!(0x38, Register8::B);
    test_srl!(0x39, Register8::C);
    test_srl!(0x3a, Register8::D);
    test_srl!(0x3b, Register8::E);
    test_srl!(0x3c, Register8::H);
    test_srl!(0x3d, Register8::L);
    test_srl!(0x3e, Register8::IndirectHL);
    test_srl!(0x3f, Register8::A);

    test_bit!(0x40, 0, Register8::B);
    test_bit!(0x41, 0, Register8::C);
    test_bit!(0x42, 0, Register8::D);
    test_bit!(0x43, 0, Register8::E);
    test_bit!(0x44, 0, Register8::H);
    test_bit!(0x45, 0, Register8::L);
    test_bit!(0x46, 0, Register8::IndirectHL);
    test_bit!(0x47, 0, Register8::A);
    test_bit!(0x48, 1, Register8::B);
    test_bit!(0x49, 1, Register8::C);
    test_bit!(0x4a, 1, Register8::D);
    test_bit!(0x4b, 1, Register8::E);
    test_bit!(0x4c, 1, Register8::H);
    test_bit!(0x4d, 1, Register8::L);
    test_bit!(0x4e, 1, Register8::IndirectHL);
    test_bit!(0x4f, 1, Register8::A);
    test_bit!(0x50, 2, Register8::B);
    test_bit!(0x51, 2, Register8::C);
    test_bit!(0x52, 2, Register8::D);
    test_bit!(0x53, 2, Register8::E);
    test_bit!(0x54, 2, Register8::H);
    test_bit!(0x55, 2, Register8::L);
    test_bit!(0x56, 2, Register8::IndirectHL);
    test_bit!(0x57, 2, Register8::A);

    test_bit!(0x58, 3, Register8::B);
    test_bit!(0x59, 3, Register8::C);
    test_bit!(0x5a, 3, Register8::D);
    test_bit!(0x5b, 3, Register8::E);
    test_bit!(0x5c, 3, Register8::H);
    test_bit!(0x5d, 3, Register8::L);
    test_bit!(0x5e, 3, Register8::IndirectHL);
    test_bit!(0x5f, 3, Register8::A);

    test_bit!(0x60, 4, Register8::B);
    test_bit!(0x61, 4, Register8::C);
    test_bit!(0x62, 4, Register8::D);
    test_bit!(0x63, 4, Register8::E);
    test_bit!(0x64, 4, Register8::H);
    test_bit!(0x65, 4, Register8::L);
    test_bit!(0x66, 4, Register8::IndirectHL);
    test_bit!(0x67, 4, Register8::A);

    test_bit!(0x68, 5, Register8::B);
    test_bit!(0x69, 5, Register8::C);
    test_bit!(0x6a, 5, Register8::D);
    test_bit!(0x6b, 5, Register8::E);
    test_bit!(0x6c, 5, Register8::H);
    test_bit!(0x6d, 5, Register8::L);
    test_bit!(0x6e, 5, Register8::IndirectHL);
    test_bit!(0x6f, 5, Register8::A);

    test_bit!(0x70, 6, Register8::B);
    test_bit!(0x71, 6, Register8::C);
    test_bit!(0x72, 6, Register8::D);
    test_bit!(0x73, 6, Register8::E);
    test_bit!(0x74, 6, Register8::H);
    test_bit!(0x75, 6, Register8::L);
    test_bit!(0x76, 6, Register8::IndirectHL);
    test_bit!(0x77, 6, Register8::A);

    test_bit!(0x78, 7, Register8::B);
    test_bit!(0x79, 7, Register8::C);
    test_bit!(0x7a, 7, Register8::D);
    test_bit!(0x7b, 7, Register8::E);
    test_bit!(0x7c, 7, Register8::H);
    test_bit!(0x7d, 7, Register8::L);
    test_bit!(0x7e, 7, Register8::IndirectHL);
    test_bit!(0x7f, 7, Register8::A);

    test_res!(0x80, 0, Register8::B);
    test_res!(0x81, 0, Register8::C);
    test_res!(0x82, 0, Register8::D);
    test_res!(0x83, 0, Register8::E);
    test_res!(0x84, 0, Register8::H);
    test_res!(0x85, 0, Register8::L);
    test_res!(0x86, 0, Register8::IndirectHL);
    test_res!(0x87, 0, Register8::A);
    test_res!(0x88, 1, Register8::B);
    test_res!(0x89, 1, Register8::C);
    test_res!(0x8a, 1, Register8::D);
    test_res!(0x8b, 1, Register8::E);
    test_res!(0x8c, 1, Register8::H);
    test_res!(0x8d, 1, Register8::L);
    test_res!(0x8e, 1, Register8::IndirectHL);
    test_res!(0x8f, 1, Register8::A);

    test_res!(0x90, 2, Register8::B);
    test_res!(0x91, 2, Register8::C);
    test_res!(0x92, 2, Register8::D);
    test_res!(0x93, 2, Register8::E);
    test_res!(0x94, 2, Register8::H);
    test_res!(0x95, 2, Register8::L);
    test_res!(0x96, 2, Register8::IndirectHL);
    test_res!(0x97, 2, Register8::A);

    test_res!(0x98, 3, Register8::B);
    test_res!(0x99, 3, Register8::C);
    test_res!(0x9a, 3, Register8::D);
    test_res!(0x9b, 3, Register8::E);
    test_res!(0x9c, 3, Register8::H);
    test_res!(0x9d, 3, Register8::L);
    test_res!(0x9e, 3, Register8::IndirectHL);
    test_res!(0x9f, 3, Register8::A);

    test_res!(0xa0, 4, Register8::B);
    test_res!(0xa1, 4, Register8::C);
    test_res!(0xa2, 4, Register8::D);
    test_res!(0xa3, 4, Register8::E);
    test_res!(0xa4, 4, Register8::H);
    test_res!(0xa5, 4, Register8::L);
    test_res!(0xa6, 4, Register8::IndirectHL);
    test_res!(0xa7, 4, Register8::A);

    test_res!(0xa8, 5, Register8::B);
    test_res!(0xa9, 5, Register8::C);
    test_res!(0xaa, 5, Register8::D);
    test_res!(0xab, 5, Register8::E);
    test_res!(0xac, 5, Register8::H);
    test_res!(0xad, 5, Register8::L);
    test_res!(0xae, 5, Register8::IndirectHL);
    test_res!(0xaf, 5, Register8::A);

    test_res!(0xb0, 6, Register8::B);
    test_res!(0xb1, 6, Register8::C);
    test_res!(0xb2, 6, Register8::D);
    test_res!(0xb3, 6, Register8::E);
    test_res!(0xb4, 6, Register8::H);
    test_res!(0xb5, 6, Register8::L);
    test_res!(0xb6, 6, Register8::IndirectHL);
    test_res!(0xb7, 6, Register8::A);

    test_res!(0xb8, 7, Register8::B);
    test_res!(0xb9, 7, Register8::C);
    test_res!(0xba, 7, Register8::D);
    test_res!(0xbb, 7, Register8::E);
    test_res!(0xbc, 7, Register8::H);
    test_res!(0xbd, 7, Register8::L);
    test_res!(0xbe, 7, Register8::IndirectHL);
    test_res!(0xbf, 7, Register8::A);

    test_set!(0xc0, 0, Register8::B);
    test_set!(0xc1, 0, Register8::C);
    test_set!(0xc2, 0, Register8::D);
    test_set!(0xc3, 0, Register8::E);
    test_set!(0xc4, 0, Register8::H);
    test_set!(0xc5, 0, Register8::L);
    test_set!(0xc6, 0, Register8::IndirectHL);
    test_set!(0xc7, 0, Register8::A);
    test_set!(0xc8, 1, Register8::B);
    test_set!(0xc9, 1, Register8::C);
    test_set!(0xca, 1, Register8::D);
    test_set!(0xcb, 1, Register8::E);
    test_set!(0xcc, 1, Register8::H);
    test_set!(0xcd, 1, Register8::L);
    test_set!(0xce, 1, Register8::IndirectHL);
    test_set!(0xcf, 1, Register8::A);

    test_set!(0xd0, 2, Register8::B);
    test_set!(0xd1, 2, Register8::C);
    test_set!(0xd2, 2, Register8::D);
    test_set!(0xd3, 2, Register8::E);
    test_set!(0xd4, 2, Register8::H);
    test_set!(0xd5, 2, Register8::L);
    test_set!(0xd6, 2, Register8::IndirectHL);
    test_set!(0xd7, 2, Register8::A);

    test_set!(0xd8, 3, Register8::B);
    test_set!(0xd9, 3, Register8::C);
    test_set!(0xda, 3, Register8::D);
    test_set!(0xdb, 3, Register8::E);
    test_set!(0xdc, 3, Register8::H);
    test_set!(0xdd, 3, Register8::L);
    test_set!(0xde, 3, Register8::IndirectHL);
    test_set!(0xdf, 3, Register8::A);

    test_set!(0xe0, 4, Register8::B);
    test_set!(0xe1, 4, Register8::C);
    test_set!(0xe2, 4, Register8::D);
    test_set!(0xe3, 4, Register8::E);
    test_set!(0xe4, 4, Register8::H);
    test_set!(0xe5, 4, Register8::L);
    test_set!(0xe6, 4, Register8::IndirectHL);
    test_set!(0xe7, 4, Register8::A);

    test_set!(0xe8, 5, Register8::B);
    test_set!(0xe9, 5, Register8::C);
    test_set!(0xea, 5, Register8::D);
    test_set!(0xeb, 5, Register8::E);
    test_set!(0xec, 5, Register8::H);
    test_set!(0xed, 5, Register8::L);
    test_set!(0xee, 5, Register8::IndirectHL);
    test_set!(0xef, 5, Register8::A);

    test_set!(0xf0, 6, Register8::B);
    test_set!(0xf1, 6, Register8::C);
    test_set!(0xf2, 6, Register8::D);
    test_set!(0xf3, 6, Register8::E);
    test_set!(0xf4, 6, Register8::H);
    test_set!(0xf5, 6, Register8::L);
    test_set!(0xf6, 6, Register8::IndirectHL);
    test_set!(0xf7, 6, Register8::A);

    test_set!(0xf8, 7, Register8::B);
    test_set!(0xf9, 7, Register8::C);
    test_set!(0xfa, 7, Register8::D);
    test_set!(0xfb, 7, Register8::E);
    test_set!(0xfc, 7, Register8::H);
    test_set!(0xfd, 7, Register8::L);
    test_set!(0xfe, 7, Register8::IndirectHL);
    test_set!(0xff, 7, Register8::A);
}
