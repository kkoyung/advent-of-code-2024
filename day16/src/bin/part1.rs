// use day16::*;
use std::{
    collections::{BTreeSet, HashSet},
    fs,
};

type Map = Vec<Vec<char>>;

type Position = (usize, usize);

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Head {
    score: usize,
    position: Position,
    direction: Direction,
    track: Vec<Position>,
}

fn front_position(position: &Position, direction: &Direction) -> Position {
    match direction {
        Direction::North => (position.0 - 1, position.1),
        Direction::East => (position.0, position.1 + 1),
        Direction::South => (position.0 + 1, position.1),
        Direction::West => (position.0, position.1 - 1),
    }
}

fn opposite_of(direction: &Direction) -> Direction {
    match direction {
        Direction::North => Direction::South,
        Direction::East => Direction::West,
        Direction::South => Direction::North,
        Direction::West => Direction::East,
    }
}

fn left_of(direction: &Direction) -> Direction {
    match direction {
        Direction::North => Direction::West,
        Direction::East => Direction::North,
        Direction::South => Direction::East,
        Direction::West => Direction::South,
    }
}

fn right_of(direction: &Direction) -> Direction {
    match direction {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

fn get_from_map(map: &Map, position: &Position) -> Option<char> {
    if position.0 < map.len() && position.1 < map[0].len() {
        Some(map[position.0][position.1])
    } else {
        None
    }
}

fn step_over(map: &Map, head: &Head) -> Vec<Head> {
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

fn process(input: &str) -> usize {
    let (map, start_position, end_position) = parse_input(input);

    let mut heads: BTreeSet<Head> = BTreeSet::new();
    heads.insert(Head {
        score: 0,
        position: start_position,
        direction: Direction::East,
        track: vec![start_position],
    });

    let mut footprint: HashSet<(Position, Direction)> = HashSet::new();
    footprint.insert((start_position, Direction::East));
    footprint.insert((start_position, opposite_of(&Direction::East)));

    let mut finished_heads: Vec<Head> = Vec::new();

    while let Some(head) = heads.pop_first() {
        step_over(&map, &head)
            .into_iter()
            .filter(|head| {
                if !footprint.contains(&(head.position, head.direction)) {
                    footprint.insert((head.position, head.direction));
                    footprint.insert((head.position, opposite_of(&head.direction)));
                    true
                } else {
                    false
                }
            })
            .filter(|head| {
                if head.position == end_position {
                    finished_heads.push(head.clone());
                    false
                } else {
                    true
                }
            })
            .for_each(|head| {
                heads.insert(head);
            });
    }

    finished_heads.iter().map(|head| head.score).min().unwrap()
}

fn parse_input(input: &str) -> (Map, Position, Position) {
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

// =====================================================================

fn main() {
    println!("Part 1");

    let input = fs::read_to_string("input").expect("Unable to read file");
    println!("{}", process(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::process;
    use day16::{INPUT1_1, INPUT1_2, OUTPUT1_1, OUTPUT1_2};

    #[test]
    fn test_example_1() {
        assert_eq!(process(INPUT1_1), OUTPUT1_1);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(process(INPUT1_2), OUTPUT1_2);
    }
}
