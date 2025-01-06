pub const INPUT1: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
pub const OUTPUT1: usize = 0;

pub const INPUT2: &str = INPUT1;
pub const OUTPUT2: usize = 0;

#[derive(PartialEq, Eq)]
pub enum Tile {
    Wall,
    Track,
    Picoseconds(usize),
}

pub type Position = (usize, usize);
pub type Map = Vec<Vec<Tile>>;
pub type Track = Vec<Position>;

pub fn parse_input(input: &str) -> (Map, Track) {
    let index_char = input
        .lines()
        .enumerate()
        .flat_map(|(i, row)| {
            row.chars()
                .enumerate()
                .map(move |(j, character)| ((i, j), character))
        })
        .collect::<Vec<(Position, char)>>();
    let start_position = index_char
        .iter()
        .find(|(_position, character)| *character == 'S')
        .unwrap()
        .0;
    let end_position = index_char
        .iter()
        .find(|(_position, character)| *character == 'E')
        .unwrap()
        .0;

    let mut map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|tile| if tile == '#' { Tile::Wall } else { Tile::Track })
                .collect::<Vec<Tile>>()
        })
        .collect::<Map>();
    let height = map.len();
    let width = map[0].len();

    let mut track: Track = vec![start_position];
    map[start_position.0][start_position.1] = Tile::Picoseconds(0);
    while *track.last().unwrap() != end_position {
        let mut position = *track.last().unwrap();
        if position.0 > 0 && map[position.0 - 1][position.1] == Tile::Track {
            position.0 -= 1;
        } else if position.0 < height - 1 && map[position.0 + 1][position.1] == Tile::Track {
            position.0 += 1;
        } else if position.1 > 0 && map[position.0][position.1 - 1] == Tile::Track {
            position.1 -= 1;
        } else if position.1 < width - 1 && map[position.0][position.1 + 1] == Tile::Track {
            position.1 += 1;
        }
        map[position.0][position.1] = Tile::Picoseconds(track.len());
        track.push(position);
    }

    (map, track)
}

