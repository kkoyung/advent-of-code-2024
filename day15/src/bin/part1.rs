// use day15::*;
use core::panic;
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

    fn shift(position: Position, direction: &Direction) -> Position {
        match direction {
            Direction::Up => (position.0 - 1, position.1),
            Direction::Down => (position.0 + 1, position.1),
            Direction::Left => (position.0, position.1 - 1),
            Direction::Right => (position.0, position.1 + 1),
        }
    }

    fn one_step(&mut self, direction: &Direction) {
        self.check();

        // Find next empty space in direction
        let mut empty_space = Some(self.robot);
        loop {
            let next_position = Self::shift(empty_space.unwrap(), direction);
            if self.map[next_position.0][next_position.1] == 'O' {
                empty_space = Some(next_position);
                continue;
            } else if self.map[next_position.0][next_position.1] == '.' {
                empty_space = Some(next_position);
                break;
            } else if self.map[next_position.0][next_position.1] == '#' {
                empty_space = None;
                break;
            } else {
                panic!(
                    "Unknown object: {}",
                    self.map[next_position.0][next_position.1]
                );
            }
        }

        // Push boxes
        if let Some(empty_space) = empty_space {
            // Fill empty space
            if self.robot.0.abs_diff(empty_space.0) + self.robot.1.abs_diff(empty_space.1) == 1 {
                self.map[empty_space.0][empty_space.1] = '@';
            } else {
                self.map[empty_space.0][empty_space.1] = 'O';
            }

            // Move robot
            let new_robot = Self::shift(self.robot, direction);
            self.map[self.robot.0][self.robot.1] = '.';
            self.map[new_robot.0][new_robot.1] = '@';
            self.robot = new_robot;
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
                if *character == 'O' {
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
        .map(|line| line.chars().collect::<Vec<char>>())
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

    for direction in actions {
        warehouse.one_step(&direction);
    }

    warehouse.sum_of_gps()
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
    use day15::{INPUT1_LARGE, INPUT1_SMALL, OUTPUT1_LARGE, OUTPUT1_SMALL};

    #[test]
    fn test_example_small() {
        assert_eq!(process(INPUT1_SMALL), OUTPUT1_SMALL);
    }

    #[test]
    fn test_example_large() {
        assert_eq!(process(INPUT1_LARGE), OUTPUT1_LARGE);
    }
}
