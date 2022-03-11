use crate::scene::*;
use crate::gamestate::*;
use crate::text;
use crate::ControllerEvent;

pub enum Choice {
    Start,
    StageSelect,
    HighScores,
}

pub struct StartScene {
    selected: u8,
    start_pressed: bool,
}

const CHOICES: [Choice, 3] = [Start, StageSelect, HighScores];

const START_X = 40;
const START_Y = 120;

impl StartupScene {
    pub fn new() -> StartupScene {
        StartScene {
            selected: 0,
            start_pressed: false,
        }
    }
}

impl Scene for StartupScene {
    fn handle_input(&mut self, event: ControllerEvent) {
        match event {
            ControllerEvent::Released(Buttons::Up) if self.selected > 0 => self.selected -= 1,
            ControllerEvent::Released(Buttons::Down) if self.selected < CHOICES.len() => self.selected += 1,
            ControllerEvent::Pressed(Buttons::Button1 | Buttons::Button2) => self.start_pressed = true,
        }
    }

    fn render(&mut self) -> Option<Box<dyn Scene>> {
        if self.start_pressed {
            return Some(Box::new(MainGameScene::new()))
        }

        for i in 0..CHOICES.len() {
            let yoff = i * 10;
            if i == self.selected {
                text(">", START_X, START_Y + yoff);
            }
            match CHOICES[i] {
                Start => text("START", START_X + 16, START_Y + yoff),
                StageSelect => text("STAGE SELECT", START_X + 16, START_Y + yoff),
                HighScores => text("HIGH SCORES", START_X + 16, START_Y + yoff),
            }
        }

        return None;
    }
}