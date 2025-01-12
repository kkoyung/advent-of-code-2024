use day23::*;
use std::{collections::BTreeSet, fs};

fn maximal_clique<'a>(graph: &'a Graph<'a>, seed: &'a Computer<'a>) -> BTreeSet<Computer<'a>> {
    let mut clique: BTreeSet<Computer> = BTreeSet::new();
    clique.insert(seed);

    loop {
        let addition = clique
            .iter()
            .filter_map(|&computer| graph.incident_map.get(computer))
            .map(|adjacent| {
                adjacent
                    .difference(&clique)
                    .copied()
                    .collect::<BTreeSet<Computer>>()
            })
            .fold(graph.computers.clone(), |accumulated, next| {
                accumulated.intersection(&next).copied().collect()
            });

        if addition.is_empty() {
            break;
        } else {
            clique.insert(addition.first().unwrap());
        }
    }

    clique
}

fn process(input: &str) -> String {
    // Prepare
    let graph = parse_input(input);

    let mut largest_clique = graph
        .computers
        .iter()
        .map(|computer| maximal_clique(&graph, computer))
        .max_by_key(|clique| clique.len())
        .unwrap()
        .into_iter()
        .collect::<Vec<Computer>>();
    largest_clique.sort();
    largest_clique.join(",")
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
    use day23::{INPUT2, OUTPUT2};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT2), OUTPUT2);
    }
}
