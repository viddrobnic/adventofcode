use std::fs;

fn main() {
    let f = fs::read_to_string("in").unwrap();
    let data: Vec<i32> = f.lines().map(|l| l.trim().parse().unwrap()).collect();

    println!("Part One: {}", part_one(&data));
    println!("Part Two: {}", part_two(&data));
}

fn part_one(data: &Vec<i32>) -> i32 {
    for a in data {
        if let Some(res) = data.iter().find(|x| *a + *x == 2020) {
            return *a * *res;
        }
    }

    0
}

fn part_two(data: &Vec<i32>) -> i32 {
    for a in data {
        for b in data {
            for c in data {
                if *a + *b + *c == 2020 {
                    return *a * *b * *c;
                }
            }
        }
    }

    0
}