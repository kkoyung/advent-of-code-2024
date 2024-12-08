use std::collections::HashSet;

pub const INPUT1: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
pub const OUTPUT1: usize = 41;

pub const INPUT2: &str = INPUT1;
pub const OUTPUT2: usize = 6;

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
pub struct State {
    pub position: (usize, usize),
    pub direction: Direction,
}

pub type Map = Vec<Vec<char>>;
pub type Trace = HashSet<State>;

pub fn find_starting_position(map: &Map) -> Option<(usize, usize)> {
    for (i, row) in map.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == '^' {
                return Some((i, j));
            }
        }
    }
    None
}

pub fn front(map: &Map, state: &State) -> Option<((usize, usize), char)> {
    let height = map.len();
    let width = map[0].len();

    match state.direction {
        Direction::Up => {
            if state.position.0 > 0 {
                Some((
                    (state.position.0 - 1, state.position.1),
                    map[state.position.0 - 1][state.position.1],
                ))
            } else {
                None
            }
        }
        Direction::Right => {
            if state.position.1 < width - 1 {
                Some((
                    (state.position.0, state.position.1 + 1),
                    map[state.position.0][state.position.1 + 1],
                ))
            } else {
                None
            }
        }
        Direction::Down => {
            if state.position.0 < height - 1 {
                Some((
                    (state.position.0 + 1, state.position.1),
                    map[state.position.0 + 1][state.position.1],
                ))
            } else {
                None
            }
        }
        Direction::Left => {
            if state.position.1 > 0 {
                Some((
                    (state.position.0, state.position.1 - 1),
                    map[state.position.0][state.position.1 - 1],
                ))
            } else {
                None
            }
        }
    }
}

pub fn turn_right(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

pub fn step(map: &Map, state: &State) -> Option<State> {
    if let Some((front_position, front_value)) = front(map, state) {
        if front_value == '#' {
            Some(State {
                direction: turn_right(&state.direction),
                ..*state
            })
        } else {
            Some(State {
                position: front_position,
                ..*state
            })
        }
    } else {
        None
    }
}
