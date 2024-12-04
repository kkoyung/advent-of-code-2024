use std::fs;

const NORTH: u8 = 0b00000001;
const NORTH_EAST: u8 = 0b00000010;
const EAST: u8 = 0b00000100;
const SOUTH_EAST: u8 = 0b00001000;
const SOUTH: u8 = 0b00010000;
const SOUTH_WEST: u8 = 0b00100000;
const WEST: u8 = 0b01000000;
const NORTH_WEST: u8 = 0b10000000;

const TARGET: [char; 3] = ['M', 'A', 'S'];

fn first(map: &[Vec<char>], c: char) -> Vec<Vec<u8>> {
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

fn step(map: &[Vec<char>], states: &[Vec<u8>], c: char) -> Vec<Vec<u8>> {
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

fn count(states: &[Vec<u8>]) -> usize {
    let height = states.len();
    let width = states[0].len();

    let mut sum = 0;
    for i in 0..height {
        for j in 0..width {
            if states[i][j] & NORTH_WEST > 0 {
                if (i <= height - 3) && (states[i + 2][j] & SOUTH_WEST > 0) {
                    sum += 1;
                }
                if (j <= width - 3) && (states[i][j + 2] & NORTH_EAST > 0) {
                    sum += 1;
                }
            }
            if states[i][j] & NORTH_EAST > 0 {
                if (i <= height - 3) && (states[i + 2][j] & SOUTH_EAST > 0) {
                    sum += 1;
                }
                if (j >= 2) && (states[i][j - 2] & NORTH_WEST > 0) {
                    sum += 1;
                }
            }
            if states[i][j] & SOUTH_EAST > 0 {
                if (i >= 2) && (states[i - 2][j] & NORTH_EAST > 0) {
                    sum += 1;
                }
                if (j >= 2) && (states[i][j - 2] & SOUTH_WEST > 0) {
                    sum += 1;
                }
            }
            if states[i][j] & SOUTH_WEST > 0 {
                if (i >= 2) && (states[i - 2][j] & NORTH_WEST > 0) {
                    sum += 1;
                }
                if (j <= width - 3) && (states[i][j + 2] & SOUTH_EAST > 0) {
                    sum += 1;
                }
            }
        }
    }

    sum / 2
}

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");

    let mut map: Vec<Vec<char>> = Vec::new();
    for line in data.lines() {
        map.push(line.chars().collect());
    }

    let mut states: Vec<Vec<u8>> = first(&map, TARGET[0]);
    for target in &TARGET[1..] {
        states = step(&map, &states, *target);
    }

    println!("{}", count(&states));
}
