use crate::memory::io_registers::*;
use crate::memory::{InterruptController, RegisterTrait};

use bitfield::bitfield;

bitfield! {
    pub struct TimerControlRegister(u8);
    impl Debug;
    clock_selection, set_clock_selection: 1, 0;
    enable, set_enable: 2;
}

#[derive(Debug)]
pub struct Timer {
    pub div: u16,
    pub tima: u16,
    pub tma: u16,
    pub tac: TimerControlRegister,
    step: u16,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            div: 0,
            tima: 0,
            tma: 0,
            tac: TimerControlRegister(0),
            step: 1,
        }
    }

    pub fn tick(&mut self, ic: &mut InterruptController) {
        self.div = self.div.wrapping_add(1);

        if self.tac.enable() {
            let overflow;
            (self.tima, overflow) = self.tima.overflowing_add(self.step);

            if overflow {
                self.tima = self.tma;
                ic.interrupt_flag.set_timer(true);
            }
        }
    }

    #[inline(always)]
    pub(crate) fn tima_divider_step(&self) -> u16 {
        match self.tac.clock_selection() & 0b11 {
            0 => 256 / 256,
            1 => 256 / 4,
            2 => 256 / 16,
            3 => 256 / 64,
            _ => unreachable!(),
        }
    }
}

impl RegisterTrait for Timer {
    fn read(&self, address: u16) -> u8 {
        match address {
            DIV => (self.div >> 8) as u8,
            TIMA => (self.tima >> 8) as u8,
            TMA => self.tma as u8,
            TAC => self.tac.0 & 0b111,
            _ => unreachable!(),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        log::trace!(
            "Timer write: address: {:#04X}, value: {:#04X}",
            address,
            value
        );

        match address {
            DIV => self.div = 0, // Writing to DIV resets it
            TIMA => self.tima = ((value as u16) << 8) & (self.tima as u16),
            TMA => self.tma = (value as u16) << 8,
            TAC => {
                self.tac.0 = value & 0b111;
                self.step = self.tima_divider_step();
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_div() {
        let mut timer = Timer::new();

        let mut ic = InterruptController::new();

        for i in 1..42 {
            for _ in 0..256 {
                timer.tick(&mut ic);
            }

            assert_eq!(timer.read(DIV), i);
        }

        timer.write(DIV, 42);
        assert_eq!(timer.read(DIV), 0);
    }

    #[test]
    fn test_tma() {
        let mut timer = Timer::new();
        let mut ic = InterruptController::new();
        timer.tac.set_enable(false);
        timer.tac.set_clock_selection(0);

        for _ in 0..1024 {
            timer.tick(&mut ic);
            assert_eq!(timer.tima, 0);
        }

        timer.write(TAC, 0b0000100);
        for _ in 0..1024 {
            timer.tick(&mut ic);
        }
        assert_eq!(timer.tima >> 8, 4);

        timer.write(TAC, 0b0000101);

        for _ in 0..16 {
            timer.tick(&mut ic);
        }
        assert_eq!(timer.tima >> 8, 8);

        timer.tma = 0xFF << 8;
        for _ in 0..(248 * 4 + 4) {
            timer.tick(&mut ic);
        }
        assert_eq!(timer.tima >> 8, 0xFF);
        for _ in 0..(4) {
            timer.tick(&mut ic);
        }
        assert_eq!(timer.tima >> 8, 0xFF);
        assert_eq!(ic.interrupt_flag.timer(), true);
    }
}
