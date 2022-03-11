use crate::tiles::*;
use crate::wasm4::*;
use crate::scene::*;
use crate::input::*;
use crate::ControllerEvent;
use fastrand::Rng;

fn random_board() -> Vec<u8> {
    let mut tiles = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];

    let rng = Rng::with_seed(67);

    let mut board = Vec::new();

    for i in 0..16 {
        let index = rng.usize(0..tiles.len());
        let tile = tiles[index];
        tiles.remove(index);
        board.push(tile);
    }
    return board;
}

fn count_inversions(board: &Vec<u8>) -> u32 {
    let mut result = 0;
    for i in 0..board.len() {
        for j in i..board.len() {
            if board[j] < board[i] {
                result += 1;
            }
        }
    }
    return result;
}

fn distance(board: &Vec<u8>) -> u32 {
    let board_pos = board.iter().position(|&x| x == 16).unwrap();
    let board_x = board_pos % 4;
    let board_y = board_pos / 4;

    let dist = (3 - board_x) + (3 - board_y);
    return dist as u32;
}

fn generate_board() -> Vec<u8> {
    let mut board = random_board();
    let dist = distance(&board);
    let inversions = count_inversions(&board);

    if (dist + inversions) % 2 == 1 {
        let p1 = board.iter().position(|&x| x == 15).unwrap();
        let p2 = board.iter().position(|&x| x == 14).unwrap();

        let tmp = board[p1];
        board[p1] = board[p2];
        board[p2] = tmp;
    }

    return board;
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
    frame: i32,
    moving: u8,
    move_direction: Direction,
}

impl MainGameScene {
    pub fn new() -> MainGameScene {
        MainGameScene {
            board: generate_board(),
            move_count: 0,
            show_hint: false,
            frame: 0,
            moving: 0,
            move_direction: Direction::Up,
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
        self.frame = 1;
        self.moving = self.board[index];
        self.move_direction = dir;
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

const SLIDE_FRAMES: i32 = 8;

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

        let empty_spot = self.board.iter().position(|&r| r == 16).unwrap();

        for i in 0..16 {
            let mut x: i32 = 8 + (36 * (i % 4));
            let mut y: i32 = 8 + (36 * (i / 4));
            let value = self.get_value(i);
            if value < 16 {
                if value == self.moving && self.frame > 0 && self.frame <= SLIDE_FRAMES {
                    match self.move_direction {
                        Direction::Up => y -= (SLIDE_FRAMES - self.frame) * (36 / SLIDE_FRAMES),
                        Direction::Down => y += (SLIDE_FRAMES - self.frame) * (36 / SLIDE_FRAMES),
                        Direction::Left => x -= (SLIDE_FRAMES - self.frame) * (36 / SLIDE_FRAMES),
                        Direction::Right => x += (SLIDE_FRAMES - self.frame) * (36 / SLIDE_FRAMES),
                    }

                    self.frame += 1;
                }
                blit(TILES[(value - 1) as usize], x, y, 36, 36, BLIT_2BPP);
                if self.show_hint {
                    text(value.to_string(), x, y);
                }
            }
        }
        return None;
    }
}