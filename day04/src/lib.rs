pub const INPUT1: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
pub const OUTPUT1: usize = 18;

pub const INPUT2: &str = INPUT1;
pub const OUTPUT2: usize = 9;

pub const NORTH: u8 = 0b00000001;
pub const NORTH_EAST: u8 = 0b00000010;
pub const EAST: u8 = 0b00000100;
pub const SOUTH_EAST: u8 = 0b00001000;
pub const SOUTH: u8 = 0b00010000;
pub const SOUTH_WEST: u8 = 0b00100000;
pub const WEST: u8 = 0b01000000;
pub const NORTH_WEST: u8 = 0b10000000;

pub fn first(map: &[Vec<char>], c: char) -> Vec<Vec<u8>> {
    let height = map.len();
    let width = map[0].len();
    let mut new_states: Vec<Vec<u8>> = vec![vec![0; width]; height];

    for i in 0..height {
        for j in 0..width {
            if map[i][j] == c {
                new_states[i][j] = 0b11111111;
            }
        }
    }

    new_states
}

pub fn step(map: &[Vec<char>], states: &[Vec<u8>], c: char) -> Vec<Vec<u8>> {
    let height = map.len();
    let width = map[0].len();
    let mut new_states: Vec<Vec<u8>> = vec![vec![0; width]; height];

    for i in 0..height {
        for j in 0..width {
            if map[i][j] == c {
                if i > 0 && (states[i - 1][j] & SOUTH > 0) {
                    new_states[i][j] |= SOUTH;
                }
                if i > 0 && j < width - 1 && (states[i - 1][j + 1] & SOUTH_WEST > 0) {
                    new_states[i][j] |= SOUTH_WEST;
                }
                if j < width - 1 && (states[i][j + 1] & WEST > 0) {
                    new_states[i][j] |= WEST;
                }
                if i < height - 1 && j < width - 1 && (states[i + 1][j + 1] & NORTH_WEST > 0) {
                    new_states[i][j] |= NORTH_WEST;
                }
                if i < height - 1 && (states[i + 1][j] & NORTH > 0) {
                    new_states[i][j] |= NORTH;
                }
                if i < height - 1 && j > 0 && (states[i + 1][j - 1] & NORTH_EAST > 0) {
                    new_states[i][j] |= NORTH_EAST;
                }
                if j > 0 && (states[i][j - 1] & EAST > 0) {
                    new_states[i][j] |= EAST;
                }
                if i > 0 && j > 0 && (states[i - 1][j - 1] & SOUTH_EAST > 0) {
                    new_states[i][j] |= SOUTH_EAST;
                }
            }
        }
    }

    new_states
}
