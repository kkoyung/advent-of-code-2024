pub const INPUT1: &str = "125 17";
pub const OUTPUT1: usize = 55312;

pub const INPUT2: &str = INPUT1;
pub const OUTPUT2: usize = 65601038650482; // Not provided by the puzzle

pub fn apply_rules(stone: u64) -> Vec<u64> {
    if stone == 0 {
        vec![1]
    } else if (stone.checked_ilog10().unwrap() + 1) % 2 == 0 {
        let number_of_digits = (stone.checked_ilog10().unwrap() + 1) as usize;
        let pivot = vec![10; number_of_digits/2].iter().fold(1, |mut prod, num| {
            prod *= num;
            prod
        });
        vec![stone / pivot, stone % pivot]
    } else {
        vec![stone * 2024]
    }
}
