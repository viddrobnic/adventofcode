fn main() {
    let input = include_str!("input.txt");
    let input: Vec<&str> = input.split("\n").filter(|x| *x != "").collect();

    day_01(&input);
    day_02(&input);
}

fn day_01(input: &Vec<&str>) {
    let (x, depth) = input.into_iter().fold((0, 0), |(x, depth), command| {
        let parts: Vec<&str> = command.split_ascii_whitespace().collect();

        let distance: i32 = parts[1].parse().unwrap();

        match parts[0] {
            "forward" => (x + distance, depth),
            "down" => (x, depth + distance),
            "up" => (x, depth - distance),
            _ => (x, depth),
        }
    });

    println!("Part one: {}", x * depth);
}

fn day_02(input: &Vec<&str>) {
    let (x, depth, _) = input
        .into_iter()
        .fold((0, 0, 0), |(x, depth, aim), command| {
            let parts: Vec<&str> = command.split_ascii_whitespace().collect();

            let distance: i32 = parts[1].parse().unwrap();

            match parts[0] {
                "forward" => (x + distance, depth + distance * aim, aim),
                "down" => (x, depth, aim + distance),
                "up" => (x, depth, aim - distance),
                _ => (x, depth, aim),
            }
        });

    println!("Part two: {}", x * depth);
}
