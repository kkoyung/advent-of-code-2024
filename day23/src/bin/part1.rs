use day23::*;
use std::fs;

fn process(input: &str) -> usize {
    // Prepare
    let graph = parse_input(input);

    // Compute triples
    let mut triples = graph
        .computers
        .iter()
        .flat_map(|computer| {
            let neighbours = graph.incident_map.get(computer).unwrap();
            neighbours.iter().enumerate().flat_map(|(i, left)| {
                neighbours
                    .iter()
                    .skip(i + 1)
                    .map(move |right| (left, right))
                    .filter(|(left, right)| {
                        if left < right {
                            graph.connections.contains(&(left, right))
                        } else {
                            graph.connections.contains(&(right, left))
                        }
                    })
                    .map(|(&left, &right)| {
                        let mut triple = [computer, left, right];
                        triple.sort();
                        (triple[0], triple[1], triple[2])
                    })
            })
        })
        .collect::<Vec<Triple>>();
    triples.sort();
    triples.dedup();

    // Filter triples
    triples
        .iter()
        .filter(|triple| {
            triple.0.starts_with("t") || triple.1.starts_with("t") || triple.2.starts_with("t")
        })
        .count()
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
    use day23::{INPUT1, OUTPUT1};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT1), OUTPUT1);
    }
}
