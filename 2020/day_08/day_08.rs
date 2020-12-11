use std::collections::HashSet;
use std::fs;

enum Return {
    Loop(i32),
    Exit(i32),
}

fn main() {
    let data = fs::read_to_string("in").unwrap();
    let data: Vec<_> = data
        .lines()
        .map(|x| {
            let mut parts = x.trim().split_whitespace();
            let parts1 = parts.next().unwrap();
            let parts2 = parts.next().unwrap();

            (parts1, parts2.parse::<i32>().unwrap())
        })
        .collect();

    println!("Part One: {}", part_one(&data));
    println!("Part Two: {}", part_two(&data));
}

fn part_one(data: &Vec<(&str, i32)>) -> i32 {
    match solver(data) {
        Return::Loop(x) => x,
        Return::Exit(_) => panic!("Should not happen"),
    }
}

fn part_two(data: &Vec<(&str, i32)>) -> i32 {
    for i in 0..data.len() {
        let mut prog = data.clone();

        if prog[i].0 == "jmp" {
            prog[i].0 = "nop"
        } else if prog[i].0 == "nop" {
            prog[i].0 = "jmp"
        } else {
            continue;
        }

        match solver(&prog) {
            Return::Loop(_) => (),
            Return::Exit(x) => return x,
        }
    }

    panic!("Should not happen");
}

fn solver(data: &Vec<(&str, i32)>) -> Return {
    let mut acc = 0;
    let mut index = 0;
    let mut executed = HashSet::<i32>::new();

    loop {
        if executed.contains(&index) {
            return Return::Loop(acc);
        }

        if index as usize >= data.len() {
            return Return::Exit(acc);
        }

        executed.insert(index);

        match data[index as usize] {
            ("acc", x) => {
                acc += x;
                index += 1;
            }
            ("jmp", x) => {
                index += x;
            }
            ("nop", _) => {
                index += 1;
            }
            _ => panic!("Instruction does not exist"),
        }
    }
}
