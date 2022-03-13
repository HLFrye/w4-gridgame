use crate::scene::*;
use crate::gamestate::*;
use crate::text;
use crate::ControllerEvent;
use crate::Buttons;

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
    fn handle_input(&mut self, event: ControllerEvent) {
        match event {
            ControllerEvent::Released(Buttons::Up) if self.selected > 0 => self.selected -= 1,
            ControllerEvent::Released(Buttons::Down) if self.selected < CHOICES.len() => self.selected += 1,
            ControllerEvent::Pressed(Buttons::Button1 | Buttons::Button2) => self.start_pressed = true,
            _ => (),
        }
    }

    fn render(&mut self, framecount: u32) -> Option<Box<dyn Scene>> {
        if self.start_pressed {
            return Some(Box::new(MainGameScene::new(framecount)))
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