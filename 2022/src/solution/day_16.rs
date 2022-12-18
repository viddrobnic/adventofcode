//! Contains solution for day 16.

use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

use crate::Solver;

/// Error that occurred during parsing of the input.
#[derive(Debug, Error)]
pub enum InputError {
    /// Input contains an invalid line.
    #[error("Invalid input line: {0}")]
    InvalidLine(String),

    /// Input contains an invalid number.
    #[error("Invalid number: {source}")]
    InvalidNumber {
        #[from]
        source: ParseIntError,
    },
}

/// Set of integers.
///
/// Maximum 64 integers can be in a set. It is implemented using bitwise operations
/// therefore it's fast, but has a limited size.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct IntSet(u64);

impl IntSet {
    /// Creates a new empty set.
    fn new() -> Self {
        IntSet(0)
    }

    /// Inserts a number into the set and returns the resulting set.
    fn insert(&self, n: i32) -> Self {
        let res = self.0 | (1 << n);
        IntSet(res)
    }

    /// Returns weather the set contains the given number.
    fn contains(&self, n: i32) -> bool {
        self.0 & (1 << n) > 0
    }

    /// Returns weather the two sets intersect.
    fn intersects(&self, other: &Self) -> bool {
        self.0 & other.0 > 0
    }
}

/// Input for the solution that can be parsed from string.
#[derive(Debug, PartialEq)]
pub struct Input {
    nr_valves: usize,
    start_valve: usize,
    flow_rates: Vec<i32>,
    distances: Vec<Vec<i32>>,
}

impl FromStr for Input {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut flow_rates = HashMap::new();
        let mut tunnels = HashMap::new();

        // Iterate through cleaned lines
        for line in s.lines().map(|line| line.trim()).filter(|line| *line != "") {
            // Split lines into parts
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 10 {
                return Err(InputError::InvalidLine(line.to_string()));
            }

            // Get valve name
            let tunnel = parts[1].to_string();

            // Get valve flow
            let flow = parts[4].replace(";", "");
            let flow = flow
                .split("=")
                .nth(1)
                .ok_or(InputError::InvalidLine(line.to_string()))?;
            let flow = flow.parse::<i32>()?;

            // Get neighbouring valves.
            let valves: HashSet<String> = parts[9..]
                .iter()
                .map(|dest| dest.replace(",", ""))
                .collect();

            flow_rates.insert(tunnel.clone(), flow);
            tunnels.insert(tunnel, valves);
        }

        Ok(Input::prepare(&flow_rates, &tunnels))
    }
}

impl Input {
    /// Prepares the input from map of flow rates an valves.
    fn prepare(
        flow_rates: &HashMap<String, i32>,
        tunnels: &HashMap<String, HashSet<String>>,
    ) -> Input {
        // Get each valve it's index from 0 to nr_valves.
        let mut indices = HashMap::new();
        for (i, (valve, _)) in flow_rates.iter().enumerate() {
            indices.insert(valve.clone(), i);
        }

        // Get number of valves and the starting valve.
        let nr_valves = indices.len();
        let starting_valve = *indices
            .get("AA")
            .expect("Valid input should have a starting valve");

        // Get flow rates as array, where flow rate for each valve is at it's index.
        let mut flow_rates_vec = vec![0; nr_valves];
        for (valve, flow_rate) in flow_rates.iter() {
            if let Some(i) = indices.get(valve) {
                flow_rates_vec[*i] = *flow_rate;
            }
        }

        // Get distances as a matrix. Number in column i and row j represents the distance
        // between valves with indices i and j.
        // First set the distances of length 1, which are given to use as the input.
        let mut distances = vec![vec![100_000; nr_valves]; nr_valves];
        for (valve, neighbours) in tunnels.iter() {
            let valve_index = match indices.get(valve) {
                None => continue,
                Some(i) => *i,
            };

            for neighbour in neighbours.iter() {
                let neighbour_index = match indices.get(neighbour) {
                    None => continue,
                    Some(i) => *i,
                };

                distances[valve_index][neighbour_index] = 1;
            }
        }

        // Set the diagonal to 0. Valve is of distance 0 from itself.
        for i in 0..nr_valves {
            distances[i][i] = 0;
        }

        // Use Floyd-Warshall to calculate the distance between two arbitrary valves.
        for middle in 0..nr_valves {
            for start in 0..nr_valves {
                for end in 0..nr_valves {
                    distances[start][end] = min(
                        distances[start][end],
                        distances[start][middle] + distances[middle][end],
                    );
                }
            }
        }

        // Construct the final input.
        Input {
            nr_valves,
            distances,
            start_valve: starting_valve,
            flow_rates: flow_rates_vec,
        }
    }
}

/// Helper function for solving the puzzle.
///
/// Its purpose is to populate the memo dictionary. Memo dictionary represents
/// pairs of opened valves and max flow achieved through them.
fn solver(
    distances: &Vec<Vec<i32>>,
    flows: &Vec<i32>,
    start_valve: usize,
    time_left: i32,
    flow: i32,
    opened_valves: IntSet,
    memo: &mut HashMap<IntSet, i32>,
) {
    // Update the current max flow in the memo.
    let current = memo.get(&opened_valves).unwrap_or(&0);
    memo.insert(opened_valves, max(*current, flow));

    // Go through the neighbours and recursively populate the memo.
    for (neighbour, distance) in distances[start_valve].iter().enumerate() {
        // If distance is too long, skip the neighbour.
        if *distance >= time_left {
            continue;
        }

        // If neighbour is the same as the starting valve skip it.
        // This is done to avoid cycling indefensibly.
        if neighbour == start_valve {
            continue;
        }

        // If neighbour's flow is 0, there is no point in visiting it.
        if flows[neighbour] == 0 {
            continue;
        }

        // If neighbour's valve is already opened, there is no point in visiting it.
        if opened_valves.contains(neighbour as i32) {
            continue;
        }

        // Open the neighbour's valve and continue from there.
        let new_opened_valves = opened_valves.insert(neighbour as i32);
        let new_time_left = time_left - distance - 1;
        solver(
            distances,
            flows,
            neighbour,
            new_time_left,
            flow + flows[neighbour] * new_time_left,
            new_opened_valves,
            memo,
        );
    }
}

/// Solution for day 16.
pub struct Solution;

impl Solver for Solution {
    type Input = Input;
    type Output = i32;

    fn get_day(&self) -> u32 {
        16
    }

    fn part_one(&self, input: &Self::Input) -> Self::Output {
        // Populate the memo of states.
        let mut memo = HashMap::new();
        solver(
            &input.distances,
            &input.flow_rates,
            input.start_valve,
            30,
            0,
            IntSet::new(),
            &mut memo,
        );

        // Find the maximum flow achieved of all the states.
        *memo
            .values()
            .max()
            .expect("Valid input should have a solution")
    }

    fn part_two(&self, input: &Self::Input) -> Self::Output {
        // Populate the memo of states.
        let mut memo = HashMap::new();
        let _ = solver(
            &input.distances,
            &input.flow_rates,
            input.start_valve,
            26,
            0,
            IntSet::new(),
            &mut memo,
        );

        // Find two disjunct states that have the larges sum. The sum of these two disjunct states
        // is the solution (the elephant can take the path to achieve one of the states, and
        // you can take the other).
        let mut max_flow = 0;
        for (opened_valves1, flow1) in memo.iter() {
            for (opened_valves2, flow2) in memo.iter() {
                if !opened_valves1.intersects(&opened_valves2) {
                    let flow = flow1 + flow2;
                    max_flow = max(max_flow, flow);
                }
            }
        }

        max_flow
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::solution::day_16::{Input, Solution};
    use crate::Solver;

    #[test]
    fn parse_input() {
        let input = r#"
            Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
            Valve BB has flow rate=13; tunnels lead to valves CC, AA
            Valve CC has flow rate=0; tunnels lead to valves BB
            "#;
        let input = Input::from_str(input).unwrap();

        assert_eq!(input.nr_valves, 3);
        assert_eq!(input.flow_rates.iter().max(), Some(&13));
    }

    #[test]
    fn part_one() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_one(&input);
        assert_eq!(solution, 1651);
    }

    #[test]
    fn part_two() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_two(&input);
        assert_eq!(solution, 1707);
    }

    const INPUT: &str = r#"
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II
    "#;
}
