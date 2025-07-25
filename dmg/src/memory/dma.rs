use super::RegisterTrait;
use super::MMU;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct DMA {
    source: u16,
    enabled: bool,
    offset: u8,
    pub mmu: Weak<RefCell<MMU>>,
}

impl DMA {
    pub fn new(mmu: Weak<RefCell<MMU>>) -> Self {
        DMA {
            source: 0,
            enabled: false,
            offset: 0,
            mmu,
        }
    }

    pub fn tick(&mut self) {
        if !self.enabled {
            return;
        }

        let mmu_rc = self.mmu.upgrade().unwrap();
        let src_addr = self.source | self.offset as u16;
        let dst_addr = 0xFE00 | self.offset as u16;

        let value = mmu_rc.borrow().read(src_addr);
        mmu_rc.borrow_mut().write(dst_addr, value);

        self.offset += 1;

        log::trace!(
            "DMA transfer: {:#04X} -> {:#04X} (value: {:#04X})",
            src_addr,
            dst_addr,
            value
        );

        if self.offset == 0x9F {
            self.enabled = false;
            log::debug!("DMA transfer completed");
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl RegisterTrait for DMA {
    fn read(&self, address: u16) -> u8 {
        if address == 0xFF46 {
            return (self.source >> 8) as u8;
        }

        return 0;
    }

    fn write(&mut self, address: u16, value: u8) {
        if address == 0xFF46 {
            self.source = (value as u16) << 8;
            self.offset = 0;
            self.enabled = true;

            log::debug!("DMA started from source: {:#04X}", self.source);
        } else {
            panic!(
                "Attempted to write to unsupported DMA address: {:#04X}",
                address
            );
        }
    }
}
