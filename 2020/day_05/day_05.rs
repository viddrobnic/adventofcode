use std::fs;

fn main() {
    let f = fs::read_to_string("in").unwrap();
    let data: Vec<&str> = f.lines().map(|x| x.trim()).collect();
    let ids: Vec<u32> = data.iter()
        .map(|x| (&x[0..7], &x[7..]))
        .map(|(x, y)| (x.replace("F", "0").replace("B", "1"), y.replace("L", "0").replace("R", "1")))
        .map(|(x, y)| (u32::from_str_radix(&x, 2).unwrap(), u32::from_str_radix(&y, 2).unwrap()))
        .map(|(x, y)| x * 8 + y).collect();

    println!("Part One: {}", part_one(&ids));
    println!("Part Two: {}", part_two(&ids));
}

fn part_one(data: &Vec<u32>) -> u32 {
    *data.iter().max().unwrap()
}

fn part_two(data: &Vec<u32>) -> u32 {
    let mut sorted = data.clone();
    sorted.sort();

    let (_, x) = sorted.iter().enumerate().skip(1).find(|(i, &x)| x - sorted[i-1] == 2).unwrap();
    *x-1
}