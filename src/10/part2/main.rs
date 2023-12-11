#[allow(unused_imports)]
use std::collections::HashMap;
use std::{
    collections::HashSet,
    io::{stdin, BufRead},
};

use cgraph::{
    graph::{builder::GraphBuilder, traits::Graph},
    iter::bfs::bfs,
};

fn main() {
    let grid: Vec<Vec<u8>> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().into_bytes())
        .collect();
    let mut builder = GraphBuilder::<(), usize>::new()
        .adj_list()
        .undirected()
        .ordinal()
        .with_size(grid.len() * grid[0].len());
    let mut start = None;
    for ii in 0..grid.len() {
        for jj in 0..grid[ii].len() {
            let i = ii as isize;
            let j = jj as isize;
            if grid[ii][jj] as char == 'S' {
                start = Some(ii * grid[ii].len() + jj);
            }
            let moves = match grid[ii][jj] as char {
                '|' => vec![(i - 1, j, &['|', '7', 'F']), (i + 1, j, &['|', 'L', 'J'])],
                '-' => vec![(i, j - 1, &['-', 'L', 'F']), (i, j + 1, &['-', '7', 'J'])],
                'L' => vec![(i - 1, j, &['|', '7', 'F']), (i, j + 1, &['-', '7', 'J'])],
                'J' => vec![(i - 1, j, &['|', '7', 'F']), (i, j - 1, &['-', 'L', 'F'])],
                '7' => vec![(i + 1, j, &['|', 'L', 'J']), (i, j - 1, &['-', 'L', 'F'])],
                'F' => vec![(i + 1, j, &['|', 'L', 'J']), (i, j + 1, &['-', '7', 'J'])],
                'S' => vec![
                    (i - 1, j, &['|', '7', 'F']),
                    (i + 1, j, &['|', 'L', 'J']),
                    (i, j - 1, &['-', 'L', 'F']),
                    (i, j + 1, &['-', '7', 'J']),
                ],
                _ => vec![],
            };
            let cur = (ii * grid[ii].len() + jj) as usize;
            for (ni, nj, allowed) in moves {
                if 0 <= ni
                    && ni < grid.len() as isize
                    && 0 <= nj
                    && nj < grid[ii].len() as isize
                    && allowed.contains(&(grid[ni as usize][nj as usize] as char))
                {
                    let adj = (ni * grid[ii].len() as isize + nj) as usize;
                    if cur < adj {
                        builder = builder.edge(cur, adj, 1);
                    }
                }
            }
        }
    }
    let graph = builder.build();
    let cycle: HashSet<usize> = bfs(&graph, start.unwrap())
        .map(|(_, node)| node.id())
        .filter(|&nid| graph.degree(nid) == 2)
        .collect();

    let mut inside = 0;

    let norths = HashSet::from(['|', 'L', 'J']);

    for ii in 0..grid.len() {
        let mut north_cycle = 0;
        for jj in 0..grid[ii].len() {
            let cur = ii * grid[ii].len() + jj;
            if cycle.contains(&cur) {
                if norths.contains(&(grid[ii][jj] as char)) {
                    north_cycle += 1;
                }
            } else if north_cycle % 2 == 1 {
                inside += 1;
            }
        }
    }

    println!("{}", inside);
}
