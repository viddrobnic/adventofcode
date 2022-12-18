//! Contains solution for day 17.
//!
use std::collections::HashMap;
use std::str::FromStr;

use thiserror::Error;

use crate::Solver;

/// Direction in which a rock can move.
#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Down,
}

impl Direction {
    /// Difference in x coordinate for the direction.
    fn dx(&self) -> i64 {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
            Direction::Down => 0,
        }
    }

    /// Difference in y coordinate for the direction.
    fn dy(&self) -> i64 {
        match self {
            Direction::Left | Direction::Right => 0,
            Direction::Down => -1,
        }
    }
}

/// Represents a 2D coordinate.
#[derive(Debug, Copy, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}

/// Abstract shape of a rock.
trait Shape {
    /// Move in the specified direction.
    fn advance(&mut self, direction: Direction);

    /// Returns the coordinates occupied by the shape.
    fn coordinates_filled(&self) -> Vec<Coordinate>;
}

/// Implementation of the minus shape.
struct ShapeMinus {
    bottom_left: Coordinate,
}

impl ShapeMinus {
    fn new(floor_y: usize) -> Self {
        ShapeMinus {
            bottom_left: Coordinate {
                x: 2,
                y: floor_y + 4,
            },
        }
    }
}

impl Shape for ShapeMinus {
    fn advance(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.bottom_left.x -= 1,
            Direction::Right => self.bottom_left.x += 1,
            Direction::Down => self.bottom_left.y -= 1,
        }
    }

    fn coordinates_filled(&self) -> Vec<Coordinate> {
        let mut res = Vec::with_capacity(4);
        for i in 0..4 {
            res.push(Coordinate {
                x: self.bottom_left.x + i,
                y: self.bottom_left.y,
            });
        }

        res
    }
}

/// Implementation of the plus shape.
struct ShapePlus {
    middle_left: Coordinate,
}

impl ShapePlus {
    fn new(floor_y: usize) -> Self {
        ShapePlus {
            middle_left: Coordinate {
                x: 2,
                y: floor_y + 5,
            },
        }
    }
}

impl Shape for ShapePlus {
    fn advance(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.middle_left.x -= 1,
            Direction::Right => self.middle_left.x += 1,
            Direction::Down => self.middle_left.y -= 1,
        }
    }

    fn coordinates_filled(&self) -> Vec<Coordinate> {
        let mut res = Vec::with_capacity(5);
        for i in 0..3 {
            res.push(Coordinate {
                x: self.middle_left.x + i,
                y: self.middle_left.y,
            });
        }

        res.push(Coordinate {
            x: self.middle_left.x + 1,
            y: self.middle_left.y + 1,
        });
        res.push(Coordinate {
            x: self.middle_left.x + 1,
            y: self.middle_left.y - 1,
        });

        res
    }
}

/// Implementation of the reverse L shape.
struct ShapeL {
    bottom_left: Coordinate,
}

impl ShapeL {
    fn new(floor_y: usize) -> Self {
        ShapeL {
            bottom_left: Coordinate {
                x: 2,
                y: floor_y + 4,
            },
        }
    }
}

impl Shape for ShapeL {
    fn advance(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.bottom_left.x -= 1,
            Direction::Right => self.bottom_left.x += 1,
            Direction::Down => self.bottom_left.y -= 1,
        }
    }

    fn coordinates_filled(&self) -> Vec<Coordinate> {
        let mut res = Vec::with_capacity(5);
        for i in 0..3 {
            res.push(Coordinate {
                x: self.bottom_left.x + i,
                y: self.bottom_left.y,
            });
        }

        res.push(Coordinate {
            x: self.bottom_left.x + 2,
            y: self.bottom_left.y + 1,
        });
        res.push(Coordinate {
            x: self.bottom_left.x + 2,
            y: self.bottom_left.y + 2,
        });

        res
    }
}

/// Implementation of the I shape.
struct ShapeI {
    bottom_left: Coordinate,
}

impl ShapeI {
    fn new(floor_y: usize) -> Self {
        ShapeI {
            bottom_left: Coordinate {
                x: 2,
                y: floor_y + 4,
            },
        }
    }
}

impl Shape for ShapeI {
    fn advance(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.bottom_left.x -= 1,
            Direction::Right => self.bottom_left.x += 1,
            Direction::Down => self.bottom_left.y -= 1,
        }
    }

    fn coordinates_filled(&self) -> Vec<Coordinate> {
        let mut res = Vec::with_capacity(4);
        for i in 0..4 {
            res.push(Coordinate {
                x: self.bottom_left.x,
                y: self.bottom_left.y + i,
            });
        }

        res
    }
}

/// Implementation of the square shape.
struct ShapeSquare {
    bottom_left: Coordinate,
}

impl ShapeSquare {
    fn new(floor_y: usize) -> Self {
        ShapeSquare {
            bottom_left: Coordinate {
                x: 2,
                y: floor_y + 4,
            },
        }
    }
}

impl Shape for ShapeSquare {
    fn advance(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.bottom_left.x -= 1,
            Direction::Right => self.bottom_left.x += 1,
            Direction::Down => self.bottom_left.y -= 1,
        }
    }

    fn coordinates_filled(&self) -> Vec<Coordinate> {
        let mut res = Vec::with_capacity(4);
        for x in 0..2 {
            for y in 0..2 {
                res.push(Coordinate {
                    x: self.bottom_left.x + x,
                    y: self.bottom_left.y + y,
                });
            }
        }

        res
    }
}

/// Error that occurred during parsing of the input.
#[derive(Debug, Error, PartialEq)]
pub enum InputError {
    /// Character in the input is invalid.
    #[error("Character {0} is invalid")]
    InvalidCharacter(char),
}

/// Input for the solution that can be parsed from a string.
#[derive(Debug, PartialEq)]
pub struct Input(Vec<Direction>);

impl FromStr for Input {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Map characters into Direction.
        let res: Result<Vec<_>, _> = s
            .trim()
            .chars()
            .map(|c| match c {
                '>' => Ok(Direction::Right),
                '<' => Ok(Direction::Left),
                c => Err(InputError::InvalidCharacter(c)),
            })
            .collect();

        Ok(Input(res?))
    }
}

/// Returns the shape of the rock for the specified index.
///
/// It sets the initial position of the rock to the correct y based
/// on the y of the floor
fn get_rock(rock_index: usize, floor_y: usize) -> Box<dyn Shape> {
    match rock_index % 5 {
        0 => Box::new(ShapeMinus::new(floor_y)),
        1 => Box::new(ShapePlus::new(floor_y)),
        2 => Box::new(ShapeL::new(floor_y)),
        3 => Box::new(ShapeI::new(floor_y)),
        4 => Box::new(ShapeSquare::new(floor_y)),
        _ => unreachable!(),
    }
}

/// Gets the y of the floor
fn get_floor_y(map: &Vec<Vec<bool>>) -> usize {
    for y in (0..map.len()).rev() {
        if map[y].iter().any(|x| *x) {
            return y;
        }
    }

    unreachable!()
}

/// Returns weather the shape will intersect with another shape, floor or borders of the map
/// if the move is performed.
fn will_intersect(
    map: &Vec<Vec<bool>>,
    coordinates_filled: &Vec<Coordinate>,
    direction: Direction,
) -> bool {
    let dx = direction.dx();
    let dy = direction.dy();

    for coord in coordinates_filled {
        let y = ((coord.y as i64) + dy) as usize;
        let x = coord.x as i64 + dx;
        if x < 0 || x > 6 {
            return true;
        }

        let x = x as usize;
        if y < map.len() && map[y][x] {
            return true;
        }
    }

    false
}

/// Returns the roof of the map.
fn get_roof(map: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut found = vec![false; 7];
    let mut y = get_floor_y(map);
    let mut roof = vec![];

    while found.contains(&false) {
        let row = &map[y];
        for i in 0..7 {
            found[i] |= row[i];
        }

        roof.insert(0, row.clone());
        if y > 0 {
            y -= 1;
        }
    }

    roof
}

/// Prints the map in an ASCII art. Used for debugging.
#[allow(dead_code)]
fn print_map(map: &Vec<Vec<bool>>) {
    for row in map.iter().rev() {
        let res = row
            .iter()
            .map(|block| if *block { "#" } else { "." })
            .collect::<String>();
        println!("{}", res);
    }
    println!();
}

/// Simulate falling of specified number of rocks.
fn simulate(mut nr_rocks: usize, moves: &Vec<Direction>) -> usize {
    // Initial the map with a floor.
    let mut map = vec![vec![true; 7]];
    let mut move_index = 0;

    // Initialize memo map.
    let mut memo = HashMap::new();
    // Use a flag for using memo, since it can be used only once.
    let mut used_memo = false;

    // Residual height after using the memo.
    let mut residual_height = 0;

    // Current rock index.
    let mut rock_index = 0;
    while rock_index < nr_rocks {
        // Use memo only if it hasn't been used already.
        if !used_memo {
            let roof = get_roof(&map);
            let cycle = memo.get(&roof);
            if let Some((start_height, rocks, prev_move_index)) = cycle {
                if rocks % 5 == rock_index % 5 && *prev_move_index == move_index {
                    // We have already seen this roof starting with the same rock and move.
                    // Use the cycle to our advantage.

                    // Calculate the height that one cycle gets you.
                    let diff_height = get_floor_y(&map) - start_height;
                    // Calculate the number of rocks used in one cycle.
                    let diff_rocks = rock_index - rocks;
                    // Calculate how many times we can repeat the cycle.
                    let nr_repeats = (nr_rocks - rocks) / diff_rocks;

                    // Reset the map, so that we only have the floor which is the roof.
                    let roof_height = roof.len();
                    map = roof;
                    // Calculate the height that we will be at after repeating as many cycles
                    // as possible. Offset the height by the height of the roof, since
                    // we will add that again when we calculate the height of the map.
                    residual_height = start_height + diff_height * nr_repeats - roof_height + 1;
                    // Reset the rock index to start from beginning.
                    rock_index = rock_index % 5;
                    // Reset the number of rocks we have to drop, to the number that is left.
                    // We have to offset it by the rock index.
                    nr_rocks = nr_rocks - rocks - nr_repeats * diff_rocks + rock_index;

                    // Remember that the memo has been used.
                    used_memo = true;
                }
            } else {
                // We haven't found a repeating roof yet, so we remember the current state
                // in the memo.
                let floor = get_floor_y(&map);
                memo.insert(roof.clone(), (floor, rock_index, move_index));
            }
        }

        // Get the y of the floor and initialize a new rock.
        let floor_y = get_floor_y(&map);
        let mut rock = get_rock(rock_index, floor_y);

        // Simulate the falling of the rock until it lands on something.
        loop {
            // Check if the rock can move in the left/right direction specified by the
            // jets of air. If it can move, update its position.
            let coordinates_filled = rock.coordinates_filled();
            if !will_intersect(&map, &coordinates_filled, moves[move_index]) {
                rock.advance(moves[move_index]);
            }
            move_index = (move_index + 1) % moves.len();

            // Check if rock lands when we move it down.
            let coordinates_filled = rock.coordinates_filled();
            if will_intersect(&map, &coordinates_filled, Direction::Down) {
                // The rock lands, we can remember its position on the map.
                for coord in coordinates_filled {
                    // Expend the map as needed.
                    if coord.y >= map.len() {
                        map.append(&mut vec![vec![false; 7]; coord.y - map.len() + 1]);
                    }
                    map[coord.y][coord.x] = true;
                }

                break;
            } else {
                // The rock doesn't land, we can update its position.
                rock.advance(Direction::Down);
            }
        }

        // Advance to the next rock.
        rock_index += 1;
    }

    // Calculate the final height
    get_floor_y(&map) + residual_height
}

/// Solution for day 17.
pub struct Solution;

impl Solver for Solution {
    type Input = Input;
    type Output = usize;

    fn get_day(&self) -> u32 {
        17
    }

    fn part_one(&self, input: &Self::Input) -> Self::Output {
        simulate(2022, &input.0)
    }

    fn part_two(&self, input: &Self::Input) -> Self::Output {
        simulate(1000000000000, &input.0)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::solution::day_17::{Direction, Input, Solution};
    use crate::Solver;

    #[test]
    fn parse_input() {
        let input = Input::from_str(">><>").unwrap();
        assert_eq!(
            input,
            Input(vec![
                Direction::Right,
                Direction::Right,
                Direction::Left,
                Direction::Right
            ])
        );
    }

    #[test]
    fn part_one() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_one(&input);
        assert_eq!(solution, 3068);
    }

    #[test]
    fn part_two() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_two(&input);
        assert_eq!(solution, 1514285714288);
    }

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
}
