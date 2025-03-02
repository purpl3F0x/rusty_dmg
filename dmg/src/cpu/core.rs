use super::Reg;

const ZERO_FLAG: u8 = 0x80;
const SUB_FLAG: u8 = 0x40;
const HALF_FLAG: u8 = 0x20;
const CARRY_FLAG: u8 = 0x10;

#[derive(Debug)]
pub struct CPU {
    af: Reg,
    bc: Reg,
    de: Reg,
    hl: Reg,
    pc: u16,
    sp: u16,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            af: Reg::new(),
            bc: Reg::new(),
            de: Reg::new(),
            hl: Reg::new(),
            pc: 0u16,
            sp: 0u16,
        }
    }

    /* Mutable 8-bit register methods  */

    pub fn a_mut(&mut self) -> &mut u8 {
        self.af.high_mut()
    }

    pub fn h_mut(&mut self) -> &mut u8 {
        self.af.low_mut()
    }

    pub fn b_mut(&mut self) -> &mut u8 {
        self.bc.high_mut()
    }

    pub fn c_mut(&mut self) -> &mut u8 {
        self.bc.low_mut()
    }

    pub fn d_mut(&mut self) -> &mut u8 {
        self.de.high_mut()
    }

    pub fn e_mut(&mut self) -> &mut u8 {
        self.de.low_mut()
    }

    pub fn l_mut(&mut self) -> &mut u8 {
        self.hl.high_mut()
    }

    pub fn f_mut(&mut self) -> &mut u8 {
        self.hl.low_mut()
    }

    /* Immutable 8-bit register methods  */

    pub fn a(&mut self) -> u8 {
        self.af.high()
    }

    pub fn h(&mut self) -> u8 {
        self.af.low()
    }

    pub fn b(&mut self) -> u8 {
        self.bc.high()
    }

    pub fn c(&mut self) -> u8 {
        self.bc.low()
    }

    pub fn d(&mut self) -> u8 {
        self.de.high()
    }

    pub fn e(&mut self) -> u8 {
        self.de.low()
    }

    pub fn l(&mut self) -> u8 {
        self.hl.high()
    }

    pub fn f(&mut self) -> u8 {
        self.hl.low()
    }

    pub fn set_flag(&mut self, flag: u8) {
        *self.f_mut() |= flag;
    }

    pub fn clear_flag(&mut self, flag: u8) {
        *self.f_mut() &= !flag;
    }

    pub fn update_flag(&mut self, flag: u8, value: bool) {
        if value {
            self.set_flag(flag);
        } else {
            self.clear_flag(flag);
        }
    }

    pub fn get_flag(&mut self, flag: u8) -> bool {
        self.f() & flag != 0
    }

    /*******************************************/
    /* 8-bit Arithmetic and Logical operations */
    /*******************************************/

    fn add_a(&mut self, value: u8) {
        let a = self.a();
        let result = a.wrapping_add(value);

        self.update_flag(ZERO_FLAG, result == 0);
        self.clear_flag(SUB_FLAG);
        self.update_flag(HALF_FLAG, (a & 0xF) + (value & 0xF) > 0xF);
        self.update_flag(CARRY_FLAG, (a as u16 + value as u16) > 0xFF);

        *self.a_mut() = result;
    }

    fn adc_a(&mut self, value: u8) {
        let a = self.a();
        let carry = if self.get_flag(CARRY_FLAG) { 1 } else { 0 };
        let result = a.wrapping_add(value).wrapping_add(carry);

        self.update_flag(ZERO_FLAG, result == 0);
        self.clear_flag(SUB_FLAG);
        self.update_flag(HALF_FLAG, (a & 0xF) + (value & 0xF) + carry > 0xF);
        self.update_flag(CARRY_FLAG, (a as u16 + value as u16 + carry as u16) > 0xFF);

        *self.a_mut() = result;
    }

    fn sub_a(&mut self, value: u8) {
        let a = self.a();
        let result = a.wrapping_sub(value);

        self.update_flag(ZERO_FLAG, result == 0);
        self.set_flag(SUB_FLAG);
        self.update_flag(HALF_FLAG, (a & 0xF) < (value & 0xF));
        self.update_flag(CARRY_FLAG, (a as u16) < (value as u16));

        *self.a_mut() = result;
    }

    fn sbc_a(&mut self, value: u8) {
        let a = self.a();
        let carry = if self.get_flag(CARRY_FLAG) { 1 } else { 0 };
        let result = a.wrapping_sub(value).wrapping_sub(carry);

        self.update_flag(ZERO_FLAG, result == 0);
        self.set_flag(SUB_FLAG);
        self.update_flag(HALF_FLAG, (a & 0xF) < (value & 0xF) + carry);
        self.update_flag(CARRY_FLAG, (a as u16) < (value as u16) + carry as u16);

        *self.a_mut() = result;
    }

    fn inc(&mut self, register: &mut u8) {
        *register = register.wrapping_add(1);

        self.update_flag(ZERO_FLAG, *register == 0);
        self.clear_flag(SUB_FLAG);
        self.update_flag(HALF_FLAG, (*register & 0xF) == 0xF);
    }

    fn inc_hl(&mut self) {
        !unimplemented!()
    }

    fn dec(&mut self, register: &mut u8) {
        *register = register.wrapping_sub(1);

        self.update_flag(ZERO_FLAG, *register == 0);
        self.set_flag(SUB_FLAG);
        self.update_flag(HALF_FLAG, (*register & 0xF) == 0xF);
    }

    fn dec_hl(&mut self) {
        !unimplemented!()
    }

    fn and_a(&mut self, value: u8) {
        let result = self.a() & value;

        self.update_flag(ZERO_FLAG, result == 0);
        self.clear_flag(SUB_FLAG);
        self.set_flag(HALF_FLAG);
        self.clear_flag(CARRY_FLAG);

        *self.a_mut() = result;
    }

    fn or_a(&mut self, value: u8) {
        let result = self.a() | value;

        self.update_flag(ZERO_FLAG, result == 0);
        self.clear_flag(SUB_FLAG);
        self.clear_flag(HALF_FLAG);
        self.clear_flag(CARRY_FLAG);

        *self.a_mut() = result;
    }

    fn xor_a(&mut self, value: u8) {
        let result = self.a() ^ value;

        self.update_flag(ZERO_FLAG, result == 0);
        self.clear_flag(SUB_FLAG);
        self.clear_flag(HALF_FLAG);
        self.clear_flag(CARRY_FLAG);

        *self.a_mut() = result;
    }

    fn ccf(&mut self) {
        self.clear_flag(SUB_FLAG);
        self.clear_flag(HALF_FLAG);
        let carry = self.get_flag(CARRY_FLAG);
        self.update_flag(CARRY_FLAG, !carry);
    }

    fn scf(&mut self) {
        self.clear_flag(SUB_FLAG);
        self.clear_flag(HALF_FLAG);
        self.set_flag(CARRY_FLAG);
    }

    fn daa(&mut self) {
        // TODO: Check if this is correct

        let mut a = self.a();
        let mut adjust = 0;
        let mut carry = self.get_flag(CARRY_FLAG);
        let half_carry = self.get_flag(HALF_FLAG);
        let subtract = self.get_flag(SUB_FLAG);

        if half_carry || (!subtract && (a & 0xF) > 9) {
            adjust |= 0x06;
        }
        if carry || (!subtract && a > 0x99) {
            adjust |= 0x60;
            carry = true;
        }

        if subtract {
            a = a.wrapping_sub(adjust);
        } else {
            a = a.wrapping_add(adjust);
        }

        self.update_flag(ZERO_FLAG, a == 0);
        self.clear_flag(HALF_FLAG);
        self.update_flag(CARRY_FLAG, carry);

        *self.a_mut() = a;
    }

    fn cpl(&mut self) {
        *self.a_mut() = !self.a();
        self.set_flag(SUB_FLAG);
        self.set_flag(HALF_FLAG);
    }

    /*******************************************/
    /*            16-bit Arithmetic            */
    /*******************************************/

    fn inc_16(&mut self, register: &mut u16) {
        *register = register.wrapping_add(1);
    }

    fn dec_16(&mut self, register: &mut u16) {
        *register = register.wrapping_sub(1);
    }

    fn add_hl(&mut self, value: u16) {
        // TODO: Check if this is correct
        let hl = self.hl.value();
        let result = hl as u32 + value as u32;

        self.clear_flag(SUB_FLAG);
        self.update_flag(HALF_FLAG, (hl & 0xFFF) + (value & 0xFFF) > 0xFFF);
        self.update_flag(CARRY_FLAG, result > 0xFFFF);

        *self.hl.value_mut() = result as u16;
    }
}
