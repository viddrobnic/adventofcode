//! Contains solution for day 13.
//!
use std::cmp::Ordering;
use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

use crate::Solver;

/// Error that occurred during parsing of the input.
#[derive(Debug, Error)]
pub enum InputError {
    /// Input contains an invalid number
    #[error("Invalid number in input: {source}")]
    InvalidNumber {
        #[from]
        source: ParseIntError,
    },

    /// Input has invalid starting character
    #[error("Invalid starting character: {0}")]
    InvalidStartingCharacter(String),

    /// Input contains a list that wasn't closed.
    #[error("List was not closed")]
    ListNotClosed,

    /// Input contains invalid number of packets.
    #[error("Number of packets should be divisible by 2, but it isn't: {0}")]
    InvalidNumberOfPackets(usize),
}

/// Represents a single packet. It can either be an integer or a list of packets.
#[derive(Debug, PartialEq, Clone)]
enum Packet {
    Integer(i32),
    List(Vec<Packet>),
}

impl FromStr for Packet {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (packet, _) = Packet::parse_from_str(s, 0)?;
        Ok(packet)
    }
}

impl Packet {
    /// Parses packet from string.
    fn parse_from_str(val: &str, mut index: usize) -> Result<(Packet, usize), InputError> {
        if &val[index..index + 1] == "[" {
            // Parse list.
            let mut list = Vec::new();
            index += 1;

            // Indices representing start and end of the buffer
            // containing the current number.
            let mut buf_start = index;
            let mut buf_end = buf_start;

            while index < val.len() {
                match &val[index..index + 1] {
                    "," => {
                        // Parse the number in the buffer and update the buffer indices.
                        let num = parse_buffer(val, buf_start, buf_end)?;
                        if let Some(num) = num {
                            list.push(Packet::Integer(num));
                        }
                        buf_start = buf_end + 1;
                        buf_end = buf_start;
                    }
                    "[" => {
                        // Parse a list by recursively calling this function.
                        let (packet, new_start) = Packet::parse_from_str(val, index)?;
                        list.push(packet);

                        // Update the current index and buffer indices.
                        index = new_start + 1;
                        buf_start = index;
                        buf_end = index;

                        continue;
                    }
                    "]" => {
                        // Finish parsing a list by returning the current packet.
                        let num = parse_buffer(val, buf_start, buf_end)?;
                        if let Some(num) = num {
                            list.push(Packet::Integer(num));
                        }

                        return Ok((Packet::List(list), index));
                    }
                    _ => {
                        // Update the buffer index.
                        buf_end += 1;
                    }
                }

                // Update the index.
                index += 1;
            }

            Err(InputError::ListNotClosed)
        } else {
            // Input shuld alwasy start with a list.
            Err(InputError::InvalidStartingCharacter(
                val[index..index + 1].to_string(),
            ))
        }
    }

    /// Compare two packets.
    fn cmp(&self, other: &Packet) -> Ordering {
        match (self, other) {
            // If packets are integers, compare the integers normally.
            (Packet::Integer(left), Packet::Integer(right)) => left.cmp(right),
            (Packet::List(left), Packet::List(right)) => {
                // Compare element wise. If all elements are the same,
                // list lengths become important.
                let mut order = Ordering::Equal;
                for (left, right) in left.iter().zip(right.iter()) {
                    match left.cmp(right) {
                        Ordering::Less => {
                            order = Ordering::Less;
                            break;
                        }
                        Ordering::Greater => {
                            order = Ordering::Greater;
                            break;
                        }
                        _ => (),
                    }
                }

                // Handle the non same cases
                if order == Ordering::Less {
                    return Ordering::Less;
                }

                if order == Ordering::Greater {
                    return Ordering::Greater;
                }

                // If all elements are the same, compare list lengths.
                left.len().cmp(&right.len())
            }
            (Packet::Integer(left), right) => {
                // If left is an integer, convert it into a list and compare the lists.
                let left_list = Packet::List(vec![Packet::Integer(*left)]);
                left_list.cmp(right)
            }
            (left, Packet::Integer(right)) => {
                // If right is an integer, convert it into a list and compare the lists.
                let right_list = Packet::List(vec![Packet::Integer(*right)]);
                left.cmp(&right_list)
            }
        }
    }
}

/// Parses buffer for a packet.
///
/// If the buffer is empty, `None` is returned,
/// otherwise the number in the buffer is parsed.
fn parse_buffer(
    val: &str,
    buffer_start: usize,
    buffer_end: usize,
) -> Result<Option<i32>, InputError> {
    if buffer_start == buffer_end {
        Ok(None)
    } else {
        let number = val[buffer_start..buffer_end].parse()?;
        Ok(Some(number))
    }
}

/// Input for the solution that can be parsed from a string.
pub struct Input(Vec<Packet>);

impl FromStr for Input {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let packets: Result<Vec<Packet>, _> = s
            .lines()
            .map(|line| line.trim()) // Trim whitespace
            .filter(|line| *line != "") // Remove empty lines
            .map(|line| line.parse()) // Parse into packets
            .collect();

        Ok(Input(packets?))
    }
}

pub struct Solution;

impl Solver for Solution {
    type Input = Input;
    type Output = i32;

    fn get_day(&self) -> u32 {
        13
    }

    fn part_one(&self, input: &Self::Input) -> Self::Output {
        let mut res = 0;

        // Using `step_by` and `zip`, group the input into pairs.
        for (i, (left, right)) in input
            .0
            .iter()
            .step_by(2)
            .zip(input.0.iter().skip(1).step_by(2))
            .enumerate()
        {
            // If pair is ordered correctly, add the index to result.
            if left.cmp(right) == Ordering::Less {
                res += i + 1;
            }
        }

        res as i32
    }

    fn part_two(&self, input: &Self::Input) -> Self::Output {
        // Construct and add divider packets into the input.
        let mut packets = input.0.clone();
        let divider_packet_1 = Packet::from_str("[[2]]").unwrap();
        let divider_packet_2 = Packet::from_str("[[6]]").unwrap();

        packets.push(divider_packet_1.clone());
        packets.push(divider_packet_2.clone());

        // Sort the input.
        packets.sort_unstable_by(|left, right| left.cmp(right));

        // Find the positions of the divider packets.
        let pos_1 = packets
            .iter()
            .position(|packet| *packet == divider_packet_1)
            .unwrap()
            + 1;

        let pos_2 = packets
            .iter()
            .position(|packet| *packet == divider_packet_2)
            .unwrap()
            + 1;

        // Return the multiplication of the two positions.
        (pos_1 * pos_2) as i32
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::solution::day_13::{Input, Packet, Solution};
    use crate::Solver;

    #[test]
    fn part_one() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_one(&input);
        assert_eq!(solution, 13);
    }

    #[test]
    fn part_two() {
        let input = Input::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_two(&input);
        assert_eq!(solution, 140);
    }

    #[test]
    fn parse_packet() {
        assert_eq!(
            Packet::from_str("[1,1,3,1,1]").unwrap(),
            Packet::List(vec![
                Packet::Integer(1),
                Packet::Integer(1),
                Packet::Integer(3),
                Packet::Integer(1),
                Packet::Integer(1),
            ]),
        );

        assert_eq!(
            Packet::from_str("[1,1,5,1,1]").unwrap(),
            Packet::List(vec![
                Packet::Integer(1),
                Packet::Integer(1),
                Packet::Integer(5),
                Packet::Integer(1),
                Packet::Integer(1),
            ]),
        );

        assert_eq!(
            Packet::from_str("[[1],[2,3,4]]").unwrap(),
            Packet::List(vec![
                Packet::List(vec![Packet::Integer(1)]),
                Packet::List(vec![
                    Packet::Integer(2),
                    Packet::Integer(3),
                    Packet::Integer(4)
                ]),
            ]),
        );

        assert_eq!(
            Packet::from_str("[[1],4]").unwrap(),
            Packet::List(vec![
                Packet::List(vec![Packet::Integer(1)]),
                Packet::Integer(4),
            ]),
        );

        assert_eq!(
            Packet::from_str("[9]").unwrap(),
            Packet::List(vec![Packet::Integer(9),]),
        );

        assert_eq!(
            Packet::from_str("[[8,7,6]]").unwrap(),
            Packet::List(vec![Packet::List(vec![
                Packet::Integer(8),
                Packet::Integer(7),
                Packet::Integer(6)
            ]),]),
        );

        assert_eq!(
            Packet::from_str("[[4,4],4,4]").unwrap(),
            Packet::List(vec![
                Packet::List(vec![Packet::Integer(4), Packet::Integer(4),]),
                Packet::Integer(4),
                Packet::Integer(4),
            ]),
        );

        assert_eq!(Packet::from_str("[]").unwrap(), Packet::List(vec![]),);

        assert_eq!(
            Packet::from_str("[[]]").unwrap(),
            Packet::List(vec![Packet::List(vec![])]),
        );

        assert_eq!(
            Packet::from_str("[[[]]]").unwrap(),
            Packet::List(vec![Packet::List(vec![Packet::List(vec![])])]),
        );

        assert_eq!(
            Packet::from_str("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap(),
            Packet::List(vec![
                Packet::Integer(1),
                Packet::List(vec![
                    Packet::Integer(2),
                    Packet::List(vec![
                        Packet::Integer(3),
                        Packet::List(vec![
                            Packet::Integer(4),
                            Packet::List(vec![
                                Packet::Integer(5),
                                Packet::Integer(6),
                                Packet::Integer(7)
                            ])
                        ])
                    ])
                ]),
                Packet::Integer(8),
                Packet::Integer(9),
            ])
        );
    }

    const INPUT: &str = r#"
        [1,1,3,1,1]
        [1,1,5,1,1]

        [[1],[2,3,4]]
        [[1],4]

        [9]
        [[8,7,6]]

        [[4,4],4,4]
        [[4,4],4,4,4]

        [7,7,7,7]
        [7,7,7]

        []
        [3]

        [[[]]]
        [[]]

        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]
        "#;
}
