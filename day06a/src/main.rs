use std::fs;

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
struct State {
    position: (usize, usize),
    direction: Direction,
}

type Map = Vec<Vec<char>>;

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");

    let mut map = data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Map>();

    let mut state = State {
        position: find_starting_position(&map).unwrap(),
        direction: Direction::Up,
    };

    // Start moving
    loop {
        map[state.position.0][state.position.1] = 'X';

        if let Some(new_state) = step(&map, &state) {
            state = new_state;
        } else {
            break;
        }
    }

    let sum = map
        .iter()
        .flatten()
        .filter(|&&value| value == 'X')
        .count();

    println!("{}", sum);
}

fn find_starting_position(map: &Map) -> Option<(usize, usize)> {
    for (i, row) in map.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == '^' {
                return Some((i, j));
            }
        }
    }
    None
}

fn front(map: &Map, state: &State) -> Option<((usize, usize), char)> {
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

fn turn_right(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn step(map: &Map, state: &State) -> Option<State> {
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
