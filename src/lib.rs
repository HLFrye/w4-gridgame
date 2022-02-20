#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
mod tiles;
mod gamestate;
mod drawing;
mod input;

use wasm4::*;
use gamestate::*;
use drawing::*;
use input::*;

use core::cell::RefCell;
use std::borrow::*;

thread_local!{
    static GAME_STATE: RefCell<GameState> = RefCell::new(GameState::new());
    static CONTROLLER_STATE: RefCell<bool> = RefCell::new(false);
}

#[no_mangle]
fn update() {
    unsafe { *DRAW_COLORS = 0x1234 }

    let gamepad = unsafe { *GAMEPAD1 };
    let mut direction: Option<Direction> = None;

    CONTROLLER_STATE.with(|state| {
        let mut state = state.borrow_mut();
        match gamepad {
            x if x & BUTTON_UP != 0 => direction = Some(Direction::Down),
            x if x & BUTTON_DOWN != 0 => direction = Some(Direction::Up),
            x if x & BUTTON_LEFT != 0 => direction = Some(Direction::Right),
            x if x & BUTTON_RIGHT != 0 => direction = Some(Direction::Left),
            _ => direction = None,
        }
        if direction != None && *state != true {
            trace("Button down");
            *state = true;
        }
        else if direction != None && *state == true {
            direction = None;
        }
        else if direction == None && *state == true {
            trace("Button up");
            *state = false;
            direction = None;
        }
    });

    GAME_STATE.with(|state| {
        let mut stateRef = &mut state.borrow_mut();

        if direction != None {
            trace("Moving");
            stateRef.move_space(direction.unwrap());
        }

        draw(&stateRef);
    });
}

#[no_mangle]
fn start() {
    trace("Started");
}