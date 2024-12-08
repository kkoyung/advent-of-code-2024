pub const INPUT1: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
pub const OUTPUT1: usize = 2;

pub const INPUT2: &str = INPUT1;
pub const OUTPUT2: usize = 4;

pub fn is_safe(nums: &[i32]) -> bool {
    let is_all_increasing_gradually = nums
        .windows(2)
        .map(|w| w[1] - w[0])
        .all(|diff| (1..=3).contains(&diff));
    let is_all_decreasing_gradually = nums
        .windows(2)
        .map(|w| w[1] - w[0])
        .all(|diff| (-3..=-1).contains(&diff));

    is_all_increasing_gradually || is_all_decreasing_gradually
}
