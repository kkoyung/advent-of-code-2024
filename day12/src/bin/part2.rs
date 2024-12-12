use day12::*;
use std::fs;

#[derive(PartialEq, Eq)]
enum Pattern {
    Left,
    Right,
    Both,
    None,
}

fn segment(line1: &[char], line2: &[char]) -> usize {
    let mut patterns = std::iter::zip(line1, line2)
        .map(|(&left, &right)| {
            if left != '.' && right == '.' {
                Pattern::Left
            } else if left == '.' && right != '.' {
                Pattern::Right
            } else if left != '.' && right != '.' {
                Pattern::Both
            } else {
                Pattern::None
            }
        })
        .collect::<Vec<Pattern>>();

    let mut count = 0;
    while let Some(pattern) = patterns.pop() {
        while patterns.last() == Some(&pattern) {
            patterns.pop();
        }
        if pattern == Pattern::Left || pattern == Pattern::Right {
            count += 1;
        }
    }

    count
}

fn side(garden: &Garden, map: &Map) -> usize {
    let height = map.len();
    let width = map[0].len();
    let mut count = 0;

    let mut label_map = vec![vec!['.'; width]; height];
    garden
        .positions
        .iter()
        .for_each(|position| label_map[position.0][position.1] = garden.label);

    // Horizontal
    let mut rows = label_map.clone();
    rows.insert(0, vec!['.'; width]);
    rows.push(vec!['.'; width]);
    count += rows
        .windows(2)
        .map(|lines| segment(&lines[0], &lines[1]))
        .sum::<usize>();

    // Vertical
    let mut cols = (0..width)
        .map(|j| (0..height).map(|i| label_map[i][j]).collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    cols.insert(0, vec!['.'; height]);
    cols.push(vec!['.'; height]);
    count += rows
        .windows(2)
        .map(|lines| segment(&lines[0], &lines[1]))
        .sum::<usize>();

    count
}

fn process(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Map>();

    let gardens = find_gardens(&mut map.clone());

    gardens
        .iter()
        .map(|garden| area(garden) * side(garden, &map))
        .sum()
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
    use day12::*;

    #[test]
    fn test_example_1() {
        assert_eq!(process(INPUT2_1), OUTPUT2_1);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(process(INPUT2_2), OUTPUT2_2);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(process(INPUT2_3), OUTPUT2_3);
    }

    #[test]
    fn test_example_4() {
        assert_eq!(process(INPUT2_4), OUTPUT2_4);
    }

    #[test]
    fn test_example_5() {
        assert_eq!(process(INPUT2_5), OUTPUT2_5);
    }
}
