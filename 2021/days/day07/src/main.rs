fn main() {
    let input: Vec<i32> = include_str!("input.txt")
        .split(",")
        .filter(|val| *val != "")
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let part_one = worker(&input, |pos, target| (pos - target).abs());
    println!("Part one: {}", part_one);

    let part_two = worker(&input, |pos, target| {
        let distance = (pos - target).abs();
        distance * (distance + 1) / 2
    });
    println!("Part two: {}", part_two);
}

fn worker(input: &Vec<i32>, cost: fn(i32, i32) -> i32) -> i32 {
    let m = input.iter().max().unwrap();

    let mut fuel = None;
    for position in 0..=*m {
        let f = input.iter().fold(0, |acc, val| acc + cost(*val, position));

        match fuel {
            None => fuel = Some(f),
            Some(f2) if f < f2 => fuel = Some(f),
            _ => (),
        };
    }

    fuel.unwrap()
}
