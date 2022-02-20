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

enum GameScene {
    Playing
}

pub struct GameState {
    board: Vec<u8>,
    move_count: u32,
    scene: GameScene,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            board: generate_board(),
            move_count: 0,
            scene: GameScene::Playing,
        }
    }

    pub fn get_value(&self, idx: i32) -> u8 {
        return self.board[idx as usize];
    }

    pub fn move_space(&mut self, dir: Direction) {
        // find the x, y of the 0
        let index = self.board.iter().position(|&r| r == 15).unwrap();

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
        let newIndex = y * 4 + x;
        self.board.swap(index, newIndex);
    }
}