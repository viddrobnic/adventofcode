mod board;

use board::Board;

fn main() {
    let input = include_str!("input.txt");
    let input: Vec<&str> = input.split("\n").filter(|s| *s != "").collect();

    let generated_numbers: Vec<i32> = input[0].split(",").map(|s| s.parse().unwrap()).collect();

    let mut boards = Vec::new();

    for i in (1..input.len()).step_by(5) {
        let data = &input[i..i + 5].to_vec();
        let board = Board::new(data);
        boards.push(board);
    }

    part_one(boards.clone(), &generated_numbers);
    part_two(boards.clone(), &generated_numbers);
}

fn part_one(mut boards: Vec<Board>, numbers: &Vec<i32>) {
    for num in numbers {
        for board in boards.iter_mut() {
            board.mark(*num);
            if board.full() {
                println!("Part one: {}", board.sum_unmarked() * num);
                return;
            }
        }
    }
}

fn part_two(mut boards: Vec<Board>, numbers: &Vec<i32>) {
    let mut last_score = 0;

    for num in numbers {
        for board in boards.iter_mut() {
            if board.mark(*num) {
                last_score = board.sum_unmarked() * num;
            }
        }
    }

    println!("Part two: {}", last_score);
}
