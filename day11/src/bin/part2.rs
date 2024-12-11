use day11::*;
use std::{collections::HashMap, fs};

struct StoneFreq {
    stone: u64,
    freq: usize,
}

fn deduplicate(stone_freq: &[StoneFreq]) -> Vec<StoneFreq> {
    stone_freq
        .iter()
        .fold(HashMap::new(), |mut map, stone_freq| {
            map.entry(stone_freq.stone)
                .and_modify(|freq| *freq += stone_freq.freq)
                .or_insert(stone_freq.freq);
            map
        })
        .iter()
        .map(|(&stone, &freq)| StoneFreq { stone, freq })
        .collect::<Vec<StoneFreq>>()
}

fn apply_rules_to_all(stone_freqs: &[StoneFreq]) -> Vec<StoneFreq> {
    stone_freqs
        .iter()
        .flat_map(|stone_freq| {
            apply_rules(stone_freq.stone)
                .iter()
                .map(|&stone| StoneFreq {
                    stone,
                    freq: stone_freq.freq,
                })
                .collect::<Vec<StoneFreq>>()
        })
        .collect::<Vec<StoneFreq>>()
}

fn process(input: &str) -> usize {
    let mut stone_freqs = input
        .split_whitespace()
        .map(|number_in_text| StoneFreq {
            stone: number_in_text.parse::<u64>().unwrap(),
            freq: 1,
        })
        .collect::<Vec<StoneFreq>>();

    for _ in 0..75 {
        stone_freqs = deduplicate(&apply_rules_to_all(&stone_freqs))
    }

    stone_freqs.iter().map(|stone_freq| stone_freq.freq).sum()
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
    use day11::{INPUT2, OUTPUT2};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT2), OUTPUT2);
    }
}
