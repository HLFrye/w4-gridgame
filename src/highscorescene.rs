use crate::scene::*;
use crate::text;
use crate::ControllerEvent;
use crate::startscene::StartScene;

struct HighScoreEntry<'a> {
    initials: &'a str,
    score: u32,
}

const DEFAULT_SCORES: [HighScoreEntry; 8] = [
    HighScoreEntry{initials: "Fry", score: 1000},
    HighScoreEntry{initials: "Mae", score: 900},
    HighScoreEntry{initials: "AMC", score: 800},
    HighScoreEntry{initials: "MET", score: 700},
    HighScoreEntry{initials: "Ill", score: 600},
    HighScoreEntry{initials: "Rob", score: 500},
    HighScoreEntry{initials: "Ske", score: 400},
    HighScoreEntry{initials: "Ren", score: 300},
];

pub struct HighScoreScene {
    done: bool,
}

impl HighScoreScene {
    pub fn new() -> HighScoreScene {
        HighScoreScene {
            done: false,
        }
    }
}

impl Scene for HighScoreScene {
    fn handle_input<'a>(&mut self, _event: ControllerEvent) {
        self.done = true;
    }

    fn render(&mut self, _framecount: u32) -> Option<Box<dyn Scene>> {

        if self.done {
            return Some(Box::new(StartScene::new()))
        }

        let mut yoff: i32 = 24;

        for score in DEFAULT_SCORES {
            text(score.initials, 36, yoff);
            text(score.score.to_string(), 92, yoff);
            yoff += 16;
        }

        return None;
    }
}