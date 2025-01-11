use day22::*;
use std::{collections::{HashMap, HashSet}, fs};

pub fn price_mapper(secret: usize, price_map: &mut HashMap<(i8, i8, i8, i8), usize>) {
    const REPEAT: usize = 2000;
    let mut secrets = [0; REPEAT + 1];
    let mut prices = [0; REPEAT + 1];
    let mut changes = [0; REPEAT + 1];
    let mut occurance: HashSet<(i8, i8, i8, i8)> = HashSet::new();

    secrets[0] = secret;
    prices[0] = secret % 10;
    for i in 1..=REPEAT {
        secrets[i] = next(secrets[i - 1]);
        prices[i] = secrets[i] % 10;
        changes[i] = prices[i] as i8 - prices[i - 1] as i8;

        if i >= 4 {
            let key = (changes[i - 3], changes[i - 2], changes[i - 1], changes[i]);
            if !occurance.contains(&key) {
                price_map.entry((changes[i - 3], changes[i - 2], changes[i - 1], changes[i]))
                    .and_modify(|total| *total += prices[i])
                    .or_insert(prices[i]);
                occurance.insert(key);
            }
        }
    }
}

fn process(input: &str) -> usize {
    let mut price_map: HashMap<(i8, i8, i8, i8), usize> = HashMap::new();

    input
        .lines()
        .map(|text| text.parse::<usize>().unwrap())
        // .inspect(|secret| {println!("now: {}", secret)})
        .for_each(|secret| price_mapper(secret, &mut price_map));

    *price_map.values().max().unwrap()
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
    use day22::{INPUT2, OUTPUT2};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT2), OUTPUT2);
    }
}
