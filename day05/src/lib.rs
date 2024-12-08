use std::collections::HashMap;

pub const INPUT1: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
pub const OUTPUT1: usize = 143;

pub const INPUT2: &str = INPUT1;
pub const OUTPUT2: usize = 123;

pub fn is_correctly_order(update: &[u8], rules: &HashMap<u8, Vec<u8>>) -> bool {
    for i in 0..update.len() {
        if !verify_no_nums_in_list(&update[0..i], rules.get(&update[i]).unwrap_or(&vec![])) {
            return false;
        }
    }
    true
}

pub fn verify_no_nums_in_list(nums: &[u8], list: &[u8]) -> bool {
    list.iter()
        .filter(|num_in_list| nums.contains(num_in_list))
        .count()
        == 0
}
