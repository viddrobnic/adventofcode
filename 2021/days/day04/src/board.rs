#[derive(Clone, Debug)]
pub struct Board {
    numbers: Vec<Vec<i32>>,
    marked: Vec<Vec<bool>>,
}

impl Board {
    pub fn new(data: &Vec<&str>) -> Self {
        let mut numbers: Vec<Vec<i32>> = Vec::with_capacity(5);
        for row in data {
            let row: Vec<i32> = row
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            numbers.push(row);
        }

        Board {
            numbers: numbers,
            marked: vec![vec![false; 5]; 5],
        }
    }

    fn find(&self, number: i32) -> Option<(usize, usize)> {
        let mut found = false;
        let (mut y, mut x) = (0, 0);

        for (i, row) in self.numbers.iter().enumerate() {
            for (j, num) in row.iter().enumerate() {
                if *num == number {
                    found = true;
                    y = i;
                    x = j;
                }
            }
        }

        if found {
            Some((y, x))
        } else {
            None
        }
    }

    // Returns if board is full after the number is marked.
    pub fn mark(&mut self, number: i32) -> bool {
        let full = self.full();

        if let Some((y, x)) = self.find(number) {
            self.marked[y][x] = true;
        }

        if !full && self.full() {
            true
        } else {
            false
        }
    }

    // Returns weather the board has at least one row or one column full.
    pub fn full(&self) -> bool {
        for row in &self.marked {
            let full = row.iter().fold(true, |acc, val| acc && *val);
            if full {
                return true;
            }
        }

        for i in 0..5 {
            let full = self.marked.iter().fold(true, |acc, val| acc && val[i]);
            if full {
                return true;
            }
        }

        false
    }

    pub fn sum_unmarked(&self) -> i32 {
        let mut sum = 0;

        for i in 0..5 {
            for j in 0..5 {
                if !self.marked[i][j] {
                    sum += self.numbers[i][j];
                }
            }
        }

        sum
    }
}
