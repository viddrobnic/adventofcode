//! Contains solution for day 6.

use crate::Solver;

/// Solution for day 6.
pub struct Solution;

impl Solver for Solution {
    type Input = String;
    type Output = usize;

    fn get_day(&self) -> u32 {
        6
    }

    fn part_one(&self, input: &Self::Input) -> Self::Output {
        find_marker(input, 4)
    }

    fn part_two(&self, input: &Self::Input) -> Self::Output {
        find_marker(input, 14)
    }
}

// Finds marker in the input. A marker is a substring of length `length` in which
// all characters are unique.
fn find_marker(input: &str, length: usize) -> usize {
    for i in length..=input.len() {
        // Substring that might be a marker.
        let substr = &input[i - length..i];

        // Check if all characters are unique. All characters are unique
        // if for every index j, the character at index j is not repeated in
        // the rest of the string after j.
        let mut marker_found = true;
        for (j, c) in substr.chars().enumerate() {
            if substr[j + 1..].contains(c) {
                marker_found = false;
                break;
            }
        }

        if marker_found {
            return i;
        }
    }

    // This should not happen.
    panic!("Day 06 failed! We haven't found the solution, but there should be one.");
}

#[cfg(test)]
mod tests {
    use crate::solution::day_06::Solution;
    use crate::Solver;

    const INPUTS: [(&str, usize, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
    ];

    #[test]
    fn part_one() {
        let solver = Solution;

        for (input, expected, _) in INPUTS.iter() {
            let owned_input = input.to_string();
            let actual = solver.part_one(&owned_input);
            assert_eq!(*expected, actual);
        }
    }

    #[test]
    fn part_two() {
        let solver = Solution;

        for (input, _, expected) in INPUTS.iter() {
            let owned_input = input.to_string();
            let actual = solver.part_two(&owned_input);
            assert_eq!(*expected, actual);
        }
    }
}
