use std::fs;

fn main() {
    let f = fs::read_to_string("in").unwrap();
    let data: Vec<_> = f.lines().collect();

    println!("Part One: {}", part_one(&data));
    println!("Part Two: {}", part_two(&data))
}

fn clean_line<'a>(line: &'a str) -> (i32, i32, char, &'a str) {
    let parts: Vec<_> = line.split_whitespace().collect();
        
    let part1: Vec<i32> = parts[0].split("-").map(|x| x.parse().unwrap()).collect();
    let min = part1[0];
    let max = part1[1];

    let letter = parts[1].chars().next().unwrap();
    let pass = parts[2];

    (min, max, letter, pass)
}

fn part_one(data: &Vec<&str>) -> i32 {
    let mut valid = 0;
    
    for line in data.iter().map(|x| x.trim()) {
        let (min, max, letter, pass) = clean_line(line);

        let c: i32 = pass.chars().filter(|x| *x == letter).count() as i32;
        if c >= min && c <= max {
            valid += 1;
        }
    }

    valid
}

fn part_two(data: &Vec<&str>) -> i32 {
    let mut valid = 0;
    
    for line in data.iter().map(|x| x.trim()) {
        let (min, max, letter, pass) = clean_line(line);

        let contains1 = pass.chars().nth((min-1) as usize).unwrap() == letter;
        let contains2 = pass.chars().nth((max-1) as usize).unwrap() == letter;

        if (contains1 && !contains2) || (!contains1 && contains2) {
            valid += 1;
        }
    }

    valid
}
