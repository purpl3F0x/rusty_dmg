use super::RegisterTrait;

#[derive(Debug)]
pub struct DMA {
    source: u8,
    next_addr: u16,
    enabled: bool,
    starting: bool,
    starting_sync: bool,
}

impl DMA {
    pub fn new() -> Self {
        DMA {
            source: 0,
            next_addr: 0,
            enabled: false,
            starting: false,
            starting_sync: false,
        }
    }

    #[inline(always)]
    pub fn tick(&mut self) -> Option<u16> {
        let mut ret = None;

        if self.starting_sync {
            self.enabled = true;
            self.starting = false;
            self.next_addr = (self.source as u16) << 8;
        }

        self.starting_sync = self.starting;
        self.starting = false;

        if self.enabled {
            let src_addr = self.next_addr;

            ret = Some(src_addr);

            self.next_addr += 1;

            if src_addr as u8 > 0x9F {
                self.enabled = false;
                ret = None;
            }
        }

        ret
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
