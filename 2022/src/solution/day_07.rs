//! Contains solution for day 7.

use crate::Solver;
use std::str::FromStr;
use thiserror::Error;

/// Error that occurred during parsing of the input.
#[derive(Debug, Error)]
pub enum InputError {
    /// Given starting line of a new node is invalid. Starting line of a new node should start
    /// with '$ cd dir_name'.
    #[error("Given start line is not a valid command ({0}). Valid command is of form '$ cd ...'")]
    InvalidStartLine(String),

    /// Given second line of a new node is invalid. Second line of a new node should be '$ ls'
    #[error("Given second line is not a valid command ({0}). Valid command is of form '$ ls'")]
    InvalidSecondLine(String),

    /// Given line output by a 'ls' command is invalid.
    #[error(
        "Given line does not represent a valid file node({0}). It is not a file nor a directory."
    )]
    InvalidFileNode(String),
}

/// Input for the solution that can be parsed from a string.
///
/// Represents a single node in a file system. A node can be either
/// - a file
/// - a directory
#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    File {
        name: String,
        size: u64,
    },
    Directory {
        name: String,
        children: Vec<Box<Node>>,
        size: u64,
    },
}

impl FromStr for Node {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Clean the input by trimming whitespace and filtering out empty lines.
        let lines: Vec<&str> = s
            .lines()
            .map(|val| val.trim())
            .filter(|val| *val != "")
            .collect();

        // Call a helper method to construct a node from lines.
        let (node, _) = Node::from_lines(&lines, 0)?;
        Ok(node)
    }
}

impl Node {
    pub fn new_directory(name: String) -> Self {
        Node::Directory {
            name,
            children: vec![],
            size: 0,
        }
    }

    /// Constructs a node from command lines.
    /// The first line of this node is at index `start_index`.
    /// It returns the node and the index of a line in which the input for this node ended.
    fn from_lines(lines: &Vec<&str>, start_index: usize) -> Result<(Node, usize), InputError> {
        // Split the starting line into parts and check their length.
        let parts: Vec<&str> = lines[start_index].split_whitespace().collect();
        if parts.len() != 3 {
            return Err(InputError::InvalidStartLine(lines[start_index].to_string()));
        }

        // Check that the first two lines of the input are correct
        if parts[0] != "$" || parts[1] != "cd" {
            return Err(InputError::InvalidStartLine(lines[start_index].to_string()));
        }
        if lines[start_index + 1] != "$ ls" {
            return Err(InputError::InvalidSecondLine(
                lines[start_index + 1].to_string(),
            ));
        }

        // Construct a new empty node
        let mut node = Self::new_directory(parts[2].to_string());

        // Parse the lines for the given node.
        let mut i = start_index + 2;
        while i < lines.len() {
            let line = lines[i];

            // If navigating upwards, we are done with this node and can exit the recursion call.
            if line == "$ cd .." {
                return Ok((node, i + 1));
            }

            if &line[0..1] == "$" {
                // If line is starting with "$", we are going one node deeper.
                // We parse the child node with a recursive call and add it to the current node.
                let (child, new_index) = Node::from_lines(lines, i)?;
                node.add_child(child);
                i = new_index;
                continue;
            } else {
                // Add a child node to the current node, based on its type.
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() != 2 {
                    return Err(InputError::InvalidFileNode(line.to_string()));
                }

                if parts[0] == "dir" {
                    // Add a new empty directory
                    let child = Self::new_directory(parts[1].to_string());
                    node.add_child(child);
                } else {
                    // Add a new file. First parse the size of the file to a number.
                    let size: u64 = parts[0]
                        .parse()
                        .map_err(|_| InputError::InvalidFileNode(line.to_string()))?;

                    // Construct and add the file to the current node.
                    let child = Node::File {
                        name: parts[1].to_string(),
                        size,
                    };
                    node.add_child(child);
                }
            }

            i += 1;
        }

        Ok((node, i))
    }

    /// Adds a child to the current node.
    ///
    /// A child can only be added to the Directory node. If this function is called on a
    /// File node, it will panic.
    ///
    /// If the current node already has a child with the same name,
    /// the existing child is replaced.
    fn add_child(&mut self, child: Node) {
        match self {
            Node::File { .. } => panic!("Cannot add child to a file node!"),
            Node::Directory { children, size, .. } => {
                // Check if a node with the same name exists as the child.
                let existing = children.iter().position(|node| node.name() == child.name());
                match existing {
                    None => {
                        // The node does not exist, we can just add the new one.
                        *size += child.size();
                        children.push(Box::new(child));
                    }
                    Some(index) => {
                        // The node does exist, we replace the old one. We also fix the size
                        // of the current node.
                        let child_size = child.size();
                        let old_child = std::mem::replace(&mut children[index], Box::new(child));
                        *size = *size - old_child.size() + child_size;
                    }
                }
            }
        }
    }

    /// Returns the name of the node
    fn name(&self) -> &str {
        match self {
            Node::File { name, .. } => name,
            Node::Directory { name, .. } => name,
        }
    }

    /// Returns the size of the node.
    fn size(&self) -> u64 {
        match self {
            Node::File { size, .. } => *size,
            Node::Directory { size, .. } => *size,
        }
    }
}

/// Solution for day 7.
pub struct Solution;

impl Solver for Solution {
    type Input = Node;
    type Output = u64;

    fn get_day(&self) -> u32 {
        7
    }

    fn part_one(&self, input: &Self::Input) -> Self::Output {
        match input {
            // If current node is a file, the sum of the children is 0.
            Node::File { .. } => 0,
            Node::Directory { children, size, .. } => {
                // Get the sum of the solutions for all the children.
                let children_sum: u64 = children.iter().map(|child| self.part_one(child)).sum();
                if *size <= 100000 {
                    // If the size of the current directory is small enough, add it to the final solution.
                    children_sum + size
                } else {
                    // Return only the sum of the children's solution.
                    children_sum
                }
            }
        }
    }

    fn part_two(&self, input: &Self::Input) -> Self::Output {
        // Calculate how much space should be freed.
        let total_disk_space: u64 = 70000000;
        let needed_unused = 30000000;
        let must_free = match input {
            Node::File { .. } => panic!("Input should be a directory if parsed correctly"),
            Node::Directory { size, .. } => {
                let unused = total_disk_space - size;
                needed_unused - unused
            }
        };

        // Calculate the solution with the help of recursion.
        part_two_acc(input, must_free, None).expect("If input is valid, there must be a solution")
    }
}

// Returns the size of the smallest directory that we need to delete.
fn part_two_acc(node: &Node, must_free: u64, current_smallest: Option<u64>) -> Option<u64> {
    match node {
        // If the current node is a file, than the smallest directory we need to delete stays the same.
        Node::File { .. } => current_smallest,
        Node::Directory { children, size, .. } => {
            // Check if this directory is a candidate for deletion.
            // If the deletion of this node will not free enough space, there is no point
            // in looking at the children, since they are smaller than the parent by definition.
            if *size < must_free {
                return current_smallest;
            }

            // Get the solution for all the children using a recursive call.
            // If a recursive call on a child returns None, it is excluded by the filter.
            let min_child = children
                .iter()
                .map(|child| part_two_acc(child, must_free, current_smallest))
                .filter(|res| res.is_some())
                .min_by_key(|res| res.unwrap());
            match min_child {
                None => {
                    // No child is a candidate for deletion. Compare this node with the current
                    // smallest node.
                    match current_smallest {
                        None => Some(*size),
                        Some(min_size) if *size < min_size=> Some(*size),
                        _ => current_smallest
                    }
                }
                // We filter the children's solutions so that they are not None, so this can't happen.
                Some(None) => panic!("Cannot happen, since the min_child is filtered so that None is not a valid value"),
                Some(Some(min_size)) => {
                    // There was a child that is a candidate for deletion. We can only compare to
                    // this child, since the child is smaller than `current_smallest`.
                    if *size < min_size {
                        Some(*size)
                    } else {
                        Some(min_size)
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::day_07::{Node, Solution};
    use crate::Solver;
    use std::str::FromStr;

    const INPUT: &str = r#"
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
        "#;

    #[test]
    fn parse_input() {
        let node = Node::from_str(INPUT).unwrap();

        assert_eq!(
            node,
            Node::Directory {
                name: "/".to_string(),
                children: vec![
                    Box::new(Node::Directory {
                        name: "a".to_string(),
                        children: vec![
                            Box::new(Node::Directory {
                                name: "e".to_string(),
                                children: vec![Box::new(Node::File {
                                    name: "i".to_string(),
                                    size: 584,
                                })],
                                size: 584,
                            }),
                            Box::new(Node::File {
                                name: "f".to_string(),
                                size: 29116,
                            }),
                            Box::new(Node::File {
                                name: "g".to_string(),
                                size: 2557,
                            }),
                            Box::new(Node::File {
                                name: "h.lst".to_string(),
                                size: 62596,
                            }),
                        ],
                        size: 94_853,
                    }),
                    Box::new(Node::File {
                        name: "b.txt".to_string(),
                        size: 14848514
                    }),
                    Box::new(Node::File {
                        name: "c.dat".to_string(),
                        size: 8504156
                    }),
                    Box::new(Node::Directory {
                        name: "d".to_string(),
                        children: vec![
                            Box::new(Node::File {
                                name: "j".to_string(),
                                size: 4060174,
                            }),
                            Box::new(Node::File {
                                name: "d.log".to_string(),
                                size: 8033020,
                            }),
                            Box::new(Node::File {
                                name: "d.ext".to_string(),
                                size: 5626152,
                            }),
                            Box::new(Node::File {
                                name: "k".to_string(),
                                size: 7214296,
                            }),
                        ],
                        size: 24_933_642,
                    }),
                ],
                size: 48_381_165,
            }
        );
    }

    #[test]
    fn part_one() {
        let input = Node::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_one(&input);
        assert_eq!(solution, 95437);
    }

    #[test]
    fn part_two() {
        let input = Node::from_str(INPUT).unwrap();
        let solver = Solution;

        let solution = solver.part_two(&input);
        assert_eq!(solution, 24933642);
    }
}
