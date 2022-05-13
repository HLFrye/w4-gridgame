use fastrand::Rng;

fn random_board(seed: u32) -> [u8; 16] {
    let rng = Rng::with_seed(seed.into());
    let mut board = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];

    for i in 1..16 {
        let mut index = rng.usize(0..16);
        while board[index] != 0 {
            index = (index + 1) % 16;
        }
        board[index] = i;
    }
    return board;
}

fn count_inversions(board: &[u8; 16]) -> u32 {
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

fn distance(board: &[u8; 16]) -> u32 {
    let board_pos = board.iter().position(|&x| x == 16).unwrap();
    let board_x = board_pos % 4;
    let board_y = board_pos / 4;

    let dist = (3 - board_x) + (3 - board_y);
    return dist as u32;
}

pub fn generate_board(seed: u32) -> [u8; 16] {
    let mut board = random_board(seed);
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
