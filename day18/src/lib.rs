use std::collections::BTreeSet;

pub const INPUT1: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
pub const MAX_COORDINATE1: usize = 6;
pub const NUMBER_OF_BYTES1: usize = 12;
pub const OUTPUT1: usize = 22;

pub const INPUT2: &str = INPUT1;
pub const MAX_COORDINATE2: usize = MAX_COORDINATE1;
pub const NUMBER_OF_BYTES2: usize = NUMBER_OF_BYTES1;
pub const OUTPUT2: &str = "6,1";

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn surrounding(&self, max_coordinate: usize) -> Vec<Position> {
        let mut positions = Vec::new();
        if self.x > 0 {
            positions.push(Position {
                x: self.x - 1,
                y: self.y,
            });
        }
        if self.x < max_coordinate {
            positions.push(Position {
                x: self.x + 1,
                y: self.y,
            });
        }
        if self.y > 0 {
            positions.push(Position {
                x: self.x,
                y: self.y - 1,
            });
        }
        if self.y < max_coordinate {
            positions.push(Position {
                x: self.x,
                y: self.y + 1,
            });
        }

        positions
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Space {
    Free,
    Corrupted,
    Step(usize),
}

pub type Map = Vec<Vec<Space>>;

pub fn run(max_coordinate: usize, map: &mut Map, step_table: &mut Vec<BTreeSet<Position>>) -> bool {
    loop {
        let previous_steps = step_table.last().unwrap();
        let previous_step_count = step_table.len() - 1;

        let new_steps = previous_steps
            .iter()
            .flat_map(|position| {
                let next_steps = position
                    .surrounding(max_coordinate)
                    .into_iter()
                    .filter(|position| map[position.x][position.y] == Space::Free)
                    .collect::<Vec<Position>>();
                next_steps.iter().for_each(|position| {
                    map[position.x][position.y] = Space::Step(previous_step_count + 1);
                });
                next_steps
            })
            .collect::<BTreeSet<Position>>();

        if new_steps.contains(&Position {
            x: max_coordinate,
            y: max_coordinate,
        }) {
            step_table.push(new_steps);
            return true; // Reach exit
        } else if new_steps.is_empty() {
            return false; // Cannot reach exit
        } else {
            step_table.push(new_steps); // Continue
        }
    }
}
