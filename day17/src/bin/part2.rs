use day17::*;
use std::fs;

// The input is equivalent to the following psuedo-code:
//
// while register_a > 0 {
//     Generate one output, which depends on register_b, register_c and
//     the last 12 bits of register_a.
//
//     The value of register_a shifts to the right by 3 bits.
// }
//
// So, the first output only depends on the 1st to 12th bit of register_a,
// counted from the least significate bit. The second output only depends on
// the 4th to 15th bit of register_a, counted from the least significate bit.
// And so on.
//
// This solution works in this way.
//
// Start from a set of possible candidates, consists of all possible 12-bit
// numbers. Then, find out all possible 12 least significate bits of
// register_a that give the first correct output number, by running the
// program on all candidates in brute force way, to form a new set of
// candidates.
//
// Next, append all possible 3-bit configurations to all candidates
// to update the set of candidates. Find out all possible 15 least significate
// bits of register_a that give the first two correct output numbers, by
// running the program on all candidates in brute force way, to form a new
// set of candidates.
//
// Again, we append all possible 3-bit configurations to all candidates to
// update the set of candidates. Find out all possible 18 least significate
// bits of register_a that give the first three correct output numbers, by
// running the program on all candidates in brute force way, to form a new
// set of candidates.
//
// We repeat these steps until we obtain a set of candidates that give all
// correct output. However, some candidates may give extra output numbers.
// So, we need to filter them out before giving our final answer.

fn process(input: &str) -> u64 {
    let (_register_a, register_b, register_c, program) = parse_input(input);

    // Start from all possible 9-bit numbers
    let mut candidates: Vec<u64> = (0..512).collect();

    for length in 1..=program.len() {
        candidates = candidates
            .iter()
            .flat_map(|candidate| {
                // Append all possible 3 bits from the left of candidates
                (0..8)
                    .map(|prefix| (prefix << (6 + 3 * length)) + candidate)
                    .collect::<Vec<u64>>()
            })
            .filter(|&register_a| {
                // Check whether the first n output match the program
                let first_n_output = run(register_a, register_b, register_c, &program)
                    .iter()
                    .take(length)
                    .copied()
                    .collect::<Vec<u8>>();
                let first_n_program = program.iter().take(length).copied().collect::<Vec<u8>>();
                std::iter::zip(first_n_output, first_n_program).all(|(out, pro)| out == pro)
            })
            .collect::<Vec<u64>>();
    }

    // Remove all candidates that gives extra output (longer than the program)
    candidates = candidates
        .iter()
        .filter(|candidate| {
            let output = run(**candidate, register_b, register_c, &program);
            output.len() == program.len()
        })
        .copied()
        .collect::<Vec<u64>>();

    // Take the first valid candidate
    *candidates.first().unwrap()
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
    use day17::{INPUT2, OUTPUT2};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT2), OUTPUT2);
    }
}
