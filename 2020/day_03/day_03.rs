use std::fs;

fn main() {
    let f = fs::read_to_string("in").unwrap();
    let data: Vec<Vec<char>> = f.lines().map(|x| x.trim().chars().collect()).collect();

    println!("Part One: {}", part_one(&data));
    println!("Part Two: {}", part_two(&data));
}

fn ride(data: &Vec<Vec<char>>, dx: usize, dy: usize) -> u64 {
    let width = data[0].len();

    let mut count = 0;

    let mut x = 0;
    let mut y = 0;

    while y < data.len() {
        if data[y][x] == '#' {
            count += 1;
        }

        x = (x + dx) % width;
        y += dy
    }

    count
}

fn part_one(data: &Vec<Vec<char>>) -> u64 {
    ride(data, 3, 1)
}

fn part_two(data: &Vec<Vec<char>>) -> u64 {
    let moves = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    moves.iter().map(|(dx, dy)| ride(data, *dx, *dy)).product()
}
