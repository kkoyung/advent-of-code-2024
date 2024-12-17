pub const INPUT1_1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
pub const OUTPUT1_1: usize = 7036;

pub const INPUT1_2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
pub const OUTPUT1_2: usize = 11048;

pub const INPUT2_1: &str = INPUT1_1;
pub const OUTPUT2_1: usize = 45;

pub const INPUT2_2: &str = INPUT1_2;
pub const OUTPUT2_2: usize = 64;

pub type Map = Vec<Vec<char>>;
pub type Position = (usize, usize);

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Head {
    pub score: usize,
    pub position: Position,
    pub direction: Direction,
    pub track: Vec<Position>,
}

pub fn front_position(position: &Position, direction: &Direction) -> Position {
    match direction {
        Direction::North => (position.0 - 1, position.1),
        Direction::East => (position.0, position.1 + 1),
        Direction::South => (position.0 + 1, position.1),
        Direction::West => (position.0, position.1 - 1),
    }
}

pub fn opposite_of(direction: &Direction) -> Direction {
    match direction {
        Direction::North => Direction::South,
        Direction::East => Direction::West,
        Direction::South => Direction::North,
        Direction::West => Direction::East,
    }
}

pub fn left_of(direction: &Direction) -> Direction {
    match direction {
        Direction::North => Direction::West,
        Direction::East => Direction::North,
        Direction::South => Direction::East,
        Direction::West => Direction::South,
    }
}

pub fn right_of(direction: &Direction) -> Direction {
    match direction {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

pub fn get_from_map(map: &Map, position: &Position) -> Option<char> {
    if position.0 < map.len() && position.1 < map[0].len() {
        Some(map[position.0][position.1])
    } else {
        None
    }
}

pub fn step_over(map: &Map, head: &Head) -> Vec<Head> {
    [
        (head.direction, 1),
        (left_of(&head.direction), 1001),
        (right_of(&head.direction), 1001),
    ]
    .into_iter()
    .filter_map(|(new_direction, additional_score)| {
        let new_position = front_position(&head.position, &new_direction);
        if get_from_map(map, &new_position) != Some('#') {
            let mut new_track = head.track.clone();
            new_track.push(new_position);
            Some(Head {
                score: head.score + additional_score,
                position: new_position,
                direction: new_direction,
                track: new_track,
            })
        } else {
            None
        }
    })
    .collect::<Vec<Head>>()
}

pub fn parse_input(input: &str) -> (Map, Position, Position) {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Map>();

    let start_position = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, value)| ((i, j), value))
        })
        .find(|(_position, character)| **character == 'S')
        .unwrap()
        .0;

    let end_position = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, value)| ((i, j), value))
        })
        .find(|(_position, character)| **character == 'E')
        .unwrap()
        .0;

    (map, start_position, end_position)
}
