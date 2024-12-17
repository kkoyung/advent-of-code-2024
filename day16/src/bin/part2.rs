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

    let mut score_map: Vec<Vec<usize>> = vec![vec![usize::MAX - 1000; map[0].len()]; map.len()];

    while let Some(head) = heads.pop_first() {
        step_over(&map, &head)
            .into_iter()
            .filter(|head| {
                if head.score <= score_map[head.position.0][head.position.1] {
                    score_map[head.position.0][head.position.1] = head.score;
                    true
                } else {
                    head.score <= score_map[head.position.0][head.position.1] + 1000
                    // Not ideal workaround. It make this program run much longer.
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

    let best_score = finished_heads.iter().map(|head| head.score).min().unwrap();
    finished_heads
        .iter()
        .filter(|head| head.score == best_score)
        .flat_map(|head| head.track.clone())
        .collect::<HashSet<Position>>()
        .len()
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
    use day16::{INPUT2_1, INPUT2_2, OUTPUT2_1, OUTPUT2_2};

    #[test]
    fn test_example_1() {
        assert_eq!(process(INPUT2_1), OUTPUT2_1);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(process(INPUT2_2), OUTPUT2_2);
    }
}
