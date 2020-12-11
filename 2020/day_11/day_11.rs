use std::fs;
use std::thread;

fn main() {
    let data = fs::read_to_string("in").unwrap();
    let data: Vec<Vec<char>> = data.lines().map(|x| x.chars().collect()).collect();

    let data_p1 = data.clone();
    let part_one = thread::spawn(move || {
        part_one(data_p1)
    });
    
    let data_p2 = data.clone();
    let part_two = std::thread::spawn(move || {
        part_two(data_p2)    
    });

    println!("Part One: {}", part_one.join().unwrap());
    println!("Part Two: {}", part_two.join().unwrap());
}

fn solver<F>(mut data: Vec<Vec<char>>, adj_calc: F, min_occ: u32) -> u32 where
    F: Fn(&Vec<Vec<char>>, usize, usize) -> u32 {

    let h = data.len();
    let w = data[0].len();

    loop {
        let mut changes: Vec<(usize, usize)> = Vec::new();
        for i in 0..h {
            for j in 0..w {
               
                let adj_occ = adj_calc(&data, i, j);

                if data[i][j] == 'L' && adj_occ == 0 {
                    changes.push((i, j));
                } else if data[i][j] == '#' && adj_occ >= min_occ {
                    changes.push((i, j));
                }
            }
        }

        if changes.is_empty() {
            break;
        }

        for (i, j) in changes {
            if data[i][j] == '#' {
                data[i][j] = 'L';
            } else if data[i][j] == 'L' {
                data[i][j] = '#';
            }
        }
    }

    data.iter().map(|line| {
        line.iter().map(|x| {
            if *x == '#' {
                1
            } else {
                0
            }
        }).sum::<u32>()
    }).sum()
}

fn part_one(data: Vec<Vec<char>>) -> u32 {
    solver(data, |data, i, j| {
        let h = data.len();
        let w = data[0].len();

        let mut adj_occ = 0;
        for (dx, dy) in [(-1, -1), (-1, 0), (0, -1), (1, 1), (0, 1), (1, 0), (-1, 1), (1, -1)].iter() {
            let y = (i as i32) + dy;
            let x = (j as i32) + dx;
            if x >= 0 && x < w as i32 && y >= 0 && y < h as i32 {
                if data[y as usize][x as usize] == '#' {
                    adj_occ += 1;
                }
            }
        }

        adj_occ
    }, 4)
}

fn part_two(data: Vec<Vec<char>>) -> u32 {
    solver(data, |data, i, j| {
        let h = data.len();
        let w = data[0].len();
        
        let mut adj_occ = 0;
        for (dx, dy) in [(-1, -1), (-1, 0), (0, -1), (1, 1), (0, 1), (1, 0), (-1, 1), (1, -1)].iter() {
            let mut y = (i as i32) + dy;
            let mut x = (j as i32) + dx;
            while x >= 0 && x < w as i32 && y >= 0 && y < h as i32 {
                if data[y as usize][x as usize] == '#' {
                    adj_occ += 1;
                }

                if data[y as usize][x as usize] != '.' {
                    break;
                }

                y += dy;
                x += dx;
            }
        }

        adj_occ
    }, 5)
}
