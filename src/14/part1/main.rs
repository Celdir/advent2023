#[allow(unused_imports)]
use std::collections::HashMap;
use std::io::{stdin, BufRead};

fn load(grid: &Vec<Vec<u8>>) -> isize {
    let mut sum = 0;
    for j in 0..grid[0].len() {
        let mut last_rock: isize = -1;
        for i in 0..grid.len() {
            match grid[i][j] {
                b'#' => last_rock = i as isize,
                b'O' => {
                    last_rock += 1;
                    sum += grid.len() as isize - last_rock;
                }
                _ => (),
            }
        }
    }
    sum
}

fn main() {
    let grid: Vec<Vec<u8>> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().into_bytes())
        .collect();

    println!("{}", load(&grid));
}
