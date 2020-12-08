use std::fs;
use std::collections::HashSet;

fn main() {
    let f = fs::read_to_string("in").unwrap();
    let data: Vec<&str> = f.lines().map(|x| x.trim()).collect();

    println!("Part One: {}", part_one(&data));
    println!("Part Two: {}", part_two(&data));
}

fn part_one(data: &Vec<&str>) -> u32 {
    let mut count = 0;
    
    let mut current = HashSet::new();
    for line in data {
        if *line == "" {
            count += current.len();
            current.clear();
        } else {
            for c in line.chars() {
                current.insert(c);
            }
        }
    }

    (count + current.len()) as u32
}

fn part_two(data: &Vec<&str>) -> u32 {
    let mut count = 0;

    let mut current = HashSet::new();
    let mut empty = true;
    for line in data {
        if *line == "" {
            count += current.len();
            current.clear();
            empty = true;
        } else {
            let mut person = HashSet::new();
            for c in line.chars() {
                person.insert(c);
            }

            if empty {
                current = current.union(&person).map(|x| *x).collect();
                empty = false;
            } else {
                current = current.intersection(&person).map(|x| *x).collect();
            }
        }
    }

    (count + current.len()) as u32
}