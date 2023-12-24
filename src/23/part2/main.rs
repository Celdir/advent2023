#[allow(unused_imports)]
use std::collections::HashMap;
use std::io::{stdin, BufRead};

use std::ops::Add;

use cgraph::graph::builder::GraphBuilder;
use cgraph::graph::traits::Graph;

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
struct Position(usize, usize);

impl Add<Direction> for Position {
    type Output = Option<Position>;

    fn add(self, d: Direction) -> Option<Position> {
        let (i, j) = (self.0, self.1);
        match d {
            Direction::Up => Some(Position(i.checked_sub(1)?, j)),
            Direction::Down => Some(Position(i + 1, j)),
            Direction::Left => Some(Position(i, j.checked_sub(1)?)),
            Direction::Right => Some(Position(i, j + 1)),
        }
    }
}

impl Position {
    fn is_valid(&self, n: usize, m: usize) -> bool {
        let (i, j) = (self.0, self.1);
        i < n && j < m
    }
}

fn maxdfs<G: Graph<NId = Position, EId = usize>>(
    g: &G,
    memo: &mut HashMap<G::EId, usize>,
    cur: G::EId,
    goal: Position,
) -> usize {
    if memo.contains_key(&cur) {
        return memo[&cur];
    }
    let edge = g.edge(cur).unwrap();
    if edge.v() == goal {
        memo.insert(cur, 1);
        return 1;
    }
    memo.insert(cur, 0);
    for (e, neighbor) in g.adj(edge.v()).unwrap() {
        if neighbor.id() != edge.u() {
            let branch = 1 + maxdfs(g, memo, e.id(), goal);
            let entry = memo.get_mut(&cur).unwrap();
            *entry = std::cmp::max(*entry, branch);
        }
    }
    return memo[&cur];
}

fn main() {
    let grid: Vec<Vec<u8>> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().into_bytes())
        .collect();
    let n = grid.len();
    let m = grid[0].len();

    let mut builder = GraphBuilder::<(), isize>::new()
        .adj_flat()
        .di()
        .keyed::<Position>();
    let mut prime_builder = GraphBuilder::<(), isize>::new()
        .adj_flat()
        .di()
        .keyed::<Position>();

    let mut start = Position(0, 0);
    let mut end = Position(n - 1, 0);

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            builder = builder.node(Position(i, j), ());
            prime_builder = prime_builder.node(Position(i, j), ());
        }
    }
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == b'.' && (i == 0 || i == n - 1) {
                match i {
                    0 => start.1 = j,
                    _ => end.1 = j,
                };
            }
            let directions = match grid[i][j] {
                b'#' => vec![],
                _ => vec![
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                ],
            };
            let cur = Position(i, j);
            for d in directions {
                let neighbor_opt = cur + d;
                if let Some(neighbor) = neighbor_opt {
                    if neighbor.is_valid(n, m) && grid[neighbor.0][neighbor.1] != b'#' {
                        builder = builder.edge(cur, neighbor, 1);
                    }
                }
            }
        }
    }

    let g = builder.build();

    let mut memo = HashMap::new();
    let ans = maxdfs(
        &g,
        &mut memo,
        g.adj(start).unwrap().next().unwrap().0.id(),
        end,
    );
    println!("{}", ans);
}
