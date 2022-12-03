//! Contains the solution for day 3.

use std::collections::HashSet;
use std::str::FromStr;

use crate::Solver;

// Represents an item
#[derive(PartialEq, Eq, Hash, Debug)]
struct Item(char);

impl Item {
    // Returns the priority of the item.
    // Items with values between 'a' and 'z' have priorities between 1 and 26.
    // Items with values between 'A' and 'Z' have priorities between 27 and 52.
    fn priority(&self) -> i32 {
        if self.0.is_uppercase() {
            self.0 as i32 - 'A' as i32 + 27
        } else {
            self.0 as i32 - 'a' as i32 + 1
        }
    }
}

// Represents a single rucksack.
#[derive(PartialEq, Debug)]
struct Rucksack {
    first_compartment: HashSet<Item>,
    second_compartment: HashSet<Item>,
}

impl Rucksack {
    // Returns the set of items in both compartments.
    fn items(&self) -> HashSet<&Item> {
        self.first_compartment
            .union(&self.second_compartment)
            .collect()
    }
}

/// Input for the solution that can be parsed from a string
#[derive(PartialEq, Debug)]
pub struct Input(Vec<Rucksack>);

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result: Vec<_> = s
            .lines() // Get lines
            .map(|val| val.trim()) // Trim the whitespace
            .filter(|val| *val != "") // Filter out empty lines
            .map(|val: &str| {
                // Determine the middle of the rucksack items.
                let mid = val.len() / 2;

                // Construct a new rucksack with empty hash sets.
                let mut result = Rucksack {
                    first_compartment: HashSet::with_capacity(mid),
                    second_compartment: HashSet::with_capacity(mid),
                };

                // Add items to the correct compartment.
                for (index, c) in val.chars().enumerate() {
                    if index < mid {
                        result.first_compartment.insert(Item(c));
                    } else {
                        result.second_compartment.insert(Item(c));
                    }
                }

                result
            })
            .collect();

        Ok(Input(result))
    }
}

/// Solution for day 3
pub struct Solution;

impl Solver for Solution {
    type Input = Input;
    type Output = i32;

    fn get_day(&self) -> u32 {
        3
    }

    fn part_one(&self, input: &Self::Input) -> Self::Output {
        // Using fold we sum up all the priorities.
        input.0.iter().fold(0, |acc, elt: &Rucksack| {
            // For each item we get the intersection between first and second compartment.
            // After that we get the priority of the element in the intersection using another fold.
            // There should be only one item in the intersection, but it works even for multiple items.
            // We return the sum of the current accumulator and the priority of the intersection
            // between both compartments.
            acc + elt
                .first_compartment
                .intersection(&elt.second_compartment)
                .fold(0, |acc, item: &Item| acc + item.priority())
        })
    }

    fn part_two(&self, input: &Self::Input) -> Self::Output {
        let input = &input.0;
        // Group the rucksacks into groups of three. We do that by combining zip, skip and step_by.
        // This result into an iterator over (Rucksack, (Rucksack, Rucksack)).
        let groups = input.iter().step_by(3).zip(
            input
                .iter()
                .skip(1)
                .step_by(3)
                .zip(input.iter().skip(2).step_by(3)),
        );

        // Using fold again we sum up all the priorities.
        groups.fold(
            0,
            |acc, (r1, (r2, r3)): (&Rucksack, (&Rucksack, &Rucksack))| {
                // Get items for all rucksacks.
                let r1 = r1.items();
                let r2 = r2.items();
                let r3 = r3.items();

                // Calculate the intersection of all rucksacks. We need to use an additional map
                // to convert &&Item into &Item.
                // Like in part_one, we get the priority of the item in the intersection
                // using another fold.
                // We return the sum of the current accumulator and the priority of the intersection
                // between all three rucksacks in a group.
                let intersection: HashSet<_> = r1.intersection(&r2).map(|item| *item).collect();
                acc + intersection
                    .intersection(&r3)
                    .map(|item| *item)
                    .fold(0, |acc, item: &Item| acc + item.priority())
            },
        )
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use std::str::FromStr;

    use crate::solution::day_03::{Input, Item, Rucksack, Solution};
    use crate::Solver;

    #[test]
    fn item_priority() {
        assert_eq!(Item('a').priority(), 1);
        assert_eq!(Item('b').priority(), 2);
        assert_eq!(Item('z').priority(), 26);
        assert_eq!(Item('A').priority(), 27);
        assert_eq!(Item('B').priority(), 28);
        assert_eq!(Item('Z').priority(), 52);
    }

    const INPUT: &str = r#"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
        "#;

    #[test]
    fn parse_input() {
        let input = Input::from_str("abcd").unwrap();
        assert_eq!(
            input,
            Input(vec![Rucksack {
                first_compartment: HashSet::from([Item('a'), Item('b')]),
                second_compartment: HashSet::from([Item('c'), Item('d')]),
            }])
        );
    }

    #[test]
    fn part_one() {
        let solver = Solution;
        let input = Input::from_str(INPUT).unwrap();

        let solution = solver.part_one(&input);
        assert_eq!(solution, 157);
    }

    #[test]
    fn part_two() {
        let solver = Solution;
        let input = Input::from_str(INPUT).unwrap();

        let solution = solver.part_two(&input);
        assert_eq!(solution, 70);
    }
}
