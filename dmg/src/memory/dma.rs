use super::RegisterTrait;
use super::MMU;

use std::cell::RefCell;
use std::rc::Weak;

#[derive(Debug)]
pub struct DMA {
    source: u8,
    next_addr: u16,
    enabled: bool,
    starting: bool,
    pub mmu: Weak<RefCell<MMU>>,
}

impl DMA {
    pub fn new(mmu: Weak<RefCell<MMU>>) -> Self {
        DMA {
            source: 0,
            next_addr: 0,
            starting: false,
            enabled: false,
            mmu,
        }
    }

    pub fn tick(&mut self) {
        if self.enabled {
            let mmu_rc = self.mmu.upgrade().unwrap();
            let src_addr = self.next_addr;
            let dst_addr = 0xFE00 | (self.next_addr & 0xFF);

            let value = mmu_rc.borrow().read(src_addr);
            mmu_rc.borrow_mut().write(dst_addr, value);

            self.next_addr += 1;

            if src_addr as u8 > 0x9F {
                self.enabled = false;
            }
        }

        if self.starting {
            self.enabled = true;
            self.starting = false;
            self.next_addr = (self.source as u16) << 8;
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl RegisterTrait for DMA {
    fn read(&self, _: u16) -> u8 {
        return self.source;
    }

    fn write(&mut self, _: u16, value: u8) {
        self.source = value;
        self.next_addr = 0;
        self.starting = true;
    }
}
