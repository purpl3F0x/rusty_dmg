#[cfg(target_endian = "little")]
const LOW_POSITION: isize = 0;
#[cfg(target_endian = "big")]
const LOW_POSITION: isize = 1;

#[cfg(target_endian = "little")]
const HIGH_POSITION: isize = 1;
#[cfg(target_endian = "big")]
const HIGH_POSITION: isize = 0;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Reg {
    value: u16,
}

impl Reg {
    pub fn new() -> Self {
        Reg { value: 0 }
    }

    pub fn low_mut(&mut self) -> &mut u8 {
        self.get_offset_byte(LOW_POSITION)
    }

    pub fn high_mut(&mut self) -> &mut u8 {
        self.get_offset_byte(HIGH_POSITION)
    }

    pub fn low(&self) -> u8 {
        *self.get_offset_byte_imm(LOW_POSITION)
    }

    pub fn high(&self) -> u8 {
        *self.get_offset_byte_imm(HIGH_POSITION)
    }

    pub fn value_mut(&mut self) -> &mut u16 {
        &mut self.value
    }

    pub fn value(&self) -> u16 {
        self.value
    }

    #[inline]
    fn get_offset_byte(&mut self, offset: isize) -> &mut u8 {
        unsafe {
            let ptr = (&mut self.value as *mut u16) as *mut u8;
            return &mut *(ptr.offset(offset));
        }
    }

    #[inline]
    fn get_offset_byte_imm(&self, offset: isize) -> &u8 {
        unsafe {
            let ptr = (&self.value as *const u16) as *const u8;
            return &*(ptr.offset(offset));
        }
    }
}
