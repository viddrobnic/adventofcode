//! Contains solution for day 15.
use std::cmp::{max, min};
use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

use crate::Solver;

/// Error that occurred during parsing of the input.
#[derive(Debug, Error, PartialEq)]
pub enum InputError {
    /// Coordinate is invalid.
    #[error("Got invalid coordinate: {0}")]
    InvalidCoordinate(String),

    /// Number in coordinate is invalid.
    #[error("Coordinate number is invalid: {source}")]
    InvalidCoordinateNumber {
        #[from]
        source: ParseIntError,
    },
}

/// Represents a point in a 2D space.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    /// Returns Manhattan distance between two points.
    fn distance(&self, other: &Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

/// A closed interval.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Interval {
    start: i64,
    end: i64,
}

impl Interval {
    /// Returns weather the two intervals intersect.
    fn intersects(&self, other: &Interval) -> bool {
        self.contains_number(other.start)
            || self.contains_number(other.end)
            || other.contains_number(self.start)
            || other.contains_number(self.end)
    }

    /// Returns weather the interval contains a given number.
    fn contains_number(&self, number: i64) -> bool {
        number >= self.start && number <= self.end
    }

    /// Returns an union of the two intervals.
    ///
    /// Intervals should intersect for this method to give a correct result.
    fn union(&self, other: &Interval) -> Interval {
        Interval {
            start: min(self.start, other.start),
            end: max(self.end, other.end),
        }
    }

    /// Returns the length of the interval.
    fn length(&self) -> usize {
        (self.end - self.start + 1) as usize
    }
}

/// Sensor and its closest beacon
#[derive(Debug, PartialEq)]
struct Sensor {
    position: Point,
    closest_beacon: Point,
}

impl FromStr for Sensor {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split line into parts.
        let parts: Vec<&str> = s.trim().split_whitespace().collect();

        // Get the coordinates from the line
        let x = parse_coordinate(parts[2])?;
        let y = parse_coordinate(parts[3])?;
        let beacon_x = parse_coordinate(parts[8])?;
        let beacon_y = parse_coordinate(parts[9])?;

        // Construct the sensor
        Ok(Sensor {
            position: Point { x, y },
            closest_beacon: Point {
                x: beacon_x,
                y: beacon_y,
            },
        })
    }
}

/// Input that can be parsed from a string.
pub struct Input(Vec<Sensor>);

impl FromStr for Input {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: Result<Vec<Sensor>, _> = s
            .lines()
            .map(|line| line.trim()) // Trim whitespace
            .filter(|line| *line != "") // Filter out empty lines
            .map(|line| line.parse()) // Parse into sensor
            .collect();

        // Sort the sensors by (x, y) pair and construct the Input.
        let mut res = res?;
        res.sort_unstable_by_key(|sensor| (sensor.position.x, sensor.position.y));
        Ok(Input(res))
    }
}

/// Parse a single coordinate from a string.
fn parse_coordinate(coordinate: &str) -> Result<i64, InputError> {
    let cleaned = coordinate.replace(",", "").replace(":", "");
    let number = cleaned
        .split("=")
        .nth(1)
        .ok_or(InputError::InvalidCoordinate(coordinate.to_string()))?;

    let number = number.parse()?;
    Ok(number)
}

/// Returns number of filled positions in a line with the given y coordinate.
///
/// Position is filled if the there can't be a beacon we don't know about there.
fn filled_positions_in_line(sensors: &Vec<Sensor>, y: i64) -> (usize, Vec<Interval>) {
    let mut intervals: Vec<Interval> = Vec::new();
    for sensor in sensors.iter() {
        // Get the radius of the sphere surrounding the sensor in which there can't be a beacon.
        let radius = sensor.position.distance(&sensor.closest_beacon);

        // Calculate the maximum x distance for the given y, on which there can't be a beacon.
        // This means that on the given y, the interval [sensor.x - x_distance, sensor.x + x_distance]
        // does not contain another beacon.
        let x_distance = radius - (sensor.position.y - y).abs();
        if x_distance < 0 {
            continue;
        }

        // Add interval to the list of intervals using bisection.
        // If exactly the same interval already exists, do nothing.
        let interval = Interval {
            start: sensor.position.x - x_distance,
            end: sensor.position.x + x_distance,
        };
        let index = intervals
            .binary_search_by_key(&(interval.start, interval.end), |it| (it.start, it.end));
        let mut index = match index {
            Ok(_) => continue,
            Err(i) => i,
        };

        intervals.insert(index, interval);

        // Determine the index at which we start merging the intervals with union.
        // If the interval before inserted one intersects with it, we start at its index.
        // Otherwise we start on the index at which the new interval was inserted.
        if index > 0 && intervals[index - 1].intersects(&interval) {
            index -= 1;
        }

        // While the interval at given index intersects with the next interval, merge them together.
        while index < intervals.len() - 1 && intervals[index].intersects(&intervals[index + 1]) {
            let other = intervals.remove(index + 1);
            intervals[index] = intervals[index].union(&other);
        }
    }

    // Calculate total length of all the intervals.
    let sum: usize = intervals.iter().map(|interval| interval.length()).sum();
    (sum, intervals)
}

/// Finds the position at which a beacon can exist, but we can't know about it.
fn find_beacon(sensors: &Vec<Sensor>, search_space: usize) -> Point {
    // Scan the search space vertically.
    for y in 0..=search_space {
        // Get intervals in which the beacon can not exist for the given y.
        let (_, intervals) = filled_positions_in_line(sensors, y as i64);

        // Go through the intervals and find an empty space between them.
        for (i, interval) in intervals.iter().enumerate() {
            // If there is space between 0 and first interval start,
            // the beacon can be at coordinate (0, y).
            if i == 0 && interval.start > 0 {
                return Point { x: 0, y: y as i64 };
            }

            // Check if there is space between the end of the current interval
            // and the start of the next interval. If there is space, between them,
            // the coordinate can be located there.
            let x = interval.end + 1;
            if i < intervals.len() - 1 && !intervals[i + 1].contains_number(x) {
                return Point { x, y: y as i64 };
            }
        }
    }

    // If the input is valid, we can't reach this point of code.
    unreachable!();
}

/// Solution for day 15.
pub struct Solution;

impl Solver for Solution {
    type Input = Input;
    type Output = i64;

    fn get_day(&self) -> u32 {
        15
    }

    fn part_one(&self, input: &Self::Input) -> Self::Output {
        let y = 2000000;
        // Get the number of filled positions in given y.
        let (filled_positions, intervals) = filled_positions_in_line(&input.0, y);

        // Get the number of beacons located on the current y and in one of the intervals.
        // This is the number of beacons that we counted, but we should not count in the
        // solution.
        let count_set: HashSet<_> = input
            .0
            .iter()
            .filter(|sensor| {
                let same_y = sensor.closest_beacon.y == y;
                let in_interval = intervals
                    .iter()
                    .any(|interval| interval.contains_number(sensor.closest_beacon.y));
                same_y && in_interval
            })
            .map(|sensor| sensor.closest_beacon.x)
            .collect();

        // Calculate the final result.
        (filled_positions - count_set.len()) as i64
    }

    fn part_two(&self, input: &Self::Input) -> Self::Output {
        // Find the beacon in the given search space.
        let search_space = 4000000;
        let beacon = find_beacon(&input.0, search_space);

        // Calculate the resulting number.
        beacon.x * (search_space as i64) + beacon.y
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::solution::day_15::{filled_positions_in_line, find_beacon, Input, Point, Sensor};

    #[test]
    fn parse_input() {
        let sensor =
            Sensor::from_str("Sensor at x=2, y=18: closest beacon is at x=-2, y=15").unwrap();
        assert_eq!(
            sensor,
            Sensor {
                position: Point { x: 2, y: 18 },
                closest_beacon: Point { x: -2, y: 15 },
            }
        );

        let sensor =
            Sensor::from_str("Sensor at x=20, y=14: closest beacon is at x=25, y=17").unwrap();
        assert_eq!(
            sensor,
            Sensor {
                position: Point { x: 20, y: 14 },
                closest_beacon: Point { x: 25, y: 17 },
            }
        );
    }

    #[test]
    fn part_one() {
        let input = Input::from_str(INPUT).unwrap();
        let (solution, _) = filled_positions_in_line(&input.0, 10);
        assert_eq!(solution, 27);
    }

    #[test]
    fn part_two() {
        let input = Input::from_str(INPUT).unwrap();
        let solution = find_beacon(&input.0, 20);
        assert_eq!(solution, Point { x: 14, y: 11 });
    }

    const INPUT: &str = r#"
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3
        "#;
}
