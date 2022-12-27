//! Contains solution for day 24.

use crate::Solver;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;
use thiserror::Error;

/// Direction of the blizzard.
#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn dx(&self) -> i64 {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
            _ => 0,
        }
    }

    fn dy(&self) -> i64 {
        match self {
            Direction::Up => -1,
            Direction::Down => 1,
            _ => 0,
        }
    }
}

/// Current state used in BFS.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct State {
    x: i64,
    y: i64,
    step: usize,
}

/// Tile on the map.
#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Blizzards(Vec<Direction>),
}

/// Error returned during parsing of the input.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum InputError {
    #[error("Got invalid character: {0}")]
    InvalidChar(char),
}

/// Input for the solution that can be parsed from a string.
#[derive(Debug, Clone, PartialEq)]
pub struct Input(Vec<Vec<Tile>>);

impl FromStr for Input {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: Result<Vec<Vec<_>>, _> = s
            .lines()
            .map(|line| line.trim()) // Trim whitespace
            .filter(|line| *line != "") // Filter out empty lines
            .map(|line| {
                // Convert line into a row.
                line.chars()
                    .map(|c| match c {
                        '.' => Ok(Tile::Empty),
                        '#' => Ok(Tile::Wall),
                        '>' => Ok(Tile::Blizzards(vec![Direction::Right])),
                        '<' => Ok(Tile::Blizzards(vec![Direction::Left])),
                        '^' => Ok(Tile::Blizzards(vec![Direction::Up])),
                        'v' => Ok(Tile::Blizzards(vec![Direction::Down])),
                        _ => Err(InputError::InvalidChar(c)),
                    })
                    .collect()
            })
            .collect();

        Ok(Input(res?))
    }
}

/// Get map for the given steps, using a memo.
fn get_map(step: usize, memo: &mut Vec<Vec<Vec<Tile>>>) -> &Vec<Vec<Tile>> {
    // If map is in the memo, return it.
    if step < memo.len() {
        return &memo[step];
    }

    // Calculate maps for the steps until the queired one.
    for _ in memo.len()..=step {
        let last = memo.last().unwrap();
        let height = last.len();
        let width = last[0].len();

        // Create a new empty map
        let mut new = vec![vec![Tile::Empty; width]; height];
        for y in 0..height {
            new[y][0] = Tile::Wall;
            new[y][width - 1] = Tile::Wall;
        }
        for x in 2..width {
            new[0][x] = Tile::Wall;
        }
        for x in 0..width - 2 {
            new[height - 1][x] = Tile::Wall;
        }

        // Copy over the moved blizzards
        for y in 0..height {
            for (x, tile) in last[y].iter().enumerate() {
                let blizzards = match tile {
                    Tile::Empty | Tile::Wall => continue,
                    Tile::Blizzards(blizzards) => blizzards,
                };

                for blizzard in blizzards {
                    // Move the blizzard.
                    let mut b_y = y as i64 + blizzard.dy();
                    if b_y == 0 {
                        b_y = (height - 2) as i64;
                    }
                    if b_y == (height - 1) as i64 {
                        b_y = 1;
                    }

                    let mut b_x = x as i64 + blizzard.dx();
                    if b_x == 0 {
                        b_x = (width - 2) as i64;
                    }
                    if b_x == (width - 1) as i64 {
                        b_x = 1;
                    }

                    // Add it to the new map
                    match &mut new[b_y as usize][b_x as usize] {
                        Tile::Empty => {
                            new[b_y as usize][b_x as usize] = Tile::Blizzards(vec![*blizzard])
                        }
                        Tile::Blizzards(existing) => existing.push(*blizzard),
                        Tile::Wall => panic!("Unexpected wall at x: {}, y: {}", x, y),
                    }
                }
            }
        }

        // Save the new map
        memo.push(new);
    }

    // Return the calculated map
    &memo[step]
}

/// Returns the shortest number of steps to get from start to end, starting at the given step.
fn solve(
    map: &Vec<Vec<Tile>>,
    start_x: i64,
    start_y: i64,
    start_step: usize,
    end_x: i64,
    end_y: i64,
) -> usize {
    let mut maps = vec![map.clone()];

    let height = map.len() as i64;
    let width = map[0].len() as i64;

    // Prepare the queue for the bfs.
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    let start_state = State {
        x: start_x,
        y: start_y,
        step: start_step,
    };
    visited.insert(start_state);
    queue.push_back(start_state);

    // BFS
    while !queue.is_empty() {
        let state = queue.pop_front().unwrap();

        // If at the end, return the number of steps.
        if state.y == end_y && state.x == end_x {
            return state.step;
        }

        // Get the map for the current step.
        let map = get_map(state.step + 1, &mut maps);

        // Go through the neighbours and the current position.
        for (dx, dy) in vec![(-1, 0), (1, 0), (0, 1), (0, -1), (0, 0)] {
            let x = state.x + dx;
            let y = state.y + dy;

            // If out of bounds, do nothing
            if x < 0 || x >= width || y < 0 || y >= height {
                continue;
            }

            // If tile is empty, add it to the queue.
            if map[y as usize][x as usize] == Tile::Empty {
                let new_state = State {
                    x,
                    y,
                    step: state.step + 1,
                };

                if !visited.contains(&new_state) {
                    visited.insert(new_state);
                    queue.push_back(new_state);
                }
            }
        }
    }

    unreachable!()
}

/// Solution for day 24
pub struct Solution;

impl Solver for Solution {
    type Input = Input;
    type Output = usize;

    fn get_day(&self) -> u32 {
        24
    }

    fn part_one(&self, input: &Self::Input) -> Self::Output {
        let width = input.0[0].len() as i64;
        let height = input.0.len() as i64;
        solve(&input.0, 1, 0, 0, width - 2, height - 1)
    }

    fn part_two(&self, input: &Self::Input) -> Self::Output {
        let width = input.0[0].len() as i64;
        let height = input.0.len() as i64;

        let start_x = 1;
        let start_y = 0;
        let end_x = width - 2;
        let end_y = height - 1;

        // Number of steps to get from start to end
        let to_end = solve(&input.0, start_x, start_y, 0, end_x, end_y);
        // Number of steps to get from start to end and back to start.
        let to_start = solve(&input.0, end_x, end_y, to_end, start_x, start_y);

        // Number of steps to get from start to end, back to start and again back to end.
        solve(&input.0, start_x, start_y, to_start, end_x, end_y)
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::day_24::{get_map, Direction, Input, Solution, Tile};
    use crate::Solver;
    use std::str::FromStr;

    #[test]
    fn part_one() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_one(&input);
        assert_eq!(solution, 18);
    }

    #[test]
    fn part_two() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_two(&input);
        assert_eq!(solution, 54);
    }

    #[test]
    fn get_map_test() {
        let input = Input::from_str(INPUT_SIMPLE).unwrap();
        let mut memo = vec![input.0];

        let map = get_map(5, &mut memo);
        let res = vec![
            vec![
                Tile::Wall,
                Tile::Empty,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
            ],
            vec![
                Tile::Wall,
                Tile::Empty,
                Tile::Empty,
                Tile::Empty,
                Tile::Empty,
                Tile::Empty,
                Tile::Wall,
            ],
            vec![
                Tile::Wall,
                Tile::Blizzards(vec![Direction::Right]),
                Tile::Empty,
                Tile::Empty,
                Tile::Empty,
                Tile::Empty,
                Tile::Wall,
            ],
            vec![
                Tile::Wall,
                Tile::Empty,
                Tile::Empty,
                Tile::Empty,
                Tile::Empty,
                Tile::Empty,
                Tile::Wall,
            ],
            vec![
                Tile::Wall,
                Tile::Empty,
                Tile::Empty,
                Tile::Empty,
                Tile::Blizzards(vec![Direction::Down]),
                Tile::Empty,
                Tile::Wall,
            ],
            vec![
                Tile::Wall,
                Tile::Empty,
                Tile::Empty,
                Tile::Empty,
                Tile::Empty,
                Tile::Empty,
                Tile::Wall,
            ],
            vec![
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Empty,
                Tile::Wall,
            ],
        ];
        assert_eq!(map, &res);
    }

    #[test]
    fn parse_input() {
        let input = Input::from_str(INPUT_SIMPLE).unwrap();
        assert_eq!(
            input,
            Input(vec![
                vec![
                    Tile::Wall,
                    Tile::Empty,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                ],
                vec![
                    Tile::Wall,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Wall
                ],
                vec![
                    Tile::Wall,
                    Tile::Blizzards(vec![Direction::Right]),
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Wall,
                ],
                vec![
                    Tile::Wall,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Wall
                ],
                vec![
                    Tile::Wall,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Blizzards(vec![Direction::Down]),
                    Tile::Empty,
                    Tile::Wall,
                ],
                vec![
                    Tile::Wall,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Wall
                ],
                vec![
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Empty,
                    Tile::Wall,
                ],
            ])
        );
    }

    const INPUT_SIMPLE: &str = r#"
        #.#####
        #.....#
        #>....#
        #.....#
        #...v.#
        #.....#
        #####.#
        "#;

    const INPUT: &str = r#"
        #.######
        #>>.<^<#
        #.<..<<#
        #>v.><>#
        #<^v^^>#
        ######.#
        "#;
}
