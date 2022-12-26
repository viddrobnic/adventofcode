//! Contains solution for day 22

use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

use crate::Solver;

/// Direction in which we are turned.
#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn dx(&self) -> i32 {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
            _ => 0,
        }
    }

    fn dy(&self) -> i32 {
        match self {
            Direction::Up => -1,
            Direction::Down => 1,
            _ => 0,
        }
    }

    /// Returns current direction with applied rotation.
    fn rotate(&self, rotation: Rotation) -> Self {
        match (self, rotation) {
            (Direction::Left, Rotation::Positive) => Direction::Down,
            (Direction::Down, Rotation::Positive) => Direction::Right,
            (Direction::Right, Rotation::Positive) => Direction::Up,
            (Direction::Up, Rotation::Positive) => Direction::Left,
            (Direction::Left, Rotation::Negative) => Direction::Up,
            (Direction::Up, Rotation::Negative) => Direction::Right,
            (Direction::Right, Rotation::Negative) => Direction::Down,
            (Direction::Down, Rotation::Negative) => Direction::Left,
        }
    }
}

/// Represents a rotation.
#[derive(Debug, PartialEq, Copy, Clone)]
enum Rotation {
    Positive,
    Negative,
}

/// Represents a single move.
#[derive(Debug, PartialEq, Copy, Clone)]
enum Move {
    Forward(i32),
    Rotation(Rotation),
}

/// Represents a tile.
#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
    /// Empty tile has nothing on it and we can't move to it.
    Empty,
    /// Open tile has nothing on it and we can move to it
    Open,
    /// Solid tile is a wall that blocks us
    Solid,
}

/// Error that occurred during parsing of the input.
#[derive(Debug, Error)]
pub enum InputError {
    /// Got invalid tile.
    #[error("")]
    InvalidTile(char),

    /// Tried parsing invalid number.
    #[error("")]
    InvalidNumber {
        #[from]
        source: ParseIntError,
    },
}

/// Input for solution that can be parsed from a string.
#[derive(Debug, PartialEq, Clone)]
pub struct Input {
    map: Vec<Vec<Tile>>,
    moves: Vec<Move>,
    start_x: i32,
    start_y: i32,
}

impl FromStr for Input {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().filter(|line| *line != "").collect();
        let map = parse_map(&lines[..lines.len() - 1])?;
        let moves = parse_moves(&lines[lines.len() - 1])?;

        let x = find_start_x(&map, 1);

        Ok(Input {
            map,
            moves,
            start_x: x,
            start_y: 1,
        })
    }
}

/// Parses map from an array of lines.
fn parse_map(lines: &[&str]) -> Result<Vec<Vec<Tile>>, InputError> {
    let map: Result<Vec<Vec<_>>, _> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    ' ' => Ok(Tile::Empty),
                    '.' => Ok(Tile::Open),
                    '#' => Ok(Tile::Solid),
                    c => Err(InputError::InvalidTile(c)),
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect();
    let mut map = map?;

    // Pad the map so that all lines are of the same length and the whole map
    // is padded with empty lines.
    let max_len = map.iter().map(|row| row.len()).max().unwrap() + 1;
    for i in 0..map.len() {
        let mut filler = vec![Tile::Empty; max_len - map[i].len()];
        map[i].insert(0, Tile::Empty);
        map[i].append(&mut filler);
    }

    map.insert(0, vec![Tile::Empty; max_len + 1]);
    map.push(vec![Tile::Empty; max_len + 1]);

    Ok(map)
}

/// Parses moves from a line.
fn parse_moves(line: &str) -> Result<Vec<Move>, InputError> {
    let mut res = Vec::new();
    let mut start = 0;
    for i in 0..line.len() {
        match &line[i..i + 1] {
            "R" => {
                let number: i32 = line[start..i].parse()?;
                res.push(Move::Forward(number));
                res.push(Move::Rotation(Rotation::Negative));
                start = i + 1;
            }
            "L" => {
                let number: i32 = line[start..i].parse()?;
                res.push(Move::Forward(number));
                res.push(Move::Rotation(Rotation::Positive));
                start = i + 1;
            }
            _ => (),
        }
    }

    let number: i32 = line[start..].parse()?;
    res.push(Move::Forward(number));

    Ok(res)
}

/// Returns the first x that contains an open tile for the given y.
fn find_start_x(map: &Vec<Vec<Tile>>, y: usize) -> i32 {
    map[y]
        .iter()
        .position(|tile| *tile == Tile::Open)
        .expect("Map should not have only empty tiles") as i32
}

/// Moves forwards and uses the given wrap function to handle moving out of bounds.
fn move_forward<F>(
    map: &Vec<Vec<Tile>>,
    mut direction: Direction,
    steps: i32,
    mut x: i32,
    mut y: i32,
    wrap: F,
) -> (i32, i32, Direction)
where
    F: Fn(&Vec<Vec<Tile>>, i32, i32, Direction) -> (i32, i32, Direction),
{
    for _ in 0..steps {
        // Move one step
        let (dx, dy) = (direction.dx(), direction.dy());
        let mut current_x = x + dx;
        let mut current_y = y + dy;
        let mut current_direction = direction;

        // Handle stepping on an empty tile by calling the wrap function
        if map[current_y as usize][current_x as usize] == Tile::Empty {
            (current_x, current_y, current_direction) =
                wrap(map, current_x, current_y, current_direction);
        }

        match map[current_y as usize][current_x as usize] {
            Tile::Open => {
                // We are able to move, so we change the position and rotation.
                x = current_x;
                y = current_y;
                direction = current_direction;
            }
            // We can't move to the wall.
            Tile::Solid => break,
            Tile::Empty => panic!("There should not be empty tiles here!"),
        }
    }

    (x, y, direction)
}

/// Wrap function for part one.
///
/// If finds the first non empty tile in the direction based on the x, y and given direction.
fn wrap_part_one(
    map: &Vec<Vec<Tile>>,
    x: i32,
    y: i32,
    direction: Direction,
) -> (i32, i32, Direction) {
    match direction {
        Direction::Left => (
            map[y as usize]
                .iter()
                .rposition(|tile| *tile != Tile::Empty)
                .unwrap() as i32,
            y,
            direction,
        ),
        Direction::Right => (
            map[y as usize]
                .iter()
                .position(|tile| *tile != Tile::Empty)
                .unwrap() as i32,
            y,
            direction,
        ),
        Direction::Up => (
            x,
            map.iter()
                .map(|row| row[x as usize])
                .rposition(|tile| tile != Tile::Empty)
                .unwrap() as i32,
            direction,
        ),
        Direction::Down => (
            x,
            map.iter()
                .map(|row| row[x as usize])
                .position(|tile| tile != Tile::Empty)
                .unwrap() as i32,
            direction,
        ),
    }
}

/// Wrap function for part two.
///
/// Hardcoded wrapping for my input. I know that this is very ugly.
fn wrap_part_two(
    _: &Vec<Vec<Tile>>,
    x: i32,
    y: i32,
    direction: Direction,
) -> (i32, i32, Direction) {
    if y == 0 && x > 50 && x <= 100 && direction == Direction::Up {
        (1, 150 + x - 50, Direction::Right)
    } else if y == 0 && x > 100 && x <= 150 && direction == Direction::Up {
        (x - 100, 200, Direction::Up)
    } else if x == 151 && y > 0 && y <= 50 && direction == Direction::Right {
        (100, 151 - y, Direction::Left)
    } else if y == 51 && x > 100 && x <= 150 && direction == Direction::Down {
        (100, 50 + x - 100, Direction::Left)
    } else if x == 101 && y > 50 && y <= 100 && direction == Direction::Right {
        (100 + y - 50, 50, Direction::Up)
    } else if x == 101 && y > 100 && y <= 150 && direction == Direction::Right {
        (150, 51 - (y - 100), Direction::Left)
    } else if y == 151 && x > 50 && x <= 100 && direction == Direction::Down {
        (50, 150 + x - 50, Direction::Left)
    } else if x == 51 && y > 150 && y <= 200 && direction == Direction::Right {
        (50 + y - 150, 150, Direction::Up)
    } else if y == 201 && x > 0 && x <= 50 && direction == Direction::Down {
        (x + 100, 1, Direction::Down)
    } else if x == 0 && y > 150 && y <= 200 && direction == Direction::Left {
        (50 + y - 150, 1, Direction::Down)
    } else if x == 0 && y > 100 && y <= 150 && direction == Direction::Left {
        (51, 51 - (y - 100), Direction::Right)
    } else if y == 100 && x > 0 && x <= 50 && direction == Direction::Up {
        (51, 50 + x, Direction::Right)
    } else if x == 50 && y > 50 && y <= 100 && direction == Direction::Left {
        (y - 50, 101, Direction::Down)
    } else if x == 50 && y > 0 && y <= 50 && direction == Direction::Left {
        (1, 151 - y, Direction::Right)
    } else {
        unreachable!()
    }
}

/// Calculates the final score.
fn score(x: i32, y: i32, direction: Direction) -> i64 {
    let row = y as i64;
    let column = x as i64;
    let facing = match direction {
        Direction::Left => 2,
        Direction::Right => 0,
        Direction::Up => 3,
        Direction::Down => 1,
    };

    1000 * row + 4 * column + facing
}

/// Solves the problem with given wrap function.
fn solve<F>(input: &Input, wrap: F) -> i64
where
    F: Fn(&Vec<Vec<Tile>>, i32, i32, Direction) -> (i32, i32, Direction),
{
    let mut x = input.start_x;
    let mut y = input.start_y;

    let mut direction = Direction::Right;

    for mv in input.moves.iter() {
        match mv {
            Move::Forward(steps) => {
                (x, y, direction) = move_forward(&input.map, direction, *steps, x, y, &wrap)
            }
            Move::Rotation(rotation) => direction = direction.rotate(*rotation),
        }
    }

    score(x, y, direction)
}

/// Solution for day 22.
pub struct Solution;

impl Solver for Solution {
    type Input = Input;
    type Output = i64;

    fn get_day(&self) -> u32 {
        22
    }

    fn part_one(&self, input: &Self::Input) -> Self::Output {
        solve(input, wrap_part_one)
    }

    fn part_two(&self, input: &Self::Input) -> Self::Output {
        solve(input, wrap_part_two)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::solution::day_22::{Input, Move, Rotation, Solution};
    use crate::Solver;

    #[test]
    fn part_one() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_one(&input);
        assert_eq!(solution, 6032);
    }

    #[test]
    fn parse_input() {
        let input = Input::from_str(INPUT).unwrap();
        assert_eq!(
            input.moves,
            vec![
                Move::Forward(10),
                Move::Rotation(Rotation::Negative),
                Move::Forward(5),
                Move::Rotation(Rotation::Positive),
                Move::Forward(5),
                Move::Rotation(Rotation::Negative),
                Move::Forward(10),
                Move::Rotation(Rotation::Positive),
                Move::Forward(4),
                Move::Rotation(Rotation::Negative),
                Move::Forward(5),
                Move::Rotation(Rotation::Positive),
                Move::Forward(5),
            ]
        );

        assert_eq!(input.start_x, 9);
    }

    const INPUT: &str = r#"
        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
"#;
}
