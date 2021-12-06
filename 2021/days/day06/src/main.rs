use core::cmp::Reverse;
use std::{cmp::Ordering, collections::BinaryHeap};

fn main() {
    let input: Vec<i32> = include_str!("input.txt")
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Part one: {}", calculate(&input, 80));
    println!("Part two: {}", calculate(&input, 256));
}

#[derive(Eq)]
struct Fish {
    number: u64,
    day: u64,
}

impl Ord for Fish {
    fn cmp(&self, other: &Self) -> Ordering {
        self.day.cmp(&other.day)
    }
}

impl PartialOrd for Fish {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Fish {
    fn eq(&self, other: &Self) -> bool {
        self.day == other.day
    }
}

fn calculate(input: &Vec<i32>, days: i32) -> u64 {
    let mut fish = BinaryHeap::new();

    for val in input {
        let f = Fish {
            number: 1,
            day: (*val).try_into().unwrap(),
        };

        fish.push(Reverse(f));
    }

    let mut prev_day = 0;
    let mut prev_fish = 0;
    loop {
        let f = fish.pop().unwrap().0;
        if f.day >= days.try_into().unwrap() {
            fish.push(Reverse(f));
            fish.push(Reverse(Fish {
                day: prev_day + 7,
                number: prev_fish,
            }));
            fish.push(Reverse(Fish {
                day: prev_day + 9,
                number: prev_fish,
            }));
            break;
        }

        if f.day == prev_day {
            prev_fish += f.number;
        } else {
            if prev_fish > 0 {
                let f1 = Fish {
                    number: prev_fish,
                    day: prev_day + 7,
                };

                let f2 = Fish {
                    number: prev_fish,
                    day: prev_day + 9,
                };

                fish.push(Reverse(f1));
                fish.push(Reverse(f2));
            }

            prev_day = f.day;
            prev_fish = f.number;
        }
    }

    fish.iter().fold(0, |acc, f| acc + f.0.number)
}
