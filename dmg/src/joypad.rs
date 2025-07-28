use crate::memory::InterruptController;

use std::cell::RefCell;
use std::rc::Rc;

use bitfield::bitfield;

#[derive(Debug)]
pub enum JoypadButton {
    A,
    B,
    Select,
    Start,
    Right,
    Left,
    Up,
    Down,
}

bitfield! {
    struct Buttons(u8);
    impl Debug;
    a, set_a: 0;
    b, set_b: 1;
    select, set_select: 2;
    start, set_start: 3;
}

bitfield! {
    struct Dpad(u8);
    impl Debug;
    right, set_right: 0;
    left, set_left: 1;
    up, set_up: 2;
    down, set_down: 3;
}

bitfield! {
    struct Select(u8);
    impl Debug;
    select_dpad, set_select_dpad: 4;
    select_buttons, set_select_buttons: 5;

}

#[derive(Debug)]
pub struct Joypad {
    buttons: Buttons,
    dpad: Dpad,
    select: Select,
    ic: Rc<RefCell<InterruptController>>,
}

impl Joypad {
    pub fn new(ic: Rc<RefCell<InterruptController>>) -> Self {
        Joypad {
            buttons: Buttons(0),
            dpad: Dpad(0),
            select: Select(0b0011_0000),
            ic,
        }
    }

    pub fn set_button(&mut self, button: JoypadButton, pressed: bool) {
        match button {
            JoypadButton::A | JoypadButton::B | JoypadButton::Select | JoypadButton::Start => {
                if pressed && !self.select.select_buttons() {
                    let old_state = match button {
                        JoypadButton::A => self.buttons.a(),
                        JoypadButton::B => self.buttons.b(),
                        JoypadButton::Select => self.buttons.select(),
                        JoypadButton::Start => self.buttons.start(),
                        _ => unreachable!(),
                    };

                    let rissing_edge = !old_state && pressed;
                    if rissing_edge {
                        log::debug!("Joypad button {:?} pressed", button);
                        self.ic.borrow_mut().interrupt_flag.set_joypad(rissing_edge);
                    }

                    match button {
                        JoypadButton::A => self.buttons.set_a(pressed),
                        JoypadButton::B => self.buttons.set_b(pressed),
                        JoypadButton::Select => self.buttons.set_select(pressed),
                        JoypadButton::Start => self.buttons.set_start(pressed),
                        _ => {}
                    }
                }
            }
            JoypadButton::Right | JoypadButton::Left | JoypadButton::Up | JoypadButton::Down => {
                if pressed && !self.select.select_dpad() {
                    let old_state = match button {
                        JoypadButton::Right => self.dpad.right(),
                        JoypadButton::Left => self.dpad.left(),
                        JoypadButton::Up => self.dpad.up(),
                        JoypadButton::Down => self.dpad.down(),
                        _ => unreachable!(),
                    };

                    let rissing_edge = !old_state && pressed;
                    if rissing_edge {
                        log::debug!("Joypad d-pad {:?} pressed", button);
                        self.ic.borrow_mut().interrupt_flag.set_joypad(rissing_edge);
                    }

                    match button {
                        JoypadButton::Right => self.dpad.set_right(pressed),
                        JoypadButton::Left => self.dpad.set_left(pressed),
                        JoypadButton::Up => self.dpad.set_up(pressed),
                        JoypadButton::Down => self.dpad.set_down(pressed),
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn set_buttons(&mut self, buttons: u8, dpad: u8) {
        self.buttons.0 = buttons & 0b0000_1111;
        self.dpad.0 = dpad & 0b0000_1111;

        if !self.select.select_buttons() && self.buttons.0 != 0 {
            self.ic.borrow_mut().interrupt_flag.set_joypad(true);
        }

        if !self.select.select_dpad() && self.dpad.0 != 0 {
            self.ic.borrow_mut().interrupt_flag.set_joypad(true);
        }
    }

    pub fn read(&self) -> u8 {
        let mut btn_state = 0;

        if !self.select.select_buttons() {
            btn_state |= self.buttons.0;
        }
        if !self.select.select_dpad() {
            btn_state |= self.dpad.0;
        }

        self.select.0 & !btn_state
    }

    pub fn write(&mut self, value: u8) {
        self.select.0 = value | 0b1100_1111;
    }
}
