#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
mod tiles;
mod gamestate;
mod drawing;
mod input;
mod scene;
mod startupscene;
mod startscene;
mod gamerunner;

use wasm4::*;
use input::*;
use gamerunner::*;

use core::cell::RefCell;

thread_local!{
    static RUNNER: RefCell<GameRunner> = RefCell::new(GameRunner::new());
}

#[no_mangle]
fn update() {
    RUNNER.with(|runner_ref| {
        let mut runner = runner_ref.borrow_mut();
        runner.update();
    });

}

#[no_mangle]
fn start() {
    trace("Started");
}