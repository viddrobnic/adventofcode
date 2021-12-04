use std::vec;

fn main() {
    let input: Vec<&str> = include_str!("input.txt")
        .split("\n")
        .filter(|s| *s != "")
        .collect();

    let input: Vec<Vec<i32>> = input
        .into_iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let counted = count(&input);

    let input_len = input.len() as i32;
    let mut count_bin = vec![0; counted.len()];

    for (i, x) in counted.iter().enumerate() {
        let y = input_len - x;
        if *x > y {
            count_bin[i] = 0;
        } else {
            count_bin[i] = 1;
        }
    }

    part_one(&count_bin);
    part_two(&input);
}

fn count(input: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut count = vec![0; input[0].len()];

    for data in input {
        for (i, x) in data.into_iter().enumerate() {
            count[i] += x;
        }
    }

    count
}

fn part_one(input: &Vec<i32>) {
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in input {
        gamma *= 2;
        epsilon *= 2;

        gamma += i;

        if *i == 0 {
            epsilon += 1;
        }
    }

    println!("Part one: {}", gamma * epsilon);
}

fn part_two(input: &Vec<Vec<i32>>) {
    let oxygen_generator_bin = find(input, 0, |nr1, nr2| if nr1 >= nr2 { 1 } else { 0 });
    let co2_generator_bin = find(input, 0, |nr1, nr2| if nr1 < nr2 { 1 } else { 0 });

    let mut oxygen_generator = 0;
    let mut co2_generator = 0;
    for i in 0..oxygen_generator_bin.len() {
        oxygen_generator *= 2;
        co2_generator *= 2;

        oxygen_generator += oxygen_generator_bin[i];
        co2_generator += co2_generator_bin[i];
    }

    println!("Part two: {}", oxygen_generator * co2_generator);
}

fn find<'a>(input: &'a Vec<Vec<i32>>, index: usize, filter_by: fn(i32, i32) -> i32) -> Vec<i32> {
    if input.len() == 1 {
        return input[0].clone();
    }

    let counted = count(input);

    let input: Vec<Vec<i32>> = input
        .iter()
        .filter(|x| {
            let nr1 = counted[index];
            let nr2 = input.len() as i32 - counted[index];

            x[index] == filter_by(nr1, nr2)
        })
        .map(|v| v.clone())
        .collect();

    find(&input, index + 1, filter_by)
}
