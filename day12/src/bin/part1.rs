use day12::*;
use std::fs;

fn perimeter(garden: &Garden, map: &Map) -> usize {
    let height = map.len();
    let width = map[0].len();
    garden
        .positions
        .iter()
        .map(|position| {
            4 - surrounding(height, width, position)
                .iter()
                .filter(|position| garden.positions.contains(position))
                .count()
        })
        .sum()
}

fn process(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Map>();

    let gardens = find_gardens(&mut map.clone());

    gardens
        .iter()
        .map(|garden| area(garden) * perimeter(garden, &map))
        .sum()
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
    use day12::*;

    #[test]
    fn test_example_1() {
        assert_eq!(process(INPUT1_1), OUTPUT1_1);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(process(INPUT1_2), OUTPUT1_2);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(process(INPUT1_3), OUTPUT1_3);
    }
}
