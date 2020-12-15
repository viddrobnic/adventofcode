use std::collections::HashMap;

fn main() {
    let starting_numbers = vec![16, 12, 1, 0, 15, 7, 11];

    println!("Part One: {}", solver(&starting_numbers, 2020));
    println!("Part Two: {}", solver(&starting_numbers, 30000000));
}

fn solver(starting_numbers: &Vec<i32>, rounds: i32) -> i32 {
    let mut last_spoken: HashMap<i32, i32> = HashMap::new();
    let mut number_spoken: HashMap<i32, i32> = HashMap::new();
    for (i, n) in starting_numbers.iter().enumerate() {
        last_spoken.insert(*n, i as i32 + 1);
        number_spoken.insert(*n, number_spoken.get(&n).unwrap_or(&0) + 1);
    }

    let mut most_recent: i32 = *starting_numbers.last().unwrap();
    let mut turn = starting_numbers.len() as i32;
    while turn != rounds {
        turn += 1;

        let prev_most_recent: i32 = most_recent;
        
        if *number_spoken.get(&most_recent).unwrap_or(&0) <= 1 {
            most_recent = 0;
        } else {
            most_recent = turn - 1 - last_spoken.get(&most_recent).unwrap();
        }

        number_spoken.insert(most_recent, number_spoken.get(&most_recent).unwrap_or(&0) + 1);
        last_spoken.insert(prev_most_recent, turn - 1);
    }

    most_recent
}
