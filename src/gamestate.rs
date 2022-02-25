use crate::tiles::*;
use crate::wasm4::*;
use crate::scene::*;
use crate::input::*;
use crate::ControllerEvent;

fn generate_board() -> Vec<u8> {
    return vec![2, 4, 9, 16, 6, 14, 11, 15, 8, 7, 5, 12, 13, 3, 10, 1];
}

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct MainGameScene {
    board: Vec<u8>,
    move_count: u32,
    show_hint: bool,
}

impl MainGameScene {
    pub fn new() -> MainGameScene {
        MainGameScene {
            board: generate_board(),
            move_count: 0,
            show_hint: false,
        }
    }

    pub fn get_value(&self, idx: i32) -> u8 {
        return self.board[idx as usize];
    }

    pub fn move_space(&mut self, dir: Direction) {
        // find the x, y of the 0
        let index = self.board.iter().position(|&r| r == 16).unwrap();

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
    }
}

const TILES: [&[u8]; 16] = [
    &tile1_bits,
    &tile2_bits,
    &tile3_bits,
    &tile4_bits,
    &tile5_bits,
    &tile6_bits,
    &tile7_bits,
    &tile8_bits,
    &tile9_bits,
    &tile10_bits,
    &tile11_bits,
    &tile12_bits,
    &tile13_bits,
    &tile14_bits,
    &tile15_bits,
    &tile16_bits,
];

impl Scene for MainGameScene {
    fn handle_input(&mut self, event: ControllerEvent) {
        match event {
            ControllerEvent::Pressed(Buttons::Up) => self.move_space(Direction::Down),
            ControllerEvent::Pressed(Buttons::Down) => self.move_space(Direction::Up),
            ControllerEvent::Pressed(Buttons::Left) => self.move_space(Direction::Right),
            ControllerEvent::Pressed(Buttons::Right) => self.move_space(Direction::Left),
            ControllerEvent::Pressed(Buttons::Button2) => self.show_hint = true,
            ControllerEvent::Released(Buttons::Button2) => self.show_hint = false,
            _ => (),
        }
    }

    fn render(&mut self) -> Option<Box<dyn Scene>> {
        unsafe {
            *DRAW_COLORS = 0x1234;
        }

        for i in 0..16 {
            let x: i32 = 8 + (36 * (i % 4));
            let y: i32 = 8 + (36 * (i / 4));
            let value = self.get_value(i);
            if value < 16 {
                blit(TILES[(value - 1) as usize], x, y, 36, 36, BLIT_2BPP);
                if self.show_hint {
                    text(value.to_string(), x, y);
                }
            }
        }
        return None;
    }
}