pub const INPUT1: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"; // position=(x,y) velocity=(x,y)
pub const WIDTH1: i32 = 11; // x-axis
pub const HEIGHT1: i32 = 7; // y-axis
pub const OUTPUT1: usize = 0;

// pub const INPUT2: &str = INPUT1;
// pub const OUTPUT2: usize = 0;

use regex::Regex;

pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {
    pub fn add(&mut self, other: &Vector) {
        self.x += other.x;
        self.y += other.y;
    }

    pub fn multiply(&mut self, other: &Vector) {
        self.x *= other.x;
        self.y *= other.y;
    }

    pub fn modulo(&mut self, other: &Vector) {
        self.x = self.x.rem_euclid(other.x);
        self.y = self.y.rem_euclid(other.y);
    }
}

pub struct Robot {
    pub position: Vector,
    pub velocity: Vector,
}

impl Robot {
    pub fn one_second(&mut self, space: &Vector) {
        self.position.add(&self.velocity);
        self.position.modulo(space);
    }

    pub fn n_seconds(&mut self, space: &Vector, seconds: i32) {
        let mut displacement = Vector {
            x: seconds,
            y: seconds,
        };
        displacement.multiply(&self.velocity);
        self.position.add(&displacement);
        self.position.modulo(space);
    }
}

pub fn parse_input(input: &str) -> Vec<Robot> {
    let regex = Regex::new(r"p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = regex.captures(line).unwrap();
            Robot {
                position: Vector {
                    x: caps[1].parse::<i32>().unwrap(),
                    y: caps[2].parse::<i32>().unwrap(),
                },
                velocity: Vector {
                    x: caps[3].parse::<i32>().unwrap(),
                    y: caps[4].parse::<i32>().unwrap(),
                },
            }
        })
        .collect::<Vec<Robot>>()
}
