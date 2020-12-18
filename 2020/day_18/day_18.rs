use std::fs;

#[derive(Debug)]
enum Expression {
    Constant(u64),
    Plus(Box<Expression>, Box<Expression>),
    Times(Box<Expression>, Box<Expression>),
}

impl Expression {
    fn next_op(e: &str, start: usize) -> Option<usize> {
        let mut level = 0;
        for (i, c) in e.chars().enumerate() {
            if c == '(' {
                level += 1;
            } else if c == ')' {
                level -= 1;
            }

            if level == 0 && (c == '+' || c == '*') {
                return Some(i + start);
            }
        }

        None
    }

    fn operator_precedance(op: &str, plus_precedance: i32, times_precedance: i32) -> i32 {
        match op {
            "+" => plus_precedance,
            "*" => times_precedance,
            _ => panic!("Invalid operator"),
        }
    }

    fn from_str(mut e: &str, plus_precedance: i32, times_precedance: i32) -> Expression {
        // Check if constant.
        if let Ok(v) = e.parse() {
            return Expression::Constant(v);
        }

        // Remove unneeded parenthesis.
        let mut clean = false;
        while !clean {
            let mut level = 0;
            for (i, c) in e.chars().enumerate() {
                if c == '(' {
                    level += 1;
                } else if c == ')' {
                    level -= 1;
                }

                if level == 0 && i != (e.len() - 1) {
                    clean = true;
                    break;
                }
            }

            if !clean {
                e = &e[1..(e.len() - 1)];
            }
        }

        // Find operator.
        let mut operator = Expression::next_op(e, 0).unwrap();
        while let Some(next_operator) = Expression::next_op(&e[(operator + 1)..], operator + 1) {
            let current_precedance = Expression::operator_precedance(
                &e[operator..(operator + 1)],
                plus_precedance,
                times_precedance,
            );
            let next_precedance = Expression::operator_precedance(
                &e[next_operator..(next_operator + 1)],
                plus_precedance,
                times_precedance,
            );

            if next_precedance <= current_precedance {
                operator = next_operator;
            } else {
                break;
            }
        }

        let op = &e[operator..(operator + 1)];
        let lhs = Expression::from_str(&e[..operator], plus_precedance, times_precedance);
        let rhs = Expression::from_str(&e[(operator + 1)..], plus_precedance, times_precedance);
        match op {
            "*" => Expression::Times(Box::new(lhs), Box::new(rhs)),
            "+" => Expression::Plus(Box::new(lhs), Box::new(rhs)),
            _ => panic!("Invalid operator"),
        }
    }

    fn eval(&self) -> u64 {
        match self {
            Expression::Constant(x) => *x,
            Expression::Times(lhs, rhs) => lhs.eval() * rhs.eval(),
            Expression::Plus(lhs, rhs) => lhs.eval() + rhs.eval(),
        }
    }
}

fn main() {
    let data: Vec<_> = fs::read_to_string("in")
        .unwrap()
        .lines()
        .map(|l| l.trim().replace(" ", ""))
        .collect();

    println!("Part One: {}", part_one(&data));
    println!("Part Two: {}", part_two(&data));
}

fn part_one(data: &Vec<String>) -> u64 {
    data.iter()
        .map(|line| Expression::from_str(line, 1, 1).eval())
        .sum()
}

fn part_two(data: &Vec<String>) -> u64 {
    data.iter()
        .map(|line| Expression::from_str(line, 2, 1).eval())
        .sum()
}
