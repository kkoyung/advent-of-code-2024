use std::ops::BitXor;

pub const INPUT1: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
pub const OUTPUT1: &str = "4,6,3,5,6,3,5,2,1,0";

pub const INPUT2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
pub const OUTPUT2: u64 = 117440;

pub fn combo(register_a: u64, register_b: u64, register_c: u64, oprand: u8) -> u64 {
    match oprand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => register_a,
        5 => register_b,
        6 => register_c,
        _oprand => panic!("Invalid combo: {}", _oprand),
    }
}

pub fn adv(
    register_a: &mut u64,
    register_b: &mut u64,
    register_c: &mut u64,
    oprand: u8,
    instruction_pointer: &mut usize,
) {
    let combo_oprand = combo(*register_a, *register_b, *register_c, oprand);
    let numerator = *register_a;
    let denominator = vec![2; combo_oprand as usize].iter().product::<u64>();
    *register_a = numerator / denominator;
    *instruction_pointer += 2;
}

pub fn bxl(register_b: &mut u64, oprand: u8, instruction_pointer: &mut usize) {
    let literal_oprand = oprand as u64;
    *register_b = register_b.bitxor(literal_oprand);
    *instruction_pointer += 2;
}

pub fn bst(
    register_a: &mut u64,
    register_b: &mut u64,
    register_c: &mut u64,
    oprand: u8,
    instruction_pointer: &mut usize,
) {
    let combo_oprand = combo(*register_a, *register_b, *register_c, oprand);
    *register_b = combo_oprand.rem_euclid(8);
    *instruction_pointer += 2;
}

pub fn jnz(register_a: &mut u64, oprand: u8, instruction_pointer: &mut usize) {
    if *register_a == 0 {
        *instruction_pointer += 2;
    } else {
        let literal_oprand = oprand as u64;
        *instruction_pointer = literal_oprand as usize;
    }
}

pub fn bxc(register_b: &mut u64, register_c: &mut u64, instruction_pointer: &mut usize) {
    *register_b = register_b.bitxor(*register_c);
    *instruction_pointer += 2;
}

pub fn out(
    register_a: &mut u64,
    register_b: &mut u64,
    register_c: &mut u64,
    oprand: u8,
    instruction_pointer: &mut usize,
) -> u8 {
    let combo_oprand = combo(*register_a, *register_b, *register_c, oprand);
    *instruction_pointer += 2;
    combo_oprand.rem_euclid(8) as u8
}

pub fn bdv(
    register_a: &mut u64,
    register_b: &mut u64,
    register_c: &mut u64,
    oprand: u8,
    instruction_pointer: &mut usize,
) {
    let combo_oprand = combo(*register_a, *register_b, *register_c, oprand);
    let numerator = *register_a;
    let denominator = vec![2; combo_oprand as usize].iter().product::<u64>();
    *register_b = numerator / denominator;
    *instruction_pointer += 2;
}

pub fn cdv(
    register_a: &mut u64,
    register_b: &mut u64,
    register_c: &mut u64,
    oprand: u8,
    instruction_pointer: &mut usize,
) {
    let combo_oprand = combo(*register_a, *register_b, *register_c, oprand);
    let numerator = *register_a;
    let denominator = vec![2; combo_oprand as usize].iter().product::<u64>();
    *register_c = numerator / denominator;
    *instruction_pointer += 2;
}

pub fn run(
    mut register_a: u64,
    mut register_b: u64,
    mut register_c: u64,
    program: &[u8],
) -> Vec<u8> {
    let mut instruction_pointer = 0;
    let mut output: Vec<u8> = Vec::new();

    while instruction_pointer < program.len() {
        let opcode = program[instruction_pointer];
        let oprand = program[instruction_pointer + 1];

        match opcode {
            0 => adv(
                &mut register_a,
                &mut register_b,
                &mut register_c,
                oprand,
                &mut instruction_pointer,
            ),
            1 => bxl(&mut register_b, oprand, &mut instruction_pointer),
            2 => bst(
                &mut register_a,
                &mut register_b,
                &mut register_c,
                oprand,
                &mut instruction_pointer,
            ),
            3 => jnz(&mut register_a, oprand, &mut instruction_pointer),
            4 => bxc(&mut register_b, &mut register_c, &mut instruction_pointer),
            5 => output.push(out(
                &mut register_a,
                &mut register_b,
                &mut register_c,
                oprand,
                &mut instruction_pointer,
            )),
            6 => bdv(
                &mut register_a,
                &mut register_b,
                &mut register_c,
                oprand,
                &mut instruction_pointer,
            ),
            7 => cdv(
                &mut register_a,
                &mut register_b,
                &mut register_c,
                oprand,
                &mut instruction_pointer,
            ),
            _ => {}
        }
    }

    output
}

pub fn parse_input(input: &str) -> (u64, u64, u64, Vec<u8>) {
    let mut lines = input.lines();
    let register_a = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse::<u64>()
        .unwrap();
    let register_b = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse::<u64>()
        .unwrap();
    let register_c = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse::<u64>()
        .unwrap();
    lines.next();
    let program = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|text| text.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    (register_a, register_b, register_c, program)
}
