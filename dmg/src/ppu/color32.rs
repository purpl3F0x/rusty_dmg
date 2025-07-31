use std::mem::{align_of, size_of};
use std::slice;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color32 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[allow(non_snake_case)]

impl Color32 {
    pub const fn RGB(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 0xFF }
    }

    pub const fn RGBA(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

pub trait IntoRawBytes<const N: usize> {
    fn as_raw_bytes(&self) -> &[u8];
    fn as_raw_bytes_mut(&mut self) -> &mut [u8];
}

impl<const N: usize> IntoRawBytes<N> for [Color32; N] {
    fn as_raw_bytes(&self) -> &[u8] {
        assert_eq!(size_of::<Color32>(), 4);
        assert_eq!(align_of::<Color32>(), align_of::<u8>());

        unsafe { slice::from_raw_parts(self.as_ptr() as *const u8, N * 4) }
    }

    fn as_raw_bytes_mut(&mut self) -> &mut [u8] {
        assert_eq!(size_of::<Color32>(), 4);
        assert_eq!(align_of::<Color32>(), align_of::<u8>());

        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr() as *mut u8, N * 4) }
    }
}
