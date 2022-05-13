use crate::wasm4::*;
use crate::slidepuzzle::SlidePuzzle;
use crate::tiles::ALL_LEVELS;
use crate::scene::Scene;
use crate::ControllerEvent;
use crate::startupscene::StartupScene;

const MAX_SCORE: u32 = 5000;

#[derive(PartialEq)]
pub enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

pub struct MainGameScene {
    board: SlidePuzzle,
    level: usize,
    move_count: u32,
    frame_count: u32,
    hint_count: u32,
    show_hint: bool,
    moving: u8,
    move_direction: Direction,
    game_over: bool,
    done: bool,
    animation_frame: i32,
}

impl MainGameScene {
    pub fn new(seed: u32, level: usize) -> MainGameScene {
        MainGameScene {
            board: SlidePuzzle::new(seed),
            level: level,
            move_count: 0,
            frame_count: 0,
            hint_count: 0,
            show_hint: false,
            moving: 0,
            move_direction: Direction::None,
            game_over: false,
            done: false,
            animation_frame: 0,
        }
    }

    pub fn get_value(&self, idx: usize) -> u8 {
        return self.board.get_idx(idx);
    }

    pub fn move_space(&mut self, dir: Direction) {
        // find the x, y of the 0
        let index = self.board.blank_index();

        let mut x = index % 4;
        let mut y = index / 4;

        // adjust x, y accordingly
        // verify new x, y is in range

        match dir {
            Direction::Up if y > 0 => y = y - 1,
            Direction::Down if y < 3 => y = y + 1,
            Direction::Left if x > 0 => x = x - 1,
            Direction::Right if x < 3 => x = x + 1,
            _ => (),
        }

        // swap old and new pos
        let new_index = y * 4 + x;
        self.board.swap(index, new_index);
        self.animation_frame = 1;
        self.moving = self.board.get_idx(index);
        self.move_direction = dir;
        self.move_count += 1;
    }

    pub fn calculate_score(&self) -> u32 {
        const FRAMES_PER_POINT: u32 = 60;
        const HINT_FRAMES_PER_POINT: u32 = 60;
        const POINTS_PER_MOVE: u32 = 10;

        return MAX_SCORE - (self.frame_count / FRAMES_PER_POINT) + (self.hint_count / HINT_FRAMES_PER_POINT) + (self.move_count * POINTS_PER_MOVE);
    }
}

const SLIDE_FRAMES: i32 = 8;

impl Scene for MainGameScene {
    fn handle_input<'a>(&mut self, event: ControllerEvent) {
        match event {
            ControllerEvent::Pressed(BUTTON_UP) => self.move_space(Direction::Down),
            ControllerEvent::Pressed(BUTTON_DOWN) => self.move_space(Direction::Up),
            ControllerEvent::Pressed(BUTTON_RIGHT) => self.move_space(Direction::Right),
            ControllerEvent::Pressed(BUTTON_LEFT) => self.move_space(Direction::Left),
            ControllerEvent::Pressed(BUTTON_2) => self.show_hint = true,
            ControllerEvent::Released(BUTTON_2) => self.show_hint = false,
            ControllerEvent::Released(BUTTON_1) if self.game_over => self.done = true,
            _ => (),
        }
    }

    fn render(&mut self, framecount: u32) -> Option<Box<dyn Scene>> {
        if self.done {
            trace("Finished scene, starting next scene");
            if self.level > ALL_LEVELS.len() {
                return Some(Box::new(StartupScene::new()));
            }
            return Some(Box::new(MainGameScene::new(framecount, self.level + 1)));
        }

        if self.board.is_solved() || self.game_over {
            self.game_over = true;
            trace("Game over, showing result");
            text("You Won!!", 0, 0);
            text("Your score: ", 0, 152);
            text(self.calculate_score().to_string(), 96, 152);

            for i in 0..16 {
                let x: i32 = 8 + (36 * (i % 4));
                let y: i32 = 8 + (36 * (i / 4));
                let value = self.get_value(i.try_into().unwrap());
                blit(ALL_LEVELS[self.level].tiles[(value - 1) as usize], x, y, 36, 36, BLIT_2BPP);
            }

            return None;
        }

        self.frame_count = framecount;
         
        unsafe {
            *DRAW_COLORS = 0x4321;
            *PALETTE = *ALL_LEVELS[self.level].palette;
        }

        if self.show_hint {
            self.hint_count += 1;
        }

        text("Move Count:", 30, 152);
        text(self.move_count.to_string(), 118, 152);

        text("Score: ", 40, 0);
        text(self.calculate_score().to_string(), 96, 0);

        for i in 0..16 {
            let mut x: i32 = 8 + (36 * (i % 4));
            let mut y: i32 = 8 + (36 * (i / 4));
            let value = self.get_value(i.try_into().unwrap());
            if value < 16 {
                if value == self.moving && self.animation_frame > 0 && self.animation_frame <= SLIDE_FRAMES {
                    match self.move_direction {
                        Direction::Up => y -= (SLIDE_FRAMES - self.animation_frame) * (36 / SLIDE_FRAMES),
                        Direction::Down => y += (SLIDE_FRAMES - self.animation_frame) * (36 / SLIDE_FRAMES),
                        Direction::Left => x -= (SLIDE_FRAMES - self.animation_frame) * (36 / SLIDE_FRAMES),
                        Direction::Right => x += (SLIDE_FRAMES - self.animation_frame) * (36 / SLIDE_FRAMES),
                        _ => (),
                    }

                    self.animation_frame += 1;
                }
                blit(ALL_LEVELS[self.level].tiles[(value - 1) as usize], x, y, 36, 36, BLIT_2BPP);
                if self.show_hint {
                    text(value.to_string(), x, y);
                }
            }
        }
        return None;
    }
}