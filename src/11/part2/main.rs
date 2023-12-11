#[allow(unused_imports)]
use std::collections::HashMap;
use std::io::{stdin, BufRead};

fn main() {
    let grid: Vec<Vec<u8>> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().into_bytes())
        .collect();

    let mut empty_rows_sum: Vec<isize> = vec![0; grid.len()];
    let mut empty_cols_sum: Vec<isize> = vec![0; grid[0].len()];

    for i in 0..grid.len() {
        empty_rows_sum[i] = match i {
            0 => 0,
            _ => empty_rows_sum[i-1],
        };
        if grid[i].iter().all(|&c| c == b'.') {
            empty_rows_sum[i] += 999999;
        }
    }
    for j in 0..grid[0].len() {
        empty_cols_sum[j] = match j {
            0 => 0,
            _ => empty_cols_sum[j-1],
        };
        if (0..grid.len()).map(|i| grid[i][j]).all(|c| c == b'.') {
            empty_cols_sum[j] += 999999;
        }
    }

    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == b'#' {
                galaxies.push((i, j));
            }
        }
    }

    let mut sum = 0;
    for a in 0..galaxies.len() {
        let (ai, aj) = galaxies[a];
        for b in a+1..galaxies.len() {
            let (bi, bj) = galaxies[b];
            let idist = (bi as isize - ai as isize).abs() + (empty_rows_sum[bi] - empty_rows_sum[ai]).abs();
            let jdist = (bj as isize - aj as isize).abs() + (empty_cols_sum[bj] - empty_cols_sum[aj]).abs();
            sum += idist + jdist;
        }
    }
    println!("{}", sum);
}
