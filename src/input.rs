use crate::wasm4::*;
use std::collections::HashSet;

#[derive(Hash, std::cmp::Eq)]
#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
pub enum Buttons {
    Button1,
    Button2,
    Up,
    Down,
    Left,
    Right,
}

pub enum ControllerEvent {
    Pressed(Buttons),
    Released(Buttons),
}

pub struct ControllerState {
    pressed: HashSet<Buttons>,
}

impl ControllerState {
    pub fn new() -> ControllerState {
        return ControllerState {
            pressed: HashSet::<Buttons>::new(),
        };
    }

    pub fn get_event(&mut self) -> Vec<ControllerEvent> {
        let gamepad = unsafe { *GAMEPAD1 };
        let mut status = HashSet::<Buttons>::new();
        
        if gamepad & BUTTON_UP != 0 {
            status.insert(Buttons::Up);
        }
        if gamepad & BUTTON_DOWN != 0 {
            status.insert(Buttons::Down);
        }
        if gamepad & BUTTON_LEFT != 0 {
            status.insert(Buttons::Left);
        }
        if gamepad & BUTTON_RIGHT != 0 {
            status.insert(Buttons::Right);
        }
        if gamepad & BUTTON_1 != 0 {
            status.insert(Buttons::Button1);
        }
        if gamepad & BUTTON_2 != 0 {
            status.insert(Buttons::Button2);
        }

        let mut out = Vec::<ControllerEvent>::new();
        for released in self.pressed.difference(&status) {
            out.push(ControllerEvent::Released(*released));
        }
        for pressed in status.difference(&self.pressed) {
            out.push(ControllerEvent::Pressed(*pressed));
        }

        self.pressed = status;

        return out;
    }
}