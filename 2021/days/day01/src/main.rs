fn main() {
    let input = include_str!("input.txt");
    let input: Vec<i32> = input
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    parto_one(&input);
    part_two(&input);
}

fn parto_one(input: &Vec<i32>) {
    let (res, _) = input
        .into_iter()
        .fold((0, None), |(res, prev), x| match prev {
            None => (res, Some(x)),
            Some(y) if x > y => (res + 1, Some(x)),
            _ => (res, Some(x)),
        });

    println!("Part one: {}", res);
}

fn part_two(input: &Vec<i32>) {
    let mut res = 0;
    for i in 1..(input.len() - 2) {
        let prev = input[i - 1] + input[i] + input[i + 1];
        let current = input[i] + input[i + 1] + input[i + 2];
        if current > prev {
            res += 1;
        }
    }

    println!("Part two: {}", res);
}
