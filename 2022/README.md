# Advent of Code 2022

This repository contains my solutions to the [2022's Advent of Code](https://adventofcode.com/2022).
This year I have decided to write my solutions in Rust and to try to write as clean code as possible. The solutions are
way more complicated than they need to be. They are also way more commented than they need to be.

## Progress

|  Day   | Name                    | Rust |
|:------:|:------------------------|:----:|
| Day 1  | Calorie Counting        |  🎄  |
| Day 2  | Rock Paper Scissors     |  🎄  |
| Day 3  | Rucksack Reorganization |  🎄  |
| Day 4  | Camp Cleanup            |  🎄  |
| Day 5  | ??                      |  ❄️  |
| Day 6  | ??                      |  ❄️  |
| Day 7  | ??                      |  ❄️  |
| Day 8  | ??                      |  ❄️  |
| Day 9  | ??                      |  ❄️  |
| Day 10 | ??                      |  ❄️  |
| Day 11 | ??                      |  ❄️  |
| Day 12 | ??                      |  ❄️  |
| Day 13 | ??                      |  ❄️  |
| Day 14 | ??                      |  ❄️  |
| Day 15 | ??                      |  ❄️  |
| Day 16 | ??                      |  ❄️  |
| Day 17 | ??                      |  ❄️  |
| Day 18 | ??                      |  ❄️  |
| Day 19 | ??                      |  ❄️  |
| Day 20 | ??                      |  ❄️  |
| Day 21 | ??                      |  ❄️  |
| Day 22 | ??                      |  ❄️  |
| Day 23 | ??                      |  ❄️  |
| Day 24 | ??                      |  ❄️  |
| Day 25 | ??                      |  ❄️  |

## Running

Program can be run with `cargo run`. There are two options:

- `cargo run all` runs all days
- `cargo run days x` run only specified days. You can specify more than one day by separating numbers with space.

The program expects the problem inputs to be located at `inputs/day_XX.txt`.

## Project structure

Project is structured into bin and lib part. The bin part is contained in the `main.rs` and handles parsing the CLI
input and running the solution.

The library part is the contains solutions to the problems. Library defines a trait `Solver` that all
solutions implement. Types implementing `Solver` define their input and output types and implement the solving of both
parts of the problem. This way parsing the input is lifted out from solving the problem, which makes error handling
better, as described [here](https://mmapped.blog/posts/12-rust-error-handling.html#lift-input-validation).

Library also defines and implements `PrintSolver` for all `Solvers`s. `PrintSolver` handles reading the input from the
file, solving the problem and writing the solution to the stdout. It also handles benchmarking of the solution.

More implementation details are described in rust docs, which can built and opened with

```bash
cargo doc --open
```
