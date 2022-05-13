use crate::scene::*;
use crate::text;
use crate::ControllerEvent;
use crate::startscene::StartScene;

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
    fn handle_input<'a>(&mut self, _event: ControllerEvent) {
        self.elapsed += 10;
    }

    fn render(&mut self, _framecount: u32) -> Option<Box<dyn Scene>> {

        if self.elapsed > 120 {
            return Some(Box::new(StartScene::new()))
        }

        const YOFF: i32 = 24;

        text("  15 Puzzle Game", 8, 8+YOFF);
        text("    Created by ", 8, 16+YOFF);
        text("Harvey Frye, 2022", 8, 24+YOFF);
        text("Implemented in Rust", 8, 40+YOFF);
        text("Source Available at", 8, 56+YOFF);
        text("https://github.com", 8, 64+YOFF);
        text("/HLFrye/w4-gridgame", 8, 72+YOFF);
        text("Read more at ", 8, 88+YOFF);
        text("https://frye.codes", 8, 96+YOFF);
        text("/tags/wasm4", 8, 104+YOFF);

        self.elapsed += 1;

        return None;
    }
}