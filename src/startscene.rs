use crate::scene::*;
use crate::text;
use crate::ControllerEvent;
use crate::highscorescene::*;
use crate::maingamescene::MainGameScene;
use crate::wasm4;

pub enum Choice {
    Start,
    StageSelect,
    HighScores,
}

pub struct StartScene {
    selected: usize,
    start_pressed: bool,
}

const CHOICES: [Choice; 3] = [Choice::Start, Choice::StageSelect, Choice::HighScores];

const START_X: usize = 20;
const START_Y: usize = 120;

impl StartScene {
    pub fn new() -> StartScene {
        StartScene {
            selected: 0,
            start_pressed: false,
        }
    }
}

impl Scene for StartScene {
    fn handle_input<'a>(&mut self, event: ControllerEvent) {
        match event {
            ControllerEvent::Released(wasm4::BUTTON_UP) if self.selected > 0 => self.selected -= 1,
            ControllerEvent::Released(wasm4::BUTTON_DOWN) if self.selected < CHOICES.len() => self.selected += 1,
            ControllerEvent::Released(wasm4::BUTTON_1) | ControllerEvent::Released(wasm4::BUTTON_2) => self.start_pressed = true,
            _ => (),
        }
    }

    fn render(&mut self, framecount: u32) -> Option<Box<dyn Scene>> {
        if self.start_pressed {
            match CHOICES[self.selected] {
                Choice::Start => return Some(Box::new(MainGameScene::new(framecount, 0))),
                Choice::StageSelect => return Some(Box::new(MainGameScene::new(framecount, 0))),
                Choice::HighScores => return Some(Box::new(HighScoreScene::new())),
            }

        }

        for i in 0..CHOICES.len() {
            let yoff = i * 10;
            let x = (START_X + 16).try_into().unwrap();
            let y = (START_Y + yoff).try_into().unwrap();
            if i == self.selected {
                text(">", START_X.try_into().unwrap(), y);
            }
            match CHOICES[i] {
                Choice::Start => text("START", x, y),
                Choice::StageSelect => text("STAGE SELECT", x, y),
                Choice::HighScores => text("HIGH SCORES", x, y),
            }
        }

        return None;
    }
}