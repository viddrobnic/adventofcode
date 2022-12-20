//! Contains solution for day 19.

use std::cmp::min;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;
use std::thread;

use crate::Solver;

/// Costs for creating ore robot.
#[derive(Debug, PartialEq, Clone)]
struct OreCost {
    ore: i32,
}

/// Costs for creating clay robot.
#[derive(Debug, PartialEq, Clone)]
struct ClayCost {
    ore: i32,
}

/// Costs for creating obsidian robot.
#[derive(Debug, PartialEq, Clone)]
struct ObsidianCost {
    ore: i32,
    clay: i32,
}

/// Costs for creating geode robot.
#[derive(Debug, PartialEq, Clone)]
struct GeodeCost {
    ore: i32,
    obsidian: i32,
}

/// Blueprint containing the costs for all the robots.
#[derive(Debug, PartialEq, Clone)]
struct Blueprint {
    ore_cost: OreCost,
    clay_cost: ClayCost,
    obsidian_cost: ObsidianCost,
    geode_cost: GeodeCost,
}

impl FromStr for Blueprint {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split_whitespace().collect();

        Ok(Blueprint {
            ore_cost: OreCost {
                ore: parts[6].parse()?,
            },
            clay_cost: ClayCost {
                ore: parts[12].parse()?,
            },
            obsidian_cost: ObsidianCost {
                ore: parts[18].parse()?,
                clay: parts[21].parse()?,
            },
            geode_cost: GeodeCost {
                ore: parts[27].parse()?,
                obsidian: parts[30].parse()?,
            },
        })
    }
}

pub struct Input(Vec<Blueprint>);

impl FromStr for Input {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: Result<Vec<_>, _> = s
            .lines()
            .map(|line| line.trim())
            .filter(|line| *line != "")
            .map(|line| line.parse())
            .collect();

        Ok(Input(res?))
    }
}

/// Helper function for solving the problem.
///
/// The input is the whole state and the output is the final result.
/// The idea is that we check which robots we can build and try building
/// them. That puts us into a new state.
fn solve(
    ore: i32,
    clay: i32,
    obsidian: i32,
    geodes: i32,
    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,
    time: i32,
    blueprint: &Blueprint,
    memo: &mut HashMap<(i32, i32, i32, i32, i32, i32, i32, i32, i32), i32>,
) -> i32 {
    if time < 0 {
        return 0;
    }

    if time == 0 {
        return geodes;
    }

    let key = (
        ore,
        clay,
        obsidian,
        geodes,
        ore_robots,
        clay_robots,
        obsidian_robots,
        geode_robots,
        time,
    );
    if let Some(res) = memo.get(&key) {
        return *res;
    }

    // Create another ore robot.
    let ore_res = if ore_robots >= blueprint.ore_cost.ore
        && ore_robots >= blueprint.clay_cost.ore
        && ore_robots >= blueprint.obsidian_cost.ore
        && ore_robots >= blueprint.geode_cost.ore
    {
        0
    } else {
        let time_needed = time_needed_to_produce(ore, ore_robots, blueprint.ore_cost.ore);

        if let Some(time_needed) = time_needed {
            solve(
                ore + time_needed * ore_robots - blueprint.ore_cost.ore,
                clay + time_needed * clay_robots,
                obsidian + time_needed * obsidian_robots,
                geodes + time_needed * geode_robots,
                ore_robots + 1,
                clay_robots,
                obsidian_robots,
                geode_robots,
                time - time_needed,
                blueprint,
                memo,
            )
        } else {
            0
        }
    };

    // Create another clay robot.
    let clay_res = if clay_robots >= blueprint.obsidian_cost.clay {
        0
    } else {
        let time_needed = time_needed_to_produce(ore, ore_robots, blueprint.clay_cost.ore);

        if let Some(time_needed) = time_needed {
            solve(
                ore + time_needed * ore_robots - blueprint.clay_cost.ore,
                clay + time_needed * clay_robots,
                obsidian + time_needed * obsidian_robots,
                geodes + time_needed * geode_robots,
                ore_robots,
                clay_robots + 1,
                obsidian_robots,
                geode_robots,
                time - time_needed,
                blueprint,
                memo,
            )
        } else {
            0
        }
    };

    // Create another obsidian robot
    let obsidian_res = if obsidian_robots >= blueprint.geode_cost.obsidian {
        0
    } else {
        let time_needed_ore = time_needed_to_produce(ore, ore_robots, blueprint.obsidian_cost.ore);
        let time_needed_clay =
            time_needed_to_produce(clay, clay_robots, blueprint.obsidian_cost.clay);
        let time_needed = max_time(time_needed_ore, time_needed_clay);

        if let Some(time_needed) = time_needed {
            solve(
                ore + time_needed * ore_robots - blueprint.obsidian_cost.ore,
                clay + time_needed * clay_robots - blueprint.obsidian_cost.clay,
                obsidian + time_needed * obsidian_robots,
                geodes + time_needed * geode_robots,
                ore_robots,
                clay_robots,
                obsidian_robots + 1,
                geode_robots,
                time - time_needed,
                blueprint,
                memo,
            )
        } else {
            0
        }
    };

    // Create another geode robot.
    let time_needed_ore = time_needed_to_produce(ore, ore_robots, blueprint.geode_cost.ore);
    let time_needed_obsidian =
        time_needed_to_produce(obsidian, obsidian_robots, blueprint.geode_cost.obsidian);
    let time_needed = max_time(time_needed_ore, time_needed_obsidian);

    let geode_res = if let Some(time_needed) = time_needed {
        solve(
            ore + time_needed * ore_robots - blueprint.geode_cost.ore,
            clay + time_needed * clay_robots,
            obsidian + time_needed * obsidian_robots - blueprint.geode_cost.obsidian,
            geodes + time_needed * geode_robots,
            ore_robots,
            clay_robots,
            obsidian_robots,
            geode_robots + 1,
            time - time_needed,
            blueprint,
            memo,
        )
    } else {
        0
    };

    // What if we don't produce a robot.
    let nothing_res = geodes + time * geode_robots;

    let res = [ore_res, clay_res, obsidian_res, geode_res, nothing_res];
    let res = res.iter().max();
    let res = *res.unwrap();

    memo.insert(key, res);
    res
}

/// Helper function that returns the time needed to produce another robot based
/// on the current resources `resource`, number of robots for the resource we have `robots` and
/// cost of a new robot `cost`.
fn time_needed_to_produce(resource: i32, robots: i32, cost: i32) -> Option<i32> {
    let ore_needed = cost - resource;
    if ore_needed <= 0 {
        return Some(1);
    }

    if robots == 0 {
        return None;
    }

    if ore_needed % robots == 0 {
        Some(ore_needed / robots + 1)
    } else {
        Some(ore_needed / robots + 2)
    }
}

/// Returns the maximum between two times.
///
/// If any time is None, returns None.
fn max_time(t1: Option<i32>, t2: Option<i32>) -> Option<i32> {
    if t1 == None || t2 == None {
        return None;
    }

    Some(t1.unwrap().max(t2.unwrap()))
}

pub struct Solution;

impl Solver for Solution {
    type Input = Input;
    type Output = i32;

    fn get_day(&self) -> u32 {
        19
    }

    fn part_one(&self, input: &Self::Input) -> Self::Output {
        // Spawn a thread to determine the max geodes for each blueprint.
        let handles: Vec<_> = input
            .0
            .iter()
            .map(|blueprint| {
                let blueprint = blueprint.clone();
                thread::spawn(move || {
                    let mut memo = HashMap::new();
                    solve(0, 0, 0, 0, 1, 0, 0, 0, 24, &blueprint, &mut memo)
                })
            })
            .collect();

        // Sum the results
        handles
            .into_iter()
            .enumerate()
            .map(|(i, handle)| (i as i32 + 1) * handle.join().unwrap())
            .sum()
    }

    fn part_two(&self, input: &Self::Input) -> Self::Output {
        // Spawn a thread to determine the max geodes for first three blueprints
        let handles: Vec<_> = input.0[..min(3, input.0.len())]
            .iter()
            .map(|blueprint| {
                let blueprint = blueprint.clone();
                thread::spawn(move || {
                    let mut memo = HashMap::new();
                    solve(0, 0, 0, 0, 1, 0, 0, 0, 32, &blueprint, &mut memo)
                })
            })
            .collect();

        // Multiply the results
        handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .product()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::str::FromStr;

    use crate::solution::day_19::{
        solve, Blueprint, ClayCost, GeodeCost, Input, ObsidianCost, OreCost, Solution,
    };
    use crate::Solver;

    #[test]
    fn parse_input() {
        let blueprint = Blueprint::from_str("Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 19 clay. Each geode robot costs 2 ore and 12 obsidian.").unwrap();
        assert_eq!(
            blueprint,
            Blueprint {
                ore_cost: OreCost { ore: 3 },
                clay_cost: ClayCost { ore: 3 },
                obsidian_cost: ObsidianCost { ore: 2, clay: 19 },
                geode_cost: GeodeCost {
                    ore: 2,
                    obsidian: 12
                },
            }
        );
    }

    #[test]
    fn part_one() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_one(&input);
        assert_eq!(solution, 33);
    }

    #[test]
    fn part_two() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_two(&input);
        assert_eq!(solution, 62);
    }

    const INPUT: &str = r#"
        Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
        Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
        "#;
}
