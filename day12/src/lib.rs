use std::collections::{BTreeSet, HashSet};

pub const INPUT1_1: &str = "AAAA
BBCD
BBCC
EEEC";
pub const OUTPUT1_1: usize = 140;

pub const INPUT1_2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
pub const OUTPUT1_2: usize = 772;

pub const INPUT1_3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
pub const OUTPUT1_3: usize = 1930;

pub const INPUT2_1: &str = INPUT1_1;
pub const OUTPUT2_1: usize = 80;

pub const INPUT2_2: &str = INPUT1_2;
pub const OUTPUT2_2: usize = 436;

pub const INPUT2_3: &str = INPUT1_3;
pub const OUTPUT2_3: usize = 1206;

pub const INPUT2_4: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
pub const OUTPUT2_4: usize = 236;

pub const INPUT2_5: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
pub const OUTPUT2_5: usize = 368;

pub type Map = Vec<Vec<char>>;
pub type Position = (usize, usize);

pub struct Garden {
    pub positions: HashSet<Position>,
    pub label: char,
}

pub fn next_region_seed(map: &Map) -> Option<(Position, char)> {
    map.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, label)| ((i, j), *label))
        })
        .find(|(_, label)| *label != '.')
}

pub fn surrounding(height: usize, width: usize, position: &Position) -> Vec<Position> {
    let mut surrounding_position: Vec<Position> = Vec::with_capacity(4);
    if position.0 > 0 {
        surrounding_position.push((position.0 - 1, position.1));
    }
    if position.0 < height - 1 {
        surrounding_position.push((position.0 + 1, position.1));
    }
    if position.1 > 0 {
        surrounding_position.push((position.0, position.1 - 1));
    }
    if position.1 < width - 1 {
        surrounding_position.push((position.0, position.1 + 1));
    }
    surrounding_position
}

pub fn find_gardens(map: &mut Map) -> Vec<Garden> {
    let height = map.len();
    let width = map[0].len();

    let mut gardens: Vec<Garden> = Vec::new();
    while let Some((position, label)) = next_region_seed(map) {
        let mut positions: HashSet<Position> = HashSet::new();
        let mut heads: BTreeSet<Position> = BTreeSet::new();
        heads.insert(position);

        while let Some(pos) = heads.pop_first() {
            positions.insert(pos);
            map[pos.0][pos.1] = '.';
            heads.extend(
                surrounding(height, width, &pos)
                    .iter()
                    .filter(|pos| map[pos.0][pos.1] == label),
            )
        }

        gardens.push(Garden { positions, label });
    }

    gardens
}

pub fn area(garden: &Garden) -> usize {
    garden.positions.len()
}
