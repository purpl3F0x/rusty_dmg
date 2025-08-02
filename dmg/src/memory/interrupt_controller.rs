use bitfield::bitfield;

bitfield! {
    #[derive(Copy, Clone)]
    pub struct InterruptsRegister(u8);
    impl Debug;
    impl BitAnd;
    pub joypad, set_joypad: 4;
    pub serial, set_serial: 3;
    pub timer, set_timer: 2;
    pub lcd, set_lcd: 1;
    pub vblank, set_vblank: 0;
}

#[derive(Debug, Clone)]
pub struct InterruptController {
    pub interrupt_enable: InterruptsRegister,
    pub interrupt_flag: InterruptsRegister,
}

impl InterruptController {
    pub fn new() -> Self {
        InterruptController {
            interrupt_enable: InterruptsRegister(0b1110_0000),
            interrupt_flag: InterruptsRegister(0b1110_0000),
        }
    }

    pub fn reset(&mut self) {
        self.interrupt_enable = InterruptsRegister(0);
        self.interrupt_flag = InterruptsRegister(0);
    }
}
