//! Contains solution for day 20.

use std::num::ParseIntError;
use std::str::FromStr;

use crate::Solver;

/// Represents a number together with an index at which it appears in the original input.
#[derive(Debug, PartialEq, Copy, Clone)]
struct Number {
    value: i64,
    order_index: usize,
}

/// Input for the solution that can be parsed from a string.
#[derive(Debug, PartialEq)]
pub struct Input(Vec<Number>);

impl FromStr for Input {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res: Result<Vec<_>, ParseIntError> = s
            .lines()
            .map(|line| line.trim())
            .filter(|line| *line != "")
            .enumerate()
            .map(|(index, line)| {
                Ok(Number {
                    value: line.parse()?,
                    order_index: index,
                })
            })
            .collect();

        Ok(Input(res?))
    }
}

/// Moves a number with given order_index.
fn move_number(list: &mut Vec<Number>, order_index: usize) {
    // Find the index of the number.
    let index = list
        .iter()
        .position(|number| number.order_index == order_index)
        .expect(&format!("Invalid order index: {}", order_index));

    // Remove the number from the list.
    let number = list.remove(index);

    // Add the number back at the correct location.
    // We have to calculate the modulo with original list length - 1, because of the wrapping.
    let new_index = (index as i64 + number.value).rem_euclid(list.len() as i64);
    list.insert(new_index as usize, number);
}

/// Performs a single mixing on the list.
fn mix(list: &mut Vec<Number>) {
    for i in 0..list.len() {
        move_number(list, i)
    }
}

/// Finds the sum of numbers at indices from instructions.
fn find_coordinates_sum(list: &Vec<Number>) -> i64 {
    // Find the index of 0.
    let index = list.iter().position(|number| number.value == 0).unwrap();

    // Calculate the indices of the numbers from 0 to 1000.
    let index1 = (index + 1000).rem_euclid(list.len());
    let index2 = (index + 2000).rem_euclid(list.len());
    let index3 = (index + 3000).rem_euclid(list.len());
    list.iter()
        .enumerate()
        .filter(|(index, _)| *index == index1 || *index == index2 || *index == index3)
        .map(|(_, number)| number.value)
        .sum()
}

/// Prints only the values in the list.
///
/// Used for debugging purposes.
#[allow(dead_code)]
fn print_list(list: &Vec<Number>) {
    let to_print = list
        .iter()
        .map(|number| number.value.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", to_print);
}

pub struct Solution;

impl Solver for Solution {
    type Input = Input;
    type Output = i64;

    fn get_day(&self) -> u32 {
        20
    }

    fn part_one(&self, input: &Self::Input) -> Self::Output {
        let mut list = input.0.clone();
        mix(&mut list);
        find_coordinates_sum(&list)
    }

    fn part_two(&self, input: &Self::Input) -> Self::Output {
        let decryption_key = 811589153;
        let mut list: Vec<Number> = input
            .0
            .iter()
            .map(|number| Number {
                value: number.value * decryption_key,
                order_index: number.order_index,
            })
            .collect();

        for _ in 0..10 {
            mix(&mut list);
        }

        find_coordinates_sum(&list)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::LinkedList;
    use std::str::FromStr;

    use crate::solution::day_20::{move_number, Input, Number, Solution};
    use crate::Solver;

    #[test]
    fn part_one() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_one(&input);
        assert_eq!(solution, 3);
    }

    #[test]
    fn part_two() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_two(&input);
        assert_eq!(solution, 1623178306);
    }

    #[test]
    fn move_number_test() {
        let mut list = Vec::from([
            Number {
                value: 0,
                order_index: 0,
            },
            Number {
                value: 2,
                order_index: 1,
            },
            Number {
                value: 0,
                order_index: 2,
            },
        ]);
        move_number(&mut list, 1);
        assert_eq!(list.iter().nth(1).unwrap().value, 2);

        let mut list = Vec::from([
            Number {
                value: 0,
                order_index: 0,
            },
            Number {
                value: 8,
                order_index: 1,
            },
            Number {
                value: 0,
                order_index: 2,
            },
        ]);
        move_number(&mut list, 1);
        assert_eq!(list.iter().nth(1).unwrap().value, 8);

        let mut list = Vec::from([
            Number {
                value: 0,
                order_index: 0,
            },
            Number {
                value: 5,
                order_index: 1,
            },
            Number {
                value: 0,
                order_index: 2,
            },
        ]);
        move_number(&mut list, 1);
        assert_eq!(list.iter().nth(0).unwrap().value, 5);
    }

    #[test]
    fn parse_input() {
        let input = Input::from_str(INPUT).unwrap();
        assert_eq!(
            input,
            Input(Vec::from([
                Number {
                    value: 1,
                    order_index: 0,
                },
                Number {
                    value: 2,
                    order_index: 1,
                },
                Number {
                    value: -3,
                    order_index: 2,
                },
                Number {
                    value: 3,
                    order_index: 3,
                },
                Number {
                    value: -2,
                    order_index: 4,
                },
                Number {
                    value: 0,
                    order_index: 5,
                },
                Number {
                    value: 4,
                    order_index: 6,
                },
            ]))
        );
    }

    const INPUT: &str = r#"
        1
        2
        -3
        3
        -2
        0
        4
        "#;
}
