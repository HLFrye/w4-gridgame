use fastrand::Rng;

fn random_board(seed: u32) -> Vec<u8> {
    let mut tiles = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];

    let rng = Rng::with_seed(seed.into());

    let mut board = Vec::new();

    for _i in 0..16 {
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

pub fn generate_board(seed: u32) -> Vec<u8> {
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
