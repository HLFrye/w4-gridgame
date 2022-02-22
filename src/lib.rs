#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
mod tiles;
mod gamestate;
mod drawing;
mod input;
mod scene;

use wasm4::*;
use gamestate::*;
use input::*;
use scene::*;

use core::cell::RefCell;

thread_local!{
    static SCENE: RefCell<MainGameScene> = RefCell::new(MainGameScene::new());
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

    SCENE.with(|scene| {
        let mut scene = scene.borrow_mut();
        for event in controller_events {
            scene.handle_input(event);
        }

        scene.render();
    });
}

#[no_mangle]
fn start() {
    trace("Started");
}