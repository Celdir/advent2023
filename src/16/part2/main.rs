use std::cmp::max;
#[allow(unused_imports)]
use std::collections::HashMap;
use std::io::{stdin, BufRead};

use cgraph::{
    graph::{builder::GraphBuilder, traits::Graph},
    iter::bfs::bfs,
};
use itertools::Itertools;

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn idx(self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 3,
        }
    }
}

fn energize<G, F>(graph: &G, start: usize, reverse_index: F) -> usize
where
    G: Graph<NId = usize>,
    F: Fn(usize) -> (usize, usize),
{
    let visited = bfs(graph, start)
        .map(|(_, node)| reverse_index(node.id()))
        .unique();
    visited.count()
}

fn main() {
    let grid: Vec<Vec<u8>> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().into_bytes())
        .collect();
    let mut builder = GraphBuilder::<(), ()>::new()
        .adj_flat()
        .directed()
        .ordinal()
        .with_size(grid.len() * grid[0].len() * 4);

    let index = |i: usize, j: usize, d: &Direction| {
        d.idx() * grid.len() * grid[i].len() + i * grid[i].len() + j
    };
    let reverse_index = |mut id: usize| {
        id = id % (grid.len() * grid[0].len());
        (id / grid[0].len(), id % grid[0].len())
    };

    for ii in 0..grid.len() {
        for jj in 0..grid[ii].len() {
            let i = ii as isize;
            let j = jj as isize;
            let moves = match grid[ii][jj] {
                b'|' => HashMap::from([
                    (Direction::Up, vec![(Direction::Up, i - 1, j)]),
                    (Direction::Down, vec![(Direction::Down, i + 1, j)]),
                    (
                        Direction::Left,
                        vec![(Direction::Up, i - 1, j), (Direction::Down, i + 1, j)],
                    ),
                    (
                        Direction::Right,
                        vec![(Direction::Up, i - 1, j), (Direction::Down, i + 1, j)],
                    ),
                ]),
                b'-' => HashMap::from([
                    (
                        Direction::Up,
                        vec![(Direction::Left, i, j - 1), (Direction::Right, i, j + 1)],
                    ),
                    (
                        Direction::Down,
                        vec![(Direction::Left, i, j - 1), (Direction::Right, i, j + 1)],
                    ),
                    (Direction::Left, vec![(Direction::Left, i, j - 1)]),
                    (Direction::Right, vec![(Direction::Right, i, j + 1)]),
                ]),
                b'/' => HashMap::from([
                    (Direction::Up, vec![(Direction::Right, i, j + 1)]),
                    (Direction::Down, vec![(Direction::Left, i, j - 1)]),
                    (Direction::Left, vec![(Direction::Down, i + 1, j)]),
                    (Direction::Right, vec![(Direction::Up, i - 1, j)]),
                ]),
                b'\\' => HashMap::from([
                    (Direction::Up, vec![(Direction::Left, i, j - 1)]),
                    (Direction::Down, vec![(Direction::Right, i, j + 1)]),
                    (Direction::Left, vec![(Direction::Up, i - 1, j)]),
                    (Direction::Right, vec![(Direction::Down, i + 1, j)]),
                ]),
                b'.' => HashMap::from([
                    (Direction::Up, vec![(Direction::Up, i - 1, j)]),
                    (Direction::Down, vec![(Direction::Down, i + 1, j)]),
                    (Direction::Left, vec![(Direction::Left, i, j - 1)]),
                    (Direction::Right, vec![(Direction::Right, i, j + 1)]),
                ]),
                _ => HashMap::new(),
            };
            for dir in vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                let cur = index(ii, jj, &dir);
                for &(next_dir, ni, nj) in &moves[&dir] {
                    if 0 <= ni
                        && ni < grid.len() as isize
                        && 0 <= nj
                        && nj < grid[ii].len() as isize
                    {
                        let next = index(ni as usize, nj as usize, &next_dir);
                        builder = builder.edge(cur, next, ());
                    }
                }
            }
        }
    }

    let graph = builder.build();
    let mut ans = 0;

    for i in 0..grid.len() {
        for j in [0, grid[i].len() - 1] {
            let direction = match j {
                0 => Direction::Right,
                _ => Direction::Left,
            };
            let start = index(i, j, &direction);
            ans = max(ans, energize(&graph, start, reverse_index));
        }
    }
    for j in 0..grid[0].len() {
        for i in [0, grid.len() - 1] {
            let direction = match i {
                0 => Direction::Down,
                _ => Direction::Up,
            };
            let start = index(i, j, &direction);
            ans = max(ans, energize(&graph, start, reverse_index));
        }
    }
    println!("{}", ans);
}
