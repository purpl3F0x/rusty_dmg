#[cfg(target_endian = "little")]
const LOW_POSITION: isize = 0;
#[cfg(target_endian = "big")]
const LOW_POSITION: isize = 1;

#[cfg(target_endian = "little")]
const HIGH_POSITION: isize = 1;
#[cfg(target_endian = "big")]
const HIGH_POSITION: isize = 0;

// use std::ops::AddAssign;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Reg {
    value: u16,
}

// impl Default for Reg {
//     #[inline]
//     fn default() -> Reg {
//         Reg { value: 0 }
//     }
// }

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

    pub fn low(&mut self) -> u8 {
        *self.get_offset_byte(LOW_POSITION)
    }

    pub fn high(&mut self) -> u8 {
        *self.get_offset_byte(HIGH_POSITION)
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
}