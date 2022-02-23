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

#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

lazy_static!{
    static ref SCENE: Mutex<MainGameScene> = Mutex::new(MainGameScene::new());
    static ref CONTROLLER_STATE: Mutex<ControllerState> = Mutex::new(ControllerState::new());
}

#[no_mangle]
fn update() {
    unsafe { *DRAW_COLORS = 0x1234 }

    let mut controller_events: Vec<ControllerEvent> = Vec::<ControllerEvent>::new();

    let mut state = CONTROLLER_STATE.lock().expect("controller");
    controller_events = state.get_event();

    let mut scene = SCENE.lock().expect("scene");
    for event in controller_events {
        scene.handle_input(event);
    }

    scene.render();
}

#[no_mangle]
fn start() {
    trace("Started");
}