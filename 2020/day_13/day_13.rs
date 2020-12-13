use std::fs;

fn main() {
    let data = fs::read_to_string("in").unwrap();
    let data: Vec<&str> = data.lines().collect();

    let earliest_departure: u32 = data[0].parse().unwrap();
    let buses: Vec<(usize, u32)> = data[1]
        .split(',')
        .enumerate()
        .filter(|(_, b)| *b != "x")
        .map(|(i, b)| (i, b.parse().unwrap()))
        .collect();

    println!("Part One: {}", part_one(earliest_departure, &buses));
    println!("Part Two: {}", part_two(&buses));
}

fn part_one(earliest_departure: u32, buses: &Vec<(usize, u32)>) -> u32 {
    let (b, t) = buses
        .iter()
        .map(|(_, b)| {
            let t = (earliest_departure as f64 / *b as f64).ceil() as u32 * b;
            (b, t)
        })
        .min_by_key(|(_, t)| *t)
        .unwrap();

    b * (t - earliest_departure)
}

fn part_two(buses: &Vec<(usize, u32)>) -> u64 {
    let n = buses.iter().fold(1u64, |acc, (_, b)| acc * (*b as u64));

    let res = buses.iter().fold(0u64, |acc, (i, b)| {
        let index = (-(*i as i32)).rem_euclid(*b as i32) as u64;

        let ni = n / (*b as u64);
        let mi = inverse(ni, (*b).into());

        acc + index * mi * ni
    });

    res.rem_euclid(n)
}

fn inverse(x: u64, n: u64) -> u64 {
    let mut r1 = n;
    let mut r2 = x.rem_euclid(n);
    let mut s1: i64 = 0;
    let mut s2: i64 = 1;

    while r2 != 0 {
        let r3 = r1 % r2;
        
        let q = (r1 / r2) as i64;
        let s3 = s1 - q * s2;

        r1 = r2;
        r2 = r3;
        
        s1 = s2;
        s2 = s3;
    }

    s1.rem_euclid(n as i64) as u64
}
