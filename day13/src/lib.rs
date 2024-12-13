pub const INPUT1: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
pub const OUTPUT1: usize = 480;

pub const INPUT2: &str = INPUT1;
pub const OUTPUT2: usize = 875318608908; // Not provided by the puzzle

pub type Vector = (usize, usize);
pub struct Machine {
    pub button_a: Vector,
    pub button_b: Vector,
    pub prize: Vector,
}

pub fn parse_input(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|machine_text| {
            let mut line = machine_text.lines();

            let mut line_a = line.next().unwrap().split_whitespace();
            #[rustfmt::skip]
            let button_a = (
                line_a.nth(2).unwrap().split("+").nth(1).unwrap()
                    .split(",").next().unwrap()
                    .parse::<usize>().unwrap(),
                line_a.next().unwrap().split("+").nth(1).unwrap()
                    .parse::<usize>().unwrap(),
            );

            let mut line_b = line.next().unwrap().split_whitespace();
            #[rustfmt::skip]
            let button_b = (
                line_b.nth(2).unwrap().split("+").nth(1).unwrap()
                    .split(",").next().unwrap()
                    .parse::<usize>().unwrap(),
                line_b.next().unwrap().split("+").nth(1).unwrap()
                    .parse::<usize>() .unwrap(),
            );

            let mut line_p = line.next().unwrap().split_whitespace();
            #[rustfmt::skip]
            let prize = (
                line_p.nth(1).unwrap().split("=").nth(1).unwrap()
                    .split(",").next().unwrap()
                    .parse::<usize>() .unwrap(),
                line_p.next().unwrap().split("=").nth(1).unwrap()
                    .parse::<usize>() .unwrap(),
            );

            Machine {
                button_a,
                button_b,
                prize,
            }
        })
        .collect::<Vec<Machine>>()
}

pub fn count(machine: &Machine) -> Option<(usize, usize)> {
    // Solve two linear equations on two variables
    let a = [
        [
            machine.button_a.0 as f64,
            machine.button_b.0 as f64,
            machine.prize.0 as f64,
        ],
        [
            machine.button_a.1 as f64,
            machine.button_b.1 as f64,
            machine.prize.1 as f64,
        ],
    ];

    #[rustfmt::skip]
    let a_count = (a[0][2] * a[1][1] - a[1][2] * a[0][1])
                / (a[0][0] * a[1][1] - a[1][0] * a[0][1]);
    #[rustfmt::skip]
    let b_count = (a[0][2] * a[1][0] - a[1][2] * a[0][0])
                / (a[0][1] * a[1][0] - a[1][1] * a[0][0]);

    if a_count < 0.0 || b_count < 0.0 {
        return None;
    }
    if a_count.is_nan() || b_count.is_nan() {
        return None;
    }

    // if machine.button_a * a_count.to
    let a_count = a_count.round() as usize;
    let b_count = b_count.round() as usize;

    if machine.button_a.0 * a_count + machine.button_b.0 * b_count == machine.prize.0
        && machine.button_a.1 * a_count + machine.button_b.1 * b_count == machine.prize.1
    {
        return Some((a_count, b_count));
    }

    None
}

pub fn cost(count: (usize, usize)) -> usize {
    count.0 * 3 + count.1
}
