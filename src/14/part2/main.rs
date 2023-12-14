#[allow(unused_imports)]
use std::collections::HashMap;
use std::io::{stdin, BufRead};

fn load(grid: &Vec<Vec<u8>>) -> usize {
    let mut sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == b'O' {
                sum += grid.len() - i;
            }
        }
    }
    sum
}

fn north(grid: &mut Vec<Vec<u8>>) {
    for j in 0..grid[0].len() {
        let mut last_rock: isize = -1;
        for i in 0..grid.len() {
            match grid[i][j] {
                b'#' => last_rock = i as isize,
                b'O' => {
                    last_rock += 1;
                    grid[i][j] = b'.';
                    grid[last_rock as usize][j] = b'O';
                }
                _ => (),
            }
        }
    }
}

fn west(grid: &mut Vec<Vec<u8>>) {
    for i in 0..grid.len() {
        let mut last_rock: isize = -1;
        for j in 0..grid[i].len() {
            match grid[i][j] {
                b'#' => last_rock = j as isize,
                b'O' => {
                    last_rock += 1;
                    grid[i][j] = b'.';
                    grid[i][last_rock as usize] = b'O';
                }
                _ => (),
            }
        }
    }
}

fn south(grid: &mut Vec<Vec<u8>>) {
    for j in 0..grid[0].len() {
        let mut last_rock: isize = grid.len() as isize;
        for i in (0..grid.len()).rev() {
            match grid[i][j] {
                b'#' => last_rock = i as isize,
                b'O' => {
                    last_rock -= 1;
                    grid[i][j] = b'.';
                    grid[last_rock as usize][j] = b'O';
                }
                _ => (),
            }
        }
    }
}

fn east(grid: &mut Vec<Vec<u8>>) {
    for i in 0..grid.len() {
        let mut last_rock: isize = grid[i].len() as isize;
        for j in (0..grid[i].len()).rev() {
            match grid[i][j] {
                b'#' => last_rock = j as isize,
                b'O' => {
                    last_rock -= 1;
                    grid[i][j] = b'.';
                    grid[i][last_rock as usize] = b'O';
                }
                _ => (),
            }
        }
    }
}

fn cycle(mut grid: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    north(&mut grid);
    west(&mut grid);
    south(&mut grid);
    east(&mut grid);
    grid
}

fn to_string(grid: &Vec<Vec<u8>>) -> String {
    grid.iter()
        .map(|line| String::from_utf8(line.to_vec()).unwrap())
        .collect::<Vec<String>>()
        .join("\n")
}

fn main() {
    let mut grid: Vec<Vec<u8>> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().into_bytes())
        .collect();

    let mut step = HashMap::new();

    for i in 0..1000000000 {
        grid = cycle(grid);
        let str = to_string(&grid);
        if step.contains_key(&str) {
            let cycle_len = i - step[&str];
            let rem = (1000000000-(step[&str]+1)) % cycle_len;
            for _ in 0..rem {
                grid = cycle(grid);
            }
            break;
        } else {
            step.insert(str, i);
        }
    }

    println!("{}", load(&grid));
}
