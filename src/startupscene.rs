use crate::scene::*;
use crate::gamestate::*;
use crate::text;
use crate::ControllerEvent;

pub struct StartupScene {
    elapsed: u32,
}

impl StartupScene {
    pub fn new() -> StartupScene {
        StartupScene {
            elapsed: 0,
        }
    }
}

impl Scene for StartupScene {
    fn handle_input(&mut self, _event: ControllerEvent) {
        self.elapsed += 10;
    }

    fn render(&mut self) -> Option<Box<dyn Scene>> {

        if self.elapsed > 120 {
            return Some(Box::new(MainGameScene::new()))
        }

        text("15 Puzzle Game", 8, 8);
        text("Created by ", 8, 16);
        text("Harvey Frye, 2022", 8, 24);
        
        text("Implemented in Rust", 8, 40);
        
        text("Source Available at", 8, 56);
        text("https://github.com", 8, 64);
        text("/HLFrye/w4-gridgame", 8, 72);

        self.elapsed += 1;

        return None;
    }
}