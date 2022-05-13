use crate::wasm4::*;

pub enum ControllerEvent {
    None,
    Pressed(u8),
    Released(u8),
}

pub struct ControllerState {
    last: u8,
}

impl ControllerState {
    pub fn new() -> ControllerState {
        return ControllerState { 
            last: 0,
        }
    }

    pub fn get_events(&mut self) -> [ControllerEvent; 8] {
        let gamepad = unsafe { *GAMEPAD1 };

        let mut output = [
            ControllerEvent::None, ControllerEvent::None, ControllerEvent::None, ControllerEvent::None, 
            ControllerEvent::None, ControllerEvent::None, ControllerEvent::None, ControllerEvent::None,
        ];

        for i in 0..8 {
            let flag: u8 = 1 << i;
            if (gamepad & flag) != 0 {
                if (self.last & flag) == 0 {
                    output[i] = ControllerEvent::Pressed(flag);
                }
            }

            if (self.last & flag) != 0 {
                if (gamepad & flag) == 0 {
                    output[i] = ControllerEvent::Released(flag);
                }
            }
        }
        self.last = gamepad;

        return output;
    }
}
