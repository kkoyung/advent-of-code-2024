// use day07::*;
use std::fs;

fn main() {
    println!("Part 2");

    let input = fs::read_to_string("input").expect("Unable to read file");
    println!("{}", process(input.as_str()));
}

fn process(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut colon = line.split(":");
            let test_value = colon.next().unwrap().parse::<usize>().unwrap();
            let nums_stack = colon
                .next()
                .unwrap()
                .split_whitespace()
                .map(|text| text.parse::<usize>().unwrap())
                .rev()
                .collect::<Vec<usize>>();
            (test_value, nums_stack)
        })
        .filter(|(test_value, nums_stack)| check(test_value, nums_stack))
        .map(|(text_value, _nums)| text_value)
        .sum::<usize>()
}

fn check(test_value: &usize, nums_stack: &[usize]) -> bool {
    if nums_stack.is_empty() {
        return false;
    }
    if nums_stack.len() == 1 {
        return *test_value == nums_stack[0];
    }

    // Early terminate
    if nums_stack.last().unwrap() > test_value {
        return false;
    }

    let length = nums_stack.len();
    let last_one = nums_stack[length - 1];
    let last_two = nums_stack[length - 2];

    // Try adding last two numbers
    let mut addition_stack = Vec::from(&nums_stack[0..length - 2]);
    addition_stack.push(last_one + last_two);
    if check(test_value, &addition_stack) {
        return true;
    }

    // Try multiplying last two numbers
    let mut multiplication_stack = Vec::from(&nums_stack[0..length - 2]);
    multiplication_stack.push(last_one * last_two);
    if check(test_value, &multiplication_stack) {
        return true;
    }

    // Try concatenating last two numbers
    let mut concatenation_stack = Vec::from(&nums_stack[0..length - 2]);
    let mut concatenated_number = last_one;
    // for _ in 0..last_two.to_string().len() {
    for _ in 0..last_two.checked_ilog10().unwrap() + 1 {
        concatenated_number *= 10;
    }
    concatenated_number += last_two;
    concatenation_stack.push(concatenated_number);
    if check(test_value, &concatenation_stack) {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::process;
    use day07::INPUT;

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT), 11387);
    }
}
