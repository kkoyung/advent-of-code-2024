use core::panic;
// use day15::*;
use std::fs;

type Map = Vec<Vec<char>>;
type Position = (usize, usize);
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Warehouse {
    map: Map,
    robot: Position,
}

impl Warehouse {
    fn check(&self) {
        if self.map[self.robot.0][self.robot.1] != '@' {
            panic!("Inconsistent warehouse");
        }
    }

    fn shift(position: &Position, direction: &Direction) -> Position {
        match direction {
            Direction::Up => (position.0 - 1, position.1),
            Direction::Down => (position.0 + 1, position.1),
            Direction::Left => (position.0, position.1 - 1),
            Direction::Right => (position.0, position.1 + 1),
        }
    }

    fn one_step(&mut self, direction: &Direction) {
        self.check();

        let mut boxes: Vec<Position> = Vec::new(); // collection of boxes to be moved
        let mut fronts: Vec<Position> = vec![self.robot]; // front position of boxes

        let boxes = loop {
            if fronts.iter().all(|front| {
                let next_position = Self::shift(front, direction);
                self.map[next_position.0][next_position.1] == '.'
            }) {
                // Let move these boxes
                break Some(boxes);
            } else if fronts.iter().any(|front| {
                let next_position = Self::shift(front, direction);
                self.map[next_position.0][next_position.1] == '#'
            }) {
                // Cannot move
                break None;
            } else {
                // Update boxes and fronts
                fronts = fronts
                    .iter()
                    .flat_map(|front| {
                        let next_position = Self::shift(front, direction);
                        match direction {
                            Direction::Up | Direction::Down => {
                                if self.map[next_position.0][next_position.1] == '[' {
                                    boxes.push((next_position.0, next_position.1));
                                    vec![
                                        next_position,
                                        Self::shift(&next_position, &Direction::Right),
                                    ]
                                } else if self.map[next_position.0][next_position.1] == ']' {
                                    boxes.push((next_position.0, next_position.1 - 1));
                                    vec![
                                        next_position,
                                        Self::shift(&next_position, &Direction::Left),
                                    ]
                                } else {
                                    vec![*front]
                                }
                            }
                            Direction::Left => {
                                if self.map[next_position.0][next_position.1] == ']' {
                                    boxes.push((next_position.0, next_position.1 - 1));
                                    vec![Self::shift(&next_position, direction)]
                                } else {
                                    vec![*front]
                                }
                            }
                            Direction::Right => {
                                if self.map[next_position.0][next_position.1] == '[' {
                                    boxes.push((next_position.0, next_position.1));
                                    vec![Self::shift(&next_position, direction)]
                                } else {
                                    vec![*front]
                                }
                            }
                        }
                    })
                    .collect::<Vec<Position>>();
            }
        };

        if let Some(boxes) = boxes {
            // Erase object to be moved
            for position in &boxes {
                self.map[position.0][position.1] = '.';
                self.map[position.0][position.1 + 1] = '.';
            }
            self.map[self.robot.0][self.robot.1] = '.';

            // Add back objects in new positions
            for position in &boxes {
                let new_position = Self::shift(position, direction);
                self.map[new_position.0][new_position.1] = '[';
                self.map[new_position.0][new_position.1 + 1] = ']';
            }
            self.robot = Self::shift(&self.robot, direction);
            self.map[self.robot.0][self.robot.1] = '@';
        }
    }

    fn sum_of_gps(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(j, value)| ((i, j), value))
            })
            .filter_map(|(position, character)| {
                if *character == '[' {
                    Some(position.0 * 100 + position.1)
                } else {
                    None
                }
            })
            .sum::<usize>()
    }
}

fn process(input: &str) -> usize {
    let empty_line = input.lines().position(|line| line.is_empty()).unwrap();

    let map = input
        .lines()
        .take(empty_line)
        .map(|line| {
            line.chars()
                .flat_map(|character| {
                    if character == '#' {
                        ['#', '#']
                    } else if character == 'O' {
                        ['[', ']']
                    } else if character == '.' {
                        ['.', '.']
                    } else if character == '@' {
                        ['@', '.']
                    } else {
                        panic!("Unknown character: {}", character)
                    }
                })
                .collect::<Vec<char>>()
        })
        .collect::<Map>();

    let robot = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, value)| ((i, j), value))
        })
        .find(|(_position, value)| **value == '@')
        .unwrap()
        .0;

    let mut warehouse = Warehouse { map, robot };

    let actions = input
        .lines()
        .skip(empty_line + 1)
        .flat_map(|line| line.chars())
        .map(|character| {
            if character == '^' {
                Direction::Up
            } else if character == 'v' {
                Direction::Down
            } else if character == '<' {
                Direction::Left
            } else if character == '>' {
                Direction::Right
            } else {
                panic!("Unknown action: {}", character);
            }
        })
        .collect::<Vec<Direction>>();

    for direction in &actions {
        warehouse.one_step(direction);
    }

    warehouse.sum_of_gps()
}

// =====================================================================

fn main() {
    println!("Part 2");

    let input = fs::read_to_string("input").expect("Unable to read file");
    println!("{}", process(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::process;
    use day15::{INPUT2_LARGE, OUTPUT2_LARGE};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT2_LARGE), OUTPUT2_LARGE);
    }
}
