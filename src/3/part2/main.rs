#[allow(unused_imports)]
use std::collections::HashMap;
use std::{
    collections::VecDeque,
    io::{stdin, BufRead},
};

fn get_adj(grid: &Vec<String>, i: usize, j: usize) -> Vec<(usize, usize, char)> {
    let deltas: [(i32, i32); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (-1, 1),
        (-1, -1),
        (1, 1),
        (1, -1),
    ];
    let mut result = Vec::new();

    let line = grid[i].as_bytes();
    for (di, dj) in deltas {
        let ni: i32 = (i as i32) + di;
        let nj: i32 = (j as i32) + dj;
        if ni < (grid.len() as i32) && ni >= 0 && nj >= 0 && nj < (line.len() as i32) {
            let adj = grid[ni as usize].as_bytes()[nj as usize] as char;
            result.push((ni as usize, nj as usize, adj));
        }
    }
    result
}

fn main() {
    let grid: Vec<String> = stdin().lock().lines().map(|l| l.unwrap()).collect();
    let mut marked = HashMap::new();
    let mut bfs = VecDeque::new();
    let mut gear_num: usize = 0;
    for i in 0..grid.len() {
        let line = grid[i].as_bytes();
        for j in 0..line.len() {
            let cur = line[j] as char;
            if cur != '*' {
                continue;
            }
            for (ni, nj, adj) in get_adj(&grid, i, j) {
                if adj.is_ascii_digit() {
                    marked.insert((ni, nj), gear_num);
                    bfs.push_back((ni, nj));
                }
            }
            gear_num += 1;
        }
    }

    while !bfs.is_empty() {
        let (i, j) = bfs.pop_front().unwrap();
        for (ni, nj, adj) in get_adj(&grid, i, j) {
            if adj.is_ascii_digit() && !marked.contains_key(&(ni, nj)) {
                marked.insert((ni, nj), marked[&(i, j)]);
                bfs.push_back((ni, nj));
            }
        }
    }

    let mut gear_vals: HashMap<usize, Vec<usize>> = HashMap::new();

    for index in 0..grid.len() {
        let line = &grid[index];

        let digits: Vec<_> = line.match_indices(|c: char| c.is_ascii_digit()).collect();
        let mut last: Option<usize> = None;
        let mut str = String::new();
        let mut gear_num = None;
        for (i, c) in digits {
            if !marked.contains_key(&(index, i)) {
                continue;
            }

            if last.is_some() && i != last.unwrap() + 1 {
                let number: usize = str.parse().unwrap();
                gear_vals.entry(gear_num.unwrap()).or_default().push(number);
                str = String::new();
            }
            str.push(c.as_bytes()[0] as char);
            last = Some(i);
            gear_num = Some(marked[&(index, i)]);
        }
        if !str.is_empty() {
            let number: usize = str.parse().unwrap();
            gear_vals.entry(gear_num.unwrap()).or_default().push(number);
        }
    }

    let mut ans = 0;
    for (_, vals) in gear_vals {
        if vals.len() == 2 {
            ans += vals[0] * vals[1];
        }
    }
    println!("{}", ans);
}
