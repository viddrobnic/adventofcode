use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
enum Action {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

#[derive(Debug)]
enum ParseActionError {
    Action,
    Value(ParseIntError),
}

impl FromStr for Action {
    type Err = ParseActionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let act = &s[0..1];
        let val: i32 = s[1..].parse().map_err(|e| ParseActionError::Value(e))?;

        match act {
            "N" => Ok(Action::North(val)),
            "S" => Ok(Action::South(val)),
            "E" => Ok(Action::East(val)),
            "W" => Ok(Action::West(val)),
            "L" => Ok(Action::Left(val)),
            "R" => Ok(Action::Right(val)),
            "F" => Ok(Action::Forward(val)),
            _ => Err(ParseActionError::Action),
        }
    }
}

fn main() {
    let data: Vec<Action> = fs::read_to_string("in")
        .unwrap()
        .lines()
        .map(|l| l.trim().parse().unwrap())
        .collect();

    println!("Part One: {}", part_one(&data));
    println!("Part Two: {}", part_two(&data));
}

fn part_one(data: &Vec<Action>) -> u32 {
    let (x, y, _) = data
        .iter()
        .fold((0i32, 0i32, 0i32), |(x, y, r), act| match act {
            Action::North(val) => (x, y + val, r),
            Action::South(val) => (x, y - val, r),
            Action::West(val) => (x - val, y, r),
            Action::East(val) => (x + val, y, r),
            Action::Left(val) => (x, y, (r + (val / 90)).rem_euclid(4)),
            Action::Right(val) => (x, y, (r - (val / 90)).rem_euclid(4)),
            Action::Forward(val) => match r {
                0 => (x + val, y, r),
                1 => (x, y + val, r),
                2 => (x - val, y, r),
                3 => (x, y - val, r),
                _ => panic!("Invalid rotation"),
            },
        });

    (x.abs() + y.abs()) as u32
}

fn part_two(data: &Vec<Action>) -> u32 {
    let (x, y, _, _) = data.iter().fold(
        (0i32, 0i32, 10i32, 1i32),
        |(x, y, x_w, y_w), act| match act {
            Action::North(val) => (x, y, x_w, y_w + val),
            Action::South(val) => (x, y, x_w, y_w - val),
            Action::West(val) => (x, y, x_w - val, y_w),
            Action::East(val) => (x, y, x_w + val, y_w),
            Action::Left(val) => {
                let mut x_w = x_w;
                let mut y_w = y_w;

                for _ in 0..(val / 90) {
                    let tmp = x_w;
                    x_w = -y_w;
                    y_w = tmp;
                }

                (x, y, x_w, y_w)
            }
            Action::Right(val) => {
                let mut x_w = x_w;
                let mut y_w = y_w;

                for _ in 0..(val / 90) {
                    let tmp = x_w;
                    x_w = y_w;
                    y_w = -tmp;
                }

                (x, y, x_w, y_w)
            }
            Action::Forward(val) => (x + val * x_w, y + val * y_w, x_w, y_w),
        },
    );

    (x.abs() + y.abs()) as u32
}
