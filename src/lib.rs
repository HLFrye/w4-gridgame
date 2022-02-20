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
    static CONTROLLER_STATE: RefCell<ControllerState> = RefCell::new(ControllerState::new());
}

#[no_mangle]
fn update() {
    unsafe { *DRAW_COLORS = 0x1234 }

    let mut controller_events: Vec<ControllerEvent> = Vec::<ControllerEvent>::new();

    CONTROLLER_STATE.with(|state| {

        let mut state = state.borrow_mut();

        controller_events = state.get_event();
    });

    GAME_STATE.with(|state| {
        let stateRef = &mut state.borrow_mut();

        for event in controller_events {
            match event {
                ControllerEvent::Pressed(Buttons::Up) => stateRef.move_space(Direction::Down),
                ControllerEvent::Pressed(Buttons::Down) => stateRef.move_space(Direction::Up),
                ControllerEvent::Pressed(Buttons::Left) => stateRef.move_space(Direction::Right),
                ControllerEvent::Pressed(Buttons::Right) => stateRef.move_space(Direction::Left),
                _ => (),
            }
        }

        draw(&stateRef);
    });
}

#[no_mangle]
fn start() {
    trace("Started");
}