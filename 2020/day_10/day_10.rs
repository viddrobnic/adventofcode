use std::fs;
use std::collections::HashMap;

fn main() {
    let f = fs::read_to_string("in").unwrap();
    let mut data: Vec<i32> = f.lines().map(|x| x.trim().parse().unwrap()).collect();
    data.push(0);
    data.push(*data.iter().max().unwrap() + 3);
    data.sort();

    println!("Part One: {}", part_one(&data));
    println!("Part Two: {}", part_two(&data));
}

fn part_one(data: &Vec<i32>) -> i32 {
    let (diff1, diff3) = data
        .iter()
        .skip(1)
        .zip(data.iter())
        .map(|(x, y)| x - y)
        .fold((0, 0), |(diff1, diff3), x| {
            if x == 1 {
                (diff1 + 1, diff3)
            } else {
                (diff1, diff3 + 1)
            }
        });

    diff1 * diff3
}

fn part_two(data: &Vec<i32>) -> u64 {
    let mut memo = HashMap::new();
    part_two_solver(data, 0, &mut memo)
}

fn part_two_solver(data: &Vec<i32>, index: usize, memo: &mut HashMap<usize, u64>) -> u64 {
    if index == data.len() - 1 {
        return 1;
    }

    if let Some(res) = memo.get(&index) {
        return *res;
    }

    let mut count = 0;
    for i in (index+1)..data.len() {
        if data[i] - data[index] <= 3{
            count += part_two_solver(data, i, memo);
        } else {
            break;
        }
    }

    memo.insert(index, count);
    count
}
