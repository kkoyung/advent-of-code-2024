use day16::*;
use std::{
    collections::{BTreeSet, HashSet},
    fs,
};

fn process(input: &str) -> usize {
    let (map, start_position, end_position) = parse_input(input);

    let mut heads: BTreeSet<Head> = BTreeSet::new();
    heads.insert(Head {
        score: 0,
        position: start_position,
        direction: Direction::East,
        track: vec![start_position],
    });
    let mut finished_heads: Vec<Head> = Vec::new();

    let mut footprint: HashSet<(Position, Direction)> = HashSet::new();
    footprint.insert((start_position, Direction::East));

    while let Some(head) = heads.pop_first() {
        step_over(&map, &head)
            .into_iter()
            .filter(|head| {
                if !footprint.contains(&(head.position, opposite_of(&head.direction))) {
                    footprint.insert((head.position, head.direction));
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
