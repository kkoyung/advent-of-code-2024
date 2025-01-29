// use day25::*;
use std::{fs, iter};

type Lock = [usize; 5];
type Key = [usize; 5];

fn parse_input(input: &str) -> (Vec<Lock>, Vec<Key>) {
    input.split("\n\n").fold(
        (Vec::new(), Vec::new()),
        |(mut locks, mut keys), schematic| {
            let mut lines = schematic.lines();

            if lines.next().unwrap() == "#####" {
                let lock: Lock = lines.take(5).fold([0; 5], |height, row| {
                    let mut chars = row.chars();
                    [
                        height[0] + if chars.next() == Some('#') { 1 } else { 0 },
                        height[1] + if chars.next() == Some('#') { 1 } else { 0 },
                        height[2] + if chars.next() == Some('#') { 1 } else { 0 },
                        height[3] + if chars.next() == Some('#') { 1 } else { 0 },
                        height[4] + if chars.next() == Some('#') { 1 } else { 0 },
                    ]
                });
                locks.push(lock);
            } else {
                let key: Key = lines.take(5).fold([0; 5], |height, row| {
                    let mut chars = row.chars();
                    [
                        height[0] + if chars.next() == Some('#') { 1 } else { 0 },
                        height[1] + if chars.next() == Some('#') { 1 } else { 0 },
                        height[2] + if chars.next() == Some('#') { 1 } else { 0 },
                        height[3] + if chars.next() == Some('#') { 1 } else { 0 },
                        height[4] + if chars.next() == Some('#') { 1 } else { 0 },
                    ]
                });
                keys.push(key);
            };

            (locks, keys)
        },
    )
}

fn is_fit(lock: &Lock, key: &Key) -> bool {
    iter::zip(lock, key).all(|(lock_pin, key_pin)| lock_pin + key_pin <= 5)
}

fn process(input: &str) -> usize {
    let (locks, keys) = parse_input(input);

    locks
        .iter()
        .flat_map(|lock| keys.iter().map(move |key| (lock, key)))
        .filter(|(lock, key)| is_fit(lock, key))
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
    use day25::{INPUT1, OUTPUT1};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT1), OUTPUT1);
    }
}
