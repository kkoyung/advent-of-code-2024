use std::ops::BitXor;

pub const INPUT1: &str = "1
10
100
2024";
pub const OUTPUT1: usize = 37327623;

pub const INPUT2: &str = "1
2
3
2024";
pub const OUTPUT2: usize = 23;

fn mix(secret: usize, value: usize) -> usize {
    secret.bitxor(value)
}

fn prune(secret: usize) -> usize {
    secret % 16777216
}

pub fn next(mut secret: usize) -> usize {
    secret = prune(mix(secret << 6, secret));
    secret = prune(mix(secret >> 5, secret));
    secret = prune(mix(secret << 11, secret));
    secret
}
