use day14::*;
use std::fs;

fn security_factor(robots: &[Robot], space: &Vector) -> usize {
    let mut quadrant = [0, 0, 0, 0];
    for robot in robots {
        if robot.position.x < space.x / 2 && robot.position.y < space.y / 2 {
            quadrant[0] += 1;
        }
        if robot.position.x > (space.x - 1) / 2 && robot.position.y < space.y / 2 {
            quadrant[1] += 1;
        }
        if robot.position.x < space.x / 2 && robot.position.y > (space.y - 1) / 2 {
            quadrant[2] += 1;
        }
        if robot.position.x > (space.x - 1) / 2 && robot.position.y > (space.y - 1) / 2 {
            quadrant[3] += 1;
        }
    }

    quadrant.iter().product()
}

fn process(input: &str, width: i32, height: i32) -> usize {
    let mut robots = parse_input(input);

    let space = Vector {
        x: width,
        y: height,
    };

    robots
        .iter_mut()
        .for_each(|robot| robot.n_seconds(&space, 100));

    security_factor(&robots, &space)
}

// =====================================================================

fn main() {
    println!("Part 1");

    let input = fs::read_to_string("input").expect("Unable to read file");
    println!("{}", process(input.as_str(), 101, 103));
}

#[cfg(test)]
mod tests {
    use super::process;
    use day14::{HEIGHT1, INPUT1, OUTPUT1, WIDTH1};

    #[test]
    fn test_example() {
        assert_eq!(process(INPUT1, WIDTH1, HEIGHT1), OUTPUT1);
    }
}
