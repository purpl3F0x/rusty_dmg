use crate::memory::interrupt_controller::InterruptController;

use bitfield::bitfield;

bitfield! {
    #[derive(Clone, Copy)]
    struct SerialControl(u8);
    impl Debug;
    u8;
    transfer_enable, set_transfer_enable: 7;
    //clck_speed, set_clock_speed: 1;
    clock_select, set_clock_select: 0;
}

#[derive(Debug)]
pub struct Serial {
    data: u8,
    enabled: SerialControl,
    internal_divider: u8,
    transfer_count: u8,
}

impl Serial {
    pub fn new() -> Self {
        Serial {
            data: 0,
            enabled: SerialControl(0b1000_0001),
            internal_divider: 0,
            transfer_count: 0,
        }
    }

    pub fn tick(&mut self, mmu: &mut InterruptController) {
        if self.enabled.transfer_enable() {
            self.internal_divider += 1;
            self.internal_divider %= 128; // Serial runs at 8192Hz == 1Kb/s

            if self.internal_divider == 0 {
                // TODO: Implement receiving bit logic
                let recv_bit = 0;

                if self.transfer_count == 0 {
                    print!("h{:02x} ({})", self.data, self.data as char)
                }

                self.data = (self.data << 1) | (recv_bit & 0b1);
                self.transfer_count += 1;

                if self.transfer_count == 8 {
                    self.transfer_count = 0;
                    self.enabled.set_transfer_enable(false);
                    mmu.interrupt_flag.set_serial(true);
                }
            }
        }
    }

    #[inline]
    pub fn write_data(&mut self, data: u8) {
        self.data = data;
    }

    #[inline]
    pub fn read_data(&self) -> u8 {
        self.data
    }

    #[inline]
    pub fn write_control(&mut self, control: u8) {
        log::warn!("Writing serial control register: {:#04X}", self.enabled.0);
        self.enabled.0 = control | 0b0111_1110;
    }

    #[inline]
    pub fn read_control(&self) -> u8 {
        self.enabled.0
    }
}
