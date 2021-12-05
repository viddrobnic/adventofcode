use std::{cmp::max, cmp::min, collections::HashMap};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let input = include_str!("input.txt");
    let segments: Vec<(Point, Point)> = input
        .split("\n")
        .filter(|s| *s != "")
        .map(|line| {
            let parts: Vec<&str> = line.split(" -> ").collect();

            let parts1: Vec<i32> = parts[0].split(",").map(|s| s.parse().unwrap()).collect();
            let parts2: Vec<i32> = parts[1].split(",").map(|s| s.parse().unwrap()).collect();

            let p1 = Point {
                x: parts1[0],
                y: parts1[1],
            };
            let p2 = Point {
                x: parts2[0],
                y: parts2[1],
            };

            (p1, p2)
        })
        .collect();

    part_one(&segments);
    part_two(&segments);
}

fn part_one(segments: &Vec<(Point, Point)>) {
    let res = worker(segments, false);
    println!("Part one: {}", res);
}

fn part_two(segments: &Vec<(Point, Point)>) {
    let res = worker(segments, true);
    println!("Part two: {}", res);
}

fn worker(segments: &Vec<(Point, Point)>, include_diagonal: bool) -> i32 {
    let mut board = HashMap::new();
    for (p1, p2) in segments {
        if p1.x == p2.x {
            for y in min(p1.y, p2.y)..=max(p1.y, p2.y) {
                let point = Point { x: p1.x, y: y };
                let current = *board.get(&point).unwrap_or(&0);
                board.insert(point, current + 1);
            }
        } else if p1.y == p2.y {
            for x in min(p1.x, p2.x)..=max(p1.x, p2.x) {
                let point = Point { x: x, y: p1.y };
                let current = *board.get(&point).unwrap_or(&0);
                board.insert(point, current + 1);
            }
        } else if include_diagonal {
            let dx = if p1.x < p2.x { 1 } else { -1 };

            let dy = if p1.y < p2.y { 1 } else { -1 };

            let len = dx * (p2.x - p1.x);

            for diff in 0..=len {
                let point = Point {
                    x: p1.x + dx * diff,
                    y: p1.y + dy * diff,
                };

                let current = *board.get(&point).unwrap_or(&0);
                board.insert(point, current + 1);
            }
        }
    }

    board
        .values()
        .fold(0, |acc, val| if *val >= 2 { acc + 1 } else { acc })
}
