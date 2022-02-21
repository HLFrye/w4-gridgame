use crate::wasm4::*;
use crate::tiles::*;
use crate::gamestate::*;

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

pub fn draw(state: &GameState) {
    for i in 0..16 {
        let x: i32 = 8 + (36 * (i % 4));
        let y: i32 = 8 + (36 * (i / 4));
        let value = state.get_value(i);
        if value < 16 {
            blit(TILES[value as usize], x, y, 36, 36, BLIT_2BPP);
        }
    }
}