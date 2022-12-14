//! Contains solution for day 14.
use std::cmp::{max, min};
use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

use crate::Solver;

/// Error that occurred during parsing of the input.
#[derive(Debug, Error)]
pub enum InputError {
    /// One of the number in the input is invalid.
    #[error("Could not parse number in input: {source}")]
    InvalidNumber {
        #[from]
        source: ParseIntError,
    },

    /// A pair of coordinates is invalid.
    #[error("Coordinate pair is invalid: {0}")]
    InvalidCoordinatePair(String),
}

/// A point on the map.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

/// Input for the solution that can be parsed from a string.
#[derive(Debug, PartialEq)]
pub struct Input {
    map: Vec<Vec<bool>>,
    max_y: usize,
}

impl FromStr for Input {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Final map. We initialize a big enough 2D vector.
        let mut res = vec![vec![false; 1000]; 200];
        // Global maximum y.
        let mut global_max_y = None;

        // Go through cleaned lines.
        for line in s.lines().map(|line| line.trim()).filter(|line| *line != "") {
            // Map each line into vector of points, each point representing an endpoint.
            let parts: Result<Vec<Point>, _> = line
                .split("->")
                .map(|part| {
                    let numbers: Result<Vec<usize>, _> =
                        part.split(",").map(|p| p.trim().parse()).collect();
                    let numbers = numbers?;

                    if numbers.len() != 2 {
                        return Err(InputError::InvalidCoordinatePair(part.to_string()));
                    }

                    Ok(Point {
                        x: numbers[0],
                        y: numbers[1],
                    })
                })
                .collect();
            let parts = parts?;

            // Go through all the endpoints and fill the lines in between
            for (p1, p2) in parts.iter().zip(parts.iter().skip(1)) {
                let min_x = min(p1.x, p2.x);
                let max_x = max(p1.x, p2.x);
                for x in min_x..=max_x {
                    let min_y = min(p1.y, p2.y);
                    let max_y = max(p1.y, p2.y);

                    // Set the global max y correctly.
                    match global_max_y {
                        None => global_max_y = Some(max_y),
                        Some(my) if max_y > my => global_max_y = Some(max_y),
                        _ => (),
                    }

                    for y in min_y..=max_y {
                        res[y][x] = true;
                    }
                }
            }
        }

        Ok(Input {
            map: res,
            max_y: global_max_y.expect("There should be a max y if input is valid."),
        })
    }
}

pub struct Solution;

impl Solver for Solution {
    type Input = Input;
    type Output = usize;

    fn get_day(&self) -> u32 {
        14
    }

    fn part_one(&self, input: &Self::Input) -> Self::Output {
        // Clone the map so we can change it in place.
        let mut map = input.map.clone();
        // Result of the puzzle. This is number of sand grains.
        let mut res = 0;

        let sand_input = Point { x: 500, y: 0 };
        'outer: loop {
            // Simulate a single sand grain
            let mut sand = sand_input;

            loop {
                // Move a sand by one.
                let moved;
                (sand, moved) = move_sand(sand, &map);

                // If sand stopped moving, save its position in the map,
                // add additional moved sand grain to the result,
                // and break the current grain simulation.
                if !moved {
                    map[sand.y][sand.x] = true;
                    res += 1;
                    break;
                }

                // If sand is dropping into the void, we can stop the simulation entirely.
                if sand.y > input.max_y {
                    break 'outer;
                }
            }
        }

        res
    }

    fn part_two(&self, input: &Self::Input) -> Self::Output {
        // Clone the map so we can change it in place.
        let mut map = input.map.clone();
        // Add the floor to the map
        for x in 0..1000 {
            map[input.max_y + 2][x] = true;
        }

        // Result of the puzzle. This is number of sand grains.
        let mut res = 0;

        let sand_input = Point { x: 500, y: 0 };
        'outer: loop {
            // Simulate a single sand grain
            let mut sand = sand_input;

            loop {
                let moved;
                (sand, moved) = move_sand(sand, &map);

                if !moved {
                    // If sand stopped moving, save its position in the map and add
                    // one to the result.
                    map[sand.y][sand.x] = true;
                    res += 1;

                    // If sand blocks the source, we can stop the simulation entirely.
                    if sand == sand_input {
                        break 'outer;
                    }

                    // Stop the simulation for the current sand.
                    break;
                }
            }
        }

        res
    }
}

/// Move sand down a single position.
fn move_sand(mut sand: Point, map: &Vec<Vec<bool>>) -> (Point, bool) {
    if !map[sand.y + 1][sand.x] {
        // Try moving it down.
        sand.y += 1;
        (sand, true)
    } else if !map[sand.y + 1][sand.x - 1] {
        // Try moving it down and left
        sand.y += 1;
        sand.x -= 1;
        (sand, true)
    } else if !map[sand.y + 1][sand.x + 1] {
        // Try moving it down and right
        sand.y += 1;
        sand.x += 1;
        (sand, true)
    } else {
        // Cannot be moved.
        (sand, false)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::solution::day_14::{Input, Solution};
    use crate::Solver;

    const INPUT: &str = r#"
        498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9
        "#;

    #[test]
    fn parse_input() {
        let input = Input::from_str(INPUT).unwrap();

        let mut expected = vec![vec![false; 1000]; 200];
        expected[4][498] = true;
        expected[5][498] = true;
        expected[6][498] = true;
        expected[6][497] = true;
        expected[6][496] = true;
        expected[4][503] = true;
        expected[4][502] = true;
        expected[5][502] = true;
        expected[6][502] = true;
        expected[7][502] = true;
        expected[8][502] = true;
        expected[9][502] = true;
        expected[9][501] = true;
        expected[9][500] = true;
        expected[9][499] = true;
        expected[9][498] = true;
        expected[9][497] = true;
        expected[9][496] = true;
        expected[9][495] = true;
        expected[9][494] = true;

        assert_eq!(
            input,
            Input {
                map: expected,
                max_y: 9
            }
        );
    }

    #[test]
    fn part_one() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_one(&input);
        assert_eq!(solution, 24);
    }

    #[test]
    fn part_two() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_two(&input);
        assert_eq!(solution, 93);
    }
}
