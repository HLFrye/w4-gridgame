use crate::scene::*;
use crate::startupscene::*;
use crate::ControllerState;
use crate::RefCell;

pub struct GameRunner {
    controller: ControllerState,
    scene: RefCell<Box<dyn Scene>>,
}

impl GameRunner {
    pub fn new() -> GameRunner {
        GameRunner {
            controller: ControllerState::new(),
            scene: RefCell::new(Box::new(StartupScene::new())),
        }
    }

    pub fn update(&mut self) {
        let controller_events = self.controller.get_event();
        let next_scene;
        {
            let mut scene = self.scene.borrow_mut();

            for event in controller_events {
                scene.handle_input(event);
            }

            next_scene = scene.render();
        }
        
        if let Some(new_scene) = next_scene {
            self.scene = RefCell::new(new_scene);
        }
    }
}