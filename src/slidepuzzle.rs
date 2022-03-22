use crate::boardutils::*;

// #[derive(PartialEq)]
// pub enum Direction {
//     Up,
//     Down,
//     Left,
//     Right,
// }

pub struct SlidePuzzle {
    board: Vec<u8>,
}

impl SlidePuzzle {
    pub fn new(seed: u32) -> SlidePuzzle {
        SlidePuzzle {
            board: generate_board(seed),
        }
    }

    pub fn is_solved(&self) -> bool {
        for i in 0..16 {
            if self.board[i as usize] != (i + 1) {
                return false;
            }
        }
        return true;
    }

    // pub fn move_space(&mut self, dir: Direction) {
    //     // find the x, y of the 0
    //     let index = self.board.iter().position(|&r| r == 16).unwrap();

    //     let mut x = index % 4;
    //     let mut y = index / 4;

    //     // adjust x, y accordingly
    //     // verify new x, y is in range

    //     match dir {
    //         Direction::Up if y > 0 => y = y - 1,
    //         Direction::Down if y < 3 => y = y + 1,
    //         Direction::Left if x > 0 => x = x - 1,
    //         Direction::Right if x < 3 => x = x + 1,
    //         _ => (),
    //     }

    //     // swap old and new pos
    //     let new_index = y * 4 + x;
    //     self.board.swap(index, new_index);
    // }

    // pub fn get_pt(&self, x: u8, y: u8) -> u8 {
    //     let idx = y*4 + x;
    //     return self.board[idx as usize];
    // }

    pub fn get_idx(&self, idx: usize) -> u8 {
        return self.board[idx];
    }

    pub fn swap(&mut self, x: usize, y: usize) {
        self.board.swap(x, y);
    }

    pub fn blank_index(&self) -> usize {
        return self.board.iter().position(|&r| r == 16).unwrap();
    }
}
