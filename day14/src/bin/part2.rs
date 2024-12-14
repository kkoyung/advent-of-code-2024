use day14::*;
use image::{ImageBuffer, Rgb};
use std::fs;

fn generate_image(robots: &[Robot], space: &Vector, id: usize) {
    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::new(space.x as u32, space.y as u32);
    for robot in robots {
        imgbuf.put_pixel(
            robot.position.x as u32,
            robot.position.y as u32,
            Rgb([255, 255, 255]),
        );
    }
    imgbuf.save(format!("./images/{:04}.png", id)).unwrap();
}

fn process(input: &str, width: i32, height: i32) -> usize {
    let mut robots = parse_input(input);

    let space = Vector {
        x: width,
        y: height,
    };

    for i in 0..10000 {
        println!("Generating image of time = {} second(s)", { i });
        generate_image(&robots, &space, i);
        robots
            .iter_mut()
            .for_each(|robot| robot.n_seconds(&space, 1));
    }

    0
}

// =====================================================================

fn main() {
    println!("Part 2");

    let input = fs::read_to_string("input").expect("Unable to read file");
    println!("{}", process(input.as_str(), 101, 103));
}

// #[cfg(test)]
// mod tests {
//     use super::process;
//     use day14::{INPUT2, OUTPUT2};
//
//     #[test]
//     fn test_example() {
//         assert_eq!(process(INPUT2), OUTPUT2);
//     }
// }
