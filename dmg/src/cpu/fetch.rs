use super::*;
use crate::memory::RegisterTrait;

impl CPU {
    #[inline(always)]
    pub(super) fn read_byte(&mut self, addr: u16) -> u8 {
        self.tick4();
        self.mmu.borrow().read(addr)
    }

    #[inline(always)]
    pub(super) fn write_byte(&mut self, addr: u16, byte: u8) {
        self.tick4();
        self.mmu.borrow_mut().write(addr, byte);
    }

    #[inline(always)]
    pub(super) fn read_instruction(&mut self) -> u8 {
        let byte = self.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }

    #[inline(always)]
    pub(super) fn read_operand(&mut self) -> u8 {
        let byte = self.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }

    #[inline(always)]
    pub(super) fn read_word(&mut self) -> u16 {
        let low = self.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);

        let high = self.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);

        u16::from_le_bytes([low, high])
    }
}
