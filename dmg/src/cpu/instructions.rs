use super::*;

use crate::read_register;
use crate::read_register16;
use crate::write_register;
use crate::write_register16;

use log::*;

impl CPU {
    /*******************************************/
    /*       Miscellaneous instructions        */
    /*******************************************/

    pub(crate) fn nop(&mut self) {
        // Do nothing
    }

    pub(crate) fn stop(&mut self) {
        self.mode = CPUMode::Stop;
    }

    pub(crate) fn halt(&mut self) {
        if self.ime {
            self.mode = CPUMode::Halt;
        } else {
            let mmu = self.mmu.borrow();

            if mmu.ic.borrow().interrupt_enable.0 != 0
                && (mmu.ic.borrow().interrupt_flag.0 & mmu.ic.borrow().interrupt_enable.0 & 0x1F)
                    != 0
            {
                self.mode = CPUMode::HaltBug;
                info!("CPU halted (bug)");
            } else {
                self.mode = CPUMode::Halt;
            }
        }
    }

    pub(crate) fn ei(&mut self) {
        self.ime = true;
    }

    pub(crate) fn di(&mut self) {
        self.ime = false;
    }

    /*******************************************/
    /*         8-bit Load instructions         */
    /*******************************************/
    pub(crate) fn ld_r_r(&mut self, dest: Register8, src: Register8) {
        let value = read_register!(self, src);
        write_register!(self, dest, value);
    }

    pub(crate) fn ld_r_n(&mut self, register: Register8) {
        let n = self.read_operand();
        write_register!(self, register, n);
    }

    // pub(crate) fn ld_r_hl(&mut self, register: Register8) {
    //     let n = self.read_byte(self.hl.value());
    //     write_register!(self, register, n);
    // }

    pub(crate) fn ld_hl_r(&mut self, register: Register8) {
        let value = read_register!(self, register);
        self.write_byte(self.hl.value(), value);
    }

    pub(crate) fn ld_bc_a(&mut self) {
        self.write_byte(self.bc.value(), self.a());
    }

    pub(crate) fn ld_de_a(&mut self) {
        self.write_byte(self.de.value(), self.a());
    }

    pub(crate) fn ld_a_rr(&mut self, src: u16) {
        let n = self.read_byte(src);
        *self.a_mut() = n;
    }

    pub(crate) fn ldi_hl_a(&mut self) {
        self.write_byte(self.hl.value(), self.a());
        *self.hl.value_mut() = self.hl.value().wrapping_add(1);
    }

    pub(crate) fn ldi_a_hl(&mut self) {
        *self.a_mut() = self.read_byte(self.hl.value());
        *self.hl.value_mut() = self.hl.value().wrapping_add(1);
    }

    pub(crate) fn ldd_hl_a(&mut self) {
        self.write_byte(self.hl.value(), self.a());
        *self.hl.value_mut() = self.hl.value().wrapping_sub(1);
    }

    pub(crate) fn ldd_a_hl(&mut self) {
        *self.a_mut() = self.read_byte(self.hl.value());
        *self.hl.value_mut() = self.hl.value().wrapping_sub(1);
    }

    pub(crate) fn ld_indirect_hl_n(&mut self) {
        let n = self.read_operand();
        self.write_byte(self.hl.value(), n);
    }

    pub(crate) fn ldh_n_a(&mut self) {
        let n = self.read_operand();
        self.write_byte(0xFF00 | n as u16, self.a());
    }

    pub(crate) fn ldh_c_a(&mut self) {
        let c = self.c();
        self.write_byte(0xFF00 | c as u16, self.a());
    }

    pub(crate) fn ld_nn_a(&mut self) {
        let nn = self.read_word();
        self.write_byte(nn, self.a());
    }

    pub(crate) fn ldh_a_n(&mut self) {
        let n = self.read_operand();
        *self.a_mut() = self.read_byte(0xFF00 | n as u16);
    }

    pub(crate) fn ldh_a_c(&mut self) {
        let c = self.c();
        *self.a_mut() = self.read_byte(0xFF00 | c as u16);
    }

    pub(crate) fn ld_a_nn(&mut self) {
        let nn = self.read_word();
        *self.a_mut() = self.read_byte(nn);
    }

    /*******************************************/
    /*         16-bit Load instructions        */
    /*******************************************/
    pub(crate) fn ld_rr_nn(&mut self, rr: Register16) {
        let value = self.read_word();
        write_register16!(self, rr, value);
    }

    pub(crate) fn ld_hl_sp_e8(&mut self) {
        let e8 = self.read_operand() as i8 as i16 as u16;
        let sp = self.sp;

        let hl = sp.wrapping_add(e8);

        *self.hl.value_mut() = hl as u16;

        self.clear_flag(SUB_FLAG);
        self.clear_flag(ZERO_FLAG);
        self.update_flag(HALF_FLAG, (sp & 0xF) > 0xF - (e8 & 0xF));
        self.update_flag(CARRY_FLAG, (sp & 0xFF) > 0xFF - (e8 & 0xFF));

        self.tick4();
    }

    pub(crate) fn ld_sp_hl(&mut self) {
        self.sp = self.hl.value();
    }

    /*******************************************/
    /* 8-bit Arithmetic and Logical operations */
    /*******************************************/

    pub(crate) fn add_a(&mut self, value: u8) {
        let a = self.a();
        let result = a.wrapping_add(value);

        self.update_flag(ZERO_FLAG, result == 0);
        self.clear_flag(SUB_FLAG);
        self.update_flag(HALF_FLAG, (a & 0xF) + (value & 0xF) > 0xF);
        self.update_flag(CARRY_FLAG, result < a);

        *self.a_mut() = result;
    }

    pub(crate) fn add_a_hl(&mut self) {
        let value = self.read_byte(self.hl.value());
        self.add_a(value);
    }

    pub(crate) fn add_a_n(&mut self) {
        let value = self.read_operand();
        self.add_a(value);
    }

    pub(crate) fn adc_a(&mut self, value: u8) {
        let a = self.a();
        let carry = if self.get_flag(CARRY_FLAG) { 1 } else { 0 };
        let result = a.wrapping_add(value).wrapping_add(carry);

        self.update_flag(ZERO_FLAG, result == 0);
        self.clear_flag(SUB_FLAG);
        self.update_flag(HALF_FLAG, (a & 0xF) + (value & 0xF) + carry > 0xF);
        self.update_flag(CARRY_FLAG, result < a || (result == a && carry > 0));

        *self.a_mut() = result;
    }

    pub(crate) fn adc_a_hl(&mut self) {
        let value = self.read_byte(self.hl.value());
        self.adc_a(value);
    }

    pub(crate) fn adc_a_n(&mut self) {
        let value = self.read_operand();
        self.adc_a(value);
    }

    pub(crate) fn sub_a(&mut self, value: u8) {
        let a = self.a();
        let result = a.wrapping_sub(value);

        self.update_flag(ZERO_FLAG, result == 0);
        self.set_flag(SUB_FLAG);
        self.update_flag(HALF_FLAG, (a & 0xF) < (value & 0xF));
        self.update_flag(CARRY_FLAG, a < value);

        *self.a_mut() = result;
    }

    pub(crate) fn sub_a_hl(&mut self) {
        let value = self.read_byte(self.hl.value());
        self.sub_a(value);
    }

    pub(crate) fn sub_a_n(&mut self) {
        let value = self.read_operand();
        self.sub_a(value);
    }

    pub(crate) fn sbc_a(&mut self, value: u8) {
        let a = self.a();
        let carry = if self.get_flag(CARRY_FLAG) { 1 } else { 0 };
        let result = a.wrapping_sub(value).wrapping_sub(carry);

        self.update_flag(ZERO_FLAG, result == 0);
        self.set_flag(SUB_FLAG);
        self.update_flag(HALF_FLAG, (a & 0xF) < (value & 0xF) + carry);
        self.update_flag(CARRY_FLAG, (a as u16) < (value as u16) + carry as u16);

        *self.a_mut() = result;
    }

    pub(crate) fn sbc_a_hl(&mut self) {
        let value = self.read_byte(self.hl.value());
        self.sbc_a(value);
    }

    pub(crate) fn sbc_a_n(&mut self) {
        let value = self.read_operand();
        self.sbc_a(value);
    }

    pub(crate) fn cp_a(&mut self, value: u8) {
        let a = self.a();
        let result = a.wrapping_sub(value);

        self.update_flag(ZERO_FLAG, result == 0);
        self.set_flag(SUB_FLAG);
        self.update_flag(HALF_FLAG, (a & 0xF) < (value & 0xF));
        self.update_flag(CARRY_FLAG, a < value);
    }

    pub(crate) fn cp_a_hl(&mut self) {
        let value = self.read_byte(self.hl.value());
        self.cp_a(value);
    }

    pub(crate) fn cp_a_n(&mut self) {
        let value = self.read_operand();
        self.cp_a(value);
    }

    pub(crate) fn inc_r(&mut self, register: Register8) {
        let value = read_register!(self, register);
        let result = value.wrapping_add(1);

        write_register!(self, register, result);

        self.update_flag(ZERO_FLAG, result == 0);
        self.clear_flag(SUB_FLAG);
        self.update_flag(HALF_FLAG, (value & 0xF) == 0xF);
    }

    pub(crate) fn inc_indirect_hl(&mut self) {
        let data = self.read_byte(self.hl.value());
        let result = data.wrapping_add(1);
        self.write_byte(self.hl.value(), result);

        self.update_flag(ZERO_FLAG, result == 0);
        self.clear_flag(SUB_FLAG);
        self.update_flag(HALF_FLAG, (result & 0xF) == 0);
    }

    pub(crate) fn dec_indirect_hl(&mut self) {
        let data = self.read_byte(self.hl.value());
        let result = data.wrapping_sub(1);
        self.write_byte(self.hl.value(), result);

        self.update_flag(ZERO_FLAG, result == 0);
        self.set_flag(SUB_FLAG);
        self.update_flag(HALF_FLAG, (result & 0xF) == 0x0F);
    }

    pub(crate) fn dec_r(&mut self, register: Register8) {
        let value = read_register!(self, register);
        let result = value.wrapping_sub(1);

        write_register!(self, register, result);

        self.update_flag(ZERO_FLAG, result == 0);
        self.set_flag(SUB_FLAG);
        self.update_flag(HALF_FLAG, (value & 0xF) == 0x0);
    }

    pub(crate) fn and_a(&mut self, value: u8) {
        let result = self.a() & value;

        self.update_flag(ZERO_FLAG, result == 0);
        self.clear_flag(SUB_FLAG);
        self.set_flag(HALF_FLAG);
        self.clear_flag(CARRY_FLAG);

        *self.a_mut() = result;
    }

    pub(crate) fn and_a_hl(&mut self) {
        let value = self.read_byte(self.hl.value());
        self.and_a(value);
    }

    pub(crate) fn and_a_n(&mut self) {
        let value = self.read_operand();
        self.and_a(value);
    }

    pub(crate) fn or_a(&mut self, value: u8) {
        let result = self.a() | value;

        self.update_flag(ZERO_FLAG, result == 0);
        self.clear_flag(SUB_FLAG);
        self.clear_flag(HALF_FLAG);
        self.clear_flag(CARRY_FLAG);

        *self.a_mut() = result;
    }

    pub(crate) fn or_a_hl(&mut self) {
        let value = self.read_byte(self.hl.value());
        self.or_a(value);
    }

    pub(crate) fn or_a_n(&mut self) {
        let value = self.read_operand();
        self.or_a(value);
    }

    pub(crate) fn xor_a(&mut self, value: u8) {
        let result = self.a() ^ value;

        self.update_flag(ZERO_FLAG, result == 0);
        self.clear_flag(SUB_FLAG);
        self.clear_flag(HALF_FLAG);
        self.clear_flag(CARRY_FLAG);

        *self.a_mut() = result;
    }

    pub(crate) fn xor_a_hl(&mut self) {
        let value = self.read_byte(self.hl.value());
        self.xor_a(value);
    }

    pub(crate) fn xor_a_n(&mut self) {
        let value = self.read_operand();
        self.xor_a(value);
    }

    pub(crate) fn ccf(&mut self) {
        self.clear_flag(SUB_FLAG);
        self.clear_flag(HALF_FLAG);
        let carry = self.get_flag(CARRY_FLAG);
        self.update_flag(CARRY_FLAG, !carry);
    }

    pub(crate) fn scf(&mut self) {
        self.clear_flag(SUB_FLAG);
        self.clear_flag(HALF_FLAG);
        self.set_flag(CARRY_FLAG);
    }

    pub(crate) fn daa(&mut self) {
        // TODO: Check if this is correct
        let mut a = self.a();
        let mut new_carry = false;

        let carry_flag = self.get_flag(CARRY_FLAG);
        let half_carry_flag = self.get_flag(HALF_FLAG);
        let subtract_flag = self.get_flag(SUB_FLAG);

        if !subtract_flag {
            if carry_flag || a > 0x99 {
                a = a.wrapping_add(0x60);
                new_carry = true;
            }
            if half_carry_flag || (a & 0x0F) > 0x09 {
                a = a.wrapping_add(0x06);
            }
        } else if carry_flag {
            new_carry = true;
            a = a.wrapping_add(if half_carry_flag { 0x9A } else { 0xA0 });
        } else if half_carry_flag {
            a = a.wrapping_add(0xFA);
        }

        self.update_flag(CARRY_FLAG, new_carry);
        self.update_flag(ZERO_FLAG, a == 0);
        self.clear_flag(HALF_FLAG);

        *self.a_mut() = a;
    }

    pub(crate) fn cpl(&mut self) {
        *self.a_mut() = !self.a();
        self.set_flag(SUB_FLAG);
        self.set_flag(HALF_FLAG);
    }

    /*******************************************/
    /*            16-bit Arithmetic            */
    /*******************************************/

    pub(crate) fn inc_rr(&mut self, rr: Register16) {
        let value = read_register16!(self, rr);
        let result = value.wrapping_add(1);
        self.tick4();
        write_register16!(self, rr, result);
    }

    pub(crate) fn dec_rr(&mut self, rr: Register16) {
        let value = read_register16!(self, rr);
        let result = value.wrapping_sub(1);
        self.tick4();
        write_register16!(self, rr, result);
    }

    pub(crate) fn add_hl(&mut self, rr: Register16) {
        let hl = self.hl.value();
        let value = read_register16!(self, rr);

        let result = hl.wrapping_add(value);

        self.clear_flag(SUB_FLAG);
        self.update_flag(HALF_FLAG, ((hl & 0xFFF) + (value & 0xFFF)) > 0xFFF);
        self.update_flag(CARRY_FLAG, hl > 0xFFFF - value);

        *self.hl.value_mut() = result;

        self.tick4();
    }

    pub(crate) fn add_sp(&mut self) {
        let sp = self.sp;
        let value = self.read_operand() as i8 as i16 as u16;
        self.sp = sp.wrapping_add(value);

        self.clear_flag(SUB_FLAG);
        self.clear_flag(ZERO_FLAG);
        self.update_flag(HALF_FLAG, (sp & 0xF) > 0xF - (value & 0xF));
        self.update_flag(CARRY_FLAG, (sp & 0xFF) > 0xFF - (value & 0xFF));

        self.tick4();
        self.tick4();
    }

    /*******************************************/
    /*     Rotate, Shift & Bit operations      */
    /*******************************************/

    pub(crate) fn rla(&mut self) {
        let carry_old = self.get_flag(CARRY_FLAG) as u8;
        let a = self.a_mut();

        let carry_new = *a >> 7;
        *a = (*a << 1) | carry_old;

        self.clear_flag(ZERO_FLAG | SUB_FLAG | HALF_FLAG);
        self.update_flag(CARRY_FLAG, carry_new == 1);
    }

    pub(crate) fn rra(&mut self) {
        let carry_old = self.get_flag(CARRY_FLAG) as u8;
        let a = self.a_mut();

        let carry_new = *a & 1;
        *a = (*a >> 1) | (carry_old << 7);

        self.clear_flag(ZERO_FLAG | SUB_FLAG | HALF_FLAG);
        self.update_flag(CARRY_FLAG, carry_new == 1);
    }

    pub(crate) fn rlca(&mut self) {
        let a = self.a_mut();
        let carry = *a >> 7;
        *a = a.rotate_left(1);

        self.clear_flag(ZERO_FLAG | SUB_FLAG | HALF_FLAG);
        self.update_flag(CARRY_FLAG, carry == 1);
    }

    pub(crate) fn rrca(&mut self) {
        let a = self.a_mut();
        let carry = *a & 1;
        *a = a.rotate_right(1);

        self.clear_flag(ZERO_FLAG | SUB_FLAG | HALF_FLAG);
        self.update_flag(CARRY_FLAG, (carry == 1) as bool);
    }

    /*******************************************/
    /*              Control Flow               */
    /*******************************************/

    pub(crate) fn jp_nn(&mut self) {
        let nn = self.read_word();
        self.pc = nn;

        self.tick4();
    }

    pub(crate) fn jr_e(&mut self) {
        let nn = self.read_operand() as i8;
        self.pc = self.pc.wrapping_add(nn as u16);
        self.tick4();
    }

    pub(crate) fn jp_cc_nn(&mut self, flag: u8, condition: bool) {
        let nn = self.read_word();

        if self.get_flag(flag) == condition {
            self.pc = nn;
            self.tick4();
        } else {
            // Skip the jump
        }
    }

    pub(crate) fn jr_cc_e(&mut self, flag: u8, condition: bool) {
        let nn = self.read_operand() as i8;

        if self.get_flag(flag) == condition {
            self.tick4();
            self.pc = self.pc.wrapping_add(nn as u16);
        } else {
            // nothing
        }
    }

    pub(crate) fn jp_hl(&mut self) {
        self.pc = self.hl.value();
    }

    pub(crate) fn call(&mut self) {
        let nn = self.read_word();

        // Push current PC onto the stack
        self.sp = self.sp.wrapping_sub(1);
        self.write_byte(self.sp, (self.pc >> 8) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.write_byte(self.sp, self.pc as u8);

        // Set PC to the new address
        self.pc = nn;
    }

    pub(crate) fn call_cc_nn(&mut self, flag: u8, condition: bool) {
        if self.get_flag(flag) == condition {
            self.call();
        } else {
            // Skip the call
            self.pc = self.pc.wrapping_add(2);
            self.tick4();
        }
    }

    pub(crate) fn ret(&mut self) {
        let low = self.read_byte(self.sp);
        self.sp = self.sp.wrapping_add(1);
        let high = self.read_byte(self.sp);
        self.sp = self.sp.wrapping_add(1);

        self.pc = u16::from_le_bytes([low, high]);
        self.tick4();
    }

    pub(crate) fn reti(&mut self) {
        self.ret();
        self.ime = true;
    }

    pub(crate) fn ret_cc(&mut self, flag: u8, condition: bool) {
        if self.get_flag(flag) == condition {
            self.ret();
        } else {
            self.tick4();
        }
    }

    pub(crate) fn rst(&mut self, vector: u8) {
        self.sp = self.sp.wrapping_sub(1);
        self.write_byte(self.sp, (self.pc >> 8) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.write_byte(self.sp, self.pc as u8);

        self.pc = vector as u16;
    }

    /*******************************************/
    /*     Stack manipulation instructions     */
    /*******************************************/
    pub(crate) fn ld_nn_sp(&mut self) {
        let nn = self.read_word();

        self.write_byte(nn, self.sp as u8);
        self.write_byte(nn.wrapping_add(1), (self.sp >> 8) as u8);
    }

    pub(crate) fn pop(&mut self, rr: Register16) {
        let low = self.read_byte(self.sp);
        self.sp = self.sp.wrapping_add(1);
        let high = self.read_byte(self.sp);
        self.sp = self.sp.wrapping_add(1);

        let mut value = ((high as u16) << 8) | (low as u16);

        if rr == Register16::AF {
            value = value & 0xFFF0;
        }

        write_register16!(self, rr, value);
    }

    pub(crate) fn push(&mut self, rr: Register16) {
        let value = read_register16!(self, rr);
        self.sp = self.sp.wrapping_sub(1);
        self.write_byte(self.sp, (value >> 8) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.write_byte(self.sp, value as u8);

        self.tick4();
    }

    /*******************************************/
    /*              CB Instructions            */
    /*******************************************/

    pub(crate) fn rlc_r(&mut self, register: Register8) {
        let mut reg_value = read_register!(self, register);
        reg_value = reg_value.rotate_left(1);
        let carry = reg_value & 1;
        write_register!(self, register, reg_value);

        self.update_flag(ZERO_FLAG, reg_value == 0);
        self.clear_flag(SUB_FLAG | HALF_FLAG);
        self.update_flag(CARRY_FLAG, carry == 1);
    }

    pub(crate) fn rrc_r(&mut self, register: Register8) {
        let mut reg_value = read_register!(self, register);
        let carry = reg_value & 1;
        reg_value = reg_value.rotate_right(1);
        write_register!(self, register, reg_value);

        self.update_flag(ZERO_FLAG, reg_value == 0);
        self.clear_flag(SUB_FLAG | HALF_FLAG);
        self.update_flag(CARRY_FLAG, carry == 1);
    }

    pub(crate) fn rl_r(&mut self, register: Register8) {
        let mut reg_value = read_register!(self, register);
        let carry = if self.get_flag(CARRY_FLAG) { 1 } else { 0 };
        let new_carry = reg_value >> 7;
        reg_value = (reg_value << 1) | carry;
        write_register!(self, register, reg_value);

        self.update_flag(ZERO_FLAG, reg_value == 0);
        self.clear_flag(SUB_FLAG | HALF_FLAG);
        self.update_flag(CARRY_FLAG, new_carry == 1);
    }

    pub(crate) fn rr_r(&mut self, register: Register8) {
        let mut reg_value = read_register!(self, register);
        let carry = if self.get_flag(CARRY_FLAG) { 1 } else { 0 };
        let new_carry = reg_value & 1;
        reg_value = (reg_value >> 1) | (carry << 7);
        write_register!(self, register, reg_value);

        self.update_flag(ZERO_FLAG, reg_value == 0);
        self.clear_flag(SUB_FLAG | HALF_FLAG);
        self.update_flag(CARRY_FLAG, new_carry == 1);
    }

    pub(crate) fn sla_r(&mut self, register: Register8) {
        let mut reg_value = read_register!(self, register);
        let carry = reg_value >> 7;
        reg_value <<= 1;
        write_register!(self, register, reg_value);

        self.update_flag(ZERO_FLAG, reg_value == 0);
        self.clear_flag(SUB_FLAG | HALF_FLAG);
        self.update_flag(CARRY_FLAG, carry == 1);
    }

    pub(crate) fn sra_r(&mut self, register: Register8) {
        let mut reg_value = read_register!(self, register);
        let carry = reg_value & 1;
        reg_value = (reg_value >> 1) | (reg_value & 0x80);
        write_register!(self, register, reg_value);

        self.update_flag(ZERO_FLAG, reg_value == 0);
        self.clear_flag(SUB_FLAG | HALF_FLAG);
        self.update_flag(CARRY_FLAG, carry == 1);
    }

    pub(crate) fn swap_r(&mut self, register: Register8) {
        let mut reg_value = read_register!(self, register);
        let high_nibble = reg_value & 0xF0;
        let low_nibble = reg_value & 0x0F;
        reg_value = (low_nibble << 4) | (high_nibble >> 4);
        write_register!(self, register, reg_value);

        self.update_flag(ZERO_FLAG, reg_value == 0);
        self.clear_flag(SUB_FLAG | HALF_FLAG | CARRY_FLAG);
    }

    pub(crate) fn srl_r(&mut self, register: Register8) {
        let mut reg_value = read_register!(self, register);
        let carry = reg_value & 1;
        reg_value >>= 1;
        write_register!(self, register, reg_value);

        self.update_flag(ZERO_FLAG, reg_value == 0);
        self.clear_flag(SUB_FLAG | HALF_FLAG);
        self.update_flag(CARRY_FLAG, carry == 1);
    }

    pub(crate) fn bit_r(&mut self, bit: U3, register: Register8) {
        let value = read_register!(self, register);
        let result = value & (1 << bit as u8);

        self.update_flag(ZERO_FLAG, result == 0);
        self.clear_flag(SUB_FLAG);
        self.set_flag(HALF_FLAG);
    }

    pub(crate) fn res_r(&mut self, bit: U3, register: Register8) {
        let value = read_register!(self, register);
        let new_value = value & !(1 << bit as u8);
        write_register!(self, register, new_value);
    }

    pub(crate) fn set_r(&mut self, bit: U3, register: Register8) {
        let value = read_register!(self, register);
        let new_value = value | (1 << bit as u8);
        write_register!(self, register, new_value);
    }
}
