//! Contains solution for day 23.

use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use crate::Solver;

/// Represents a position on the map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

/// Input for the solution that can be parsed from a string.
#[derive(Debug, PartialEq)]
pub struct Input(HashSet<Position>);

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = HashSet::new();

        for (y, line) in s
            .lines()
            .map(|line| line.trim())
            .filter(|line| *line != "")
            .enumerate()
        {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    res.insert(Position {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }

        Ok(Input(res))
    }
}

/// Direction in which the elf can move.
#[derive(Debug, PartialEq)]
struct Direction {
    empty: Vec<Position>,
    moves: Position,
}

/// Move all the elves once.
fn move_once(
    elves: &HashSet<Position>,
    directions: &[Direction],
    start_direction: usize,
) -> HashSet<Position> {
    let mut proposals = HashMap::new();
    for elf in elves.iter() {
        // Check if all neighbours are empty
        let mut empty = true;
        'outer: for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let neighbour = Position {
                    x: elf.x + dx,
                    y: elf.y + dy,
                };
                if elves.contains(&neighbour) {
                    empty = false;
                    break 'outer;
                }
            }
        }

        // If all neighbours are empty, we should stay where we are.
        if empty {
            proposals.insert(*elf, vec![*elf]);
            continue;
        }

        // Iterate through valid directions.
        let mut moved = false;
        for i in 0..directions.len() {
            let index = (i + start_direction) % directions.len();
            let direction = &directions[index];

            // Check if any of the neighbours are empty.
            let empty = direction.empty.iter().all(|diff| {
                let neighbour = Position {
                    x: elf.x + diff.x,
                    y: elf.y + diff.y,
                };

                !elves.contains(&neighbour)
            });

            // If it's empty, add a proposal.
            if empty {
                let proposal = Position {
                    x: elf.x + direction.moves.x,
                    y: elf.y + direction.moves.y,
                };

                match proposals.get_mut(&proposal) {
                    None => {
                        proposals.insert(proposal, vec![*elf]);
                    }
                    Some(existing) => existing.push(*elf),
                }

                moved = true;
                break;
            }
        }

        if !moved {
            // If elf could not be moved, stay at the same place.
            proposals.insert(*elf, vec![*elf]);
        }
    }

    // Convert proposals into locations.
    let mut res = HashSet::new();
    for (proposal, elves) in proposals.iter() {
        if elves.len() == 1 {
            // If there is only one elf that proposed the location, move them there.
            res.insert(*proposal);
        } else {
            // If there are multiple elves that proposed the location, do not move them.
            for elf in elves {
                res.insert(*elf);
            }
        }
    }

    assert_eq!(res.len(), elves.len());

    res
}

/// Constructs possible move directions.
fn construct_directions() -> Vec<Direction> {
    vec![
        Direction {
            empty: vec![
                Position { x: 0, y: -1 },
                Position { x: 1, y: -1 },
                Position { x: -1, y: -1 },
            ],
            moves: Position { x: 0, y: -1 },
        },
        Direction {
            empty: vec![
                Position { x: 0, y: 1 },
                Position { x: 1, y: 1 },
                Position { x: -1, y: 1 },
            ],
            moves: Position { x: 0, y: 1 },
        },
        Direction {
            empty: vec![
                Position { x: -1, y: 0 },
                Position { x: -1, y: -1 },
                Position { x: -1, y: 1 },
            ],
            moves: Position { x: -1, y: 0 },
        },
        Direction {
            empty: vec![
                Position { x: 1, y: 0 },
                Position { x: 1, y: -1 },
                Position { x: 1, y: 1 },
            ],
            moves: Position { x: 1, y: 0 },
        },
    ]
}

/// Gets the bounding rectangle of the elves.
fn get_bounding_rect(elves: &HashSet<Position>) -> (Position, Position) {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    for elf in elves {
        min_x = min(elf.x, min_x);
        min_y = min(elf.y, min_y);
        max_x = max(elf.x, max_x);
        max_y = max(elf.y, max_y);
    }

    (
        Position { x: min_x, y: min_y },
        Position { x: max_x, y: max_y },
    )
}

/// Calculates number of empty fields.
fn get_empty_fields(elves: &HashSet<Position>) -> i64 {
    let (min_pos, max_pos) = get_bounding_rect(elves);

    let width = (max_pos.x - min_pos.x + 1) as i64;
    let height = (max_pos.y - min_pos.y + 1) as i64;
    let nr_tiles = width * height;
    nr_tiles - elves.len() as i64
}

/// Draws the board. Used for debugging purposes
#[allow(dead_code)]
fn draw(elves: &HashSet<Position>) {
    let (min_pos, max_pos) = get_bounding_rect(elves);
    let width = (max_pos.x - min_pos.x + 1) as i64;
    let height = (max_pos.y - min_pos.y + 1) as i64;

    let mut map = vec![vec!["."; width as usize]; height as usize];
    for elf in elves {
        let x = elf.x - min_pos.x;
        let y = elf.y - min_pos.y;

        map[y as usize][x as usize] = "#";
    }

    for line in map.iter() {
        println!("{}", line.join(""));
    }

    println!();
}

/// Solution for day 23.
pub struct Solution;

impl Solver for Solution {
    type Input = Input;
    type Output = i64;

    fn get_day(&self) -> u32 {
        23
    }

    fn part_one(&self, input: &Self::Input) -> Self::Output {
        // Initialize elves, directions and start direction.
        let directions = construct_directions();
        let mut start_direction = 0;
        let mut elves = input.0.clone();

        for _ in 0..10 {
            // Move elves and change the start direction.
            elves = move_once(&elves, &directions, start_direction);
            start_direction = (start_direction + 1) % directions.len();
        }

        get_empty_fields(&elves)
    }

    fn part_two(&self, input: &Self::Input) -> Self::Output {
        // Initialize elves, directions and start direction.
        let directions = construct_directions();
        let mut start_direction = 0;
        let mut elves = input.0.clone();

        let mut rounds = 0;
        loop {
            // Move elves.
            let new_elves = move_once(&elves, &directions, start_direction);
            start_direction = (start_direction + 1) % directions.len();
            rounds += 1;

            // If elves have not changed, exit the loop.
            if elves == new_elves {
                break;
            }

            elves = new_elves;
        }

        // Return the number of rounds.
        rounds
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::str::FromStr;

    use crate::solution::day_23::{Input, Position, Solution};
    use crate::Solver;

    #[test]
    fn parse_input() {
        let input = Input::from_str(
            "....#..
                ..###.#",
        )
        .unwrap();
        let mut res = HashSet::new();
        res.insert(Position { x: 4, y: 0 });
        res.insert(Position { x: 2, y: 1 });
        res.insert(Position { x: 3, y: 1 });
        res.insert(Position { x: 4, y: 1 });
        res.insert(Position { x: 6, y: 1 });

        assert_eq!(input, Input(res));
    }

    #[test]
    fn part_one() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_one(&input);
        assert_eq!(solution, 110);
    }

    #[test]
    fn part_two() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_two(&input);
        assert_eq!(solution, 20);
    }

    const INPUT: &str = r#"
        ....#..
        ..###.#
        #...#.#
        .#...##
        #.###..
        ##.#.##
        .#..#..
        "#;
}
