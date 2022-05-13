use crate::scene::*;
use crate::startupscene::*;
use crate::input::*;
use crate::RefCell;

// TODO: Implement the following scenes
// Start screen
// Choose level screen
// High score screen
// Add high score screen

pub struct GameRunner {
    controller: ControllerState,
    scene: RefCell<Box<dyn Scene>>,
    frame_count: u32,
}

impl GameRunner {
    pub fn new() -> GameRunner {
        GameRunner {
            controller: ControllerState::new(),
            scene: RefCell::new(Box::new(StartupScene::new())),
            frame_count: 0,
        }
    }

    pub fn update(&mut self) {
        self.frame_count = self.frame_count.wrapping_add(1);
        let controller_events = self.controller.get_events();
        let next_scene;
        {
            let mut scene = self.scene.borrow_mut();

            for input in controller_events {
                match input {
                    ControllerEvent::None => (),
                    x => scene.handle_input(x),
                }
            }

            next_scene = scene.render(self.frame_count);
        }

        if let Some(new_scene) = next_scene {
            self.scene = RefCell::new(new_scene);
        }
    }
}