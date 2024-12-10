pub const INPUT1: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
pub const OUTPUT1: usize = 36;

pub const INPUT2: &str = INPUT1;
pub const OUTPUT2: usize = 81;

pub type Map = Vec<Vec<u32>>;
pub type Position = (usize, usize);

pub fn surrounding(position: &Position, height: usize, width: usize) -> Vec<Position> {
    let mut surrounding_position = Vec::new();

    // Up
    if position.0 > 0 {
        surrounding_position.push((position.0 - 1, position.1));
    }
    // Down
    if position.0 < height - 1 {
        surrounding_position.push((position.0 + 1, position.1));
    }
    // Left
    if position.1 > 0 {
        surrounding_position.push((position.0, position.1 - 1));
    }
    // Right
    if position.1 < width - 1 {
        surrounding_position.push((position.0, position.1 + 1));
    }

    surrounding_position
}

pub fn move_down(map: &Map, position: &Position) -> Vec<Position> {
    let height = map.len();
    let width = map[0].len();
    let current_level = map[position.0][position.1];

    surrounding(position, height, width)
        .into_iter()
        .filter(|position| map[position.0][position.1] == current_level - 1)
        .collect::<Vec<Position>>()
}
