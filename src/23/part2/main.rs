#[allow(unused_imports)]
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{stdin, BufRead};

use std::ops::Add;

use cgraph::graph::builder::GraphBuilder;
use cgraph::graph::traits::{DirectedGraph, Graph};
use cgraph::iter::bfs::bfs;

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

fn maxdfs<G: Graph<NId = Position, EId = usize, E = isize>>(
    g: &G,
    vis: &mut HashSet<G::NId>,
    cur: Position,
    goal: Position,
) -> isize {
    if cur == goal {
        return 0;
    }
    vis.insert(cur);
    let mut max = -1;
    for (edge, neighbor) in g.adj(cur).unwrap() {
        if !vis.contains(&neighbor.id()) {
            let branch = maxdfs(g, vis, neighbor.id(), goal);
            if branch >= 0 {
                max = std::cmp::max(max, branch + *edge.data());
            }
        }
    }
    vis.remove(&cur);
    return max;
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

    let mut start = Position(0, 0);
    let mut end = Position(n - 1, 0);

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            builder = builder.node(Position(i, j), ());
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

    let mut prime_builder = GraphBuilder::<(), isize>::new()
        .adj_flat()
        .un()
        .keyed::<Position>();
    prime_builder = prime_builder.node(start, ());
    prime_builder = prime_builder.node(end, ());
    for node in g.nodes() {
        if g.out_degree(node.id()) > 2 {
            prime_builder = prime_builder.node(node.id(), ());
        }
    }
    let mut parent: HashMap<Position, (Position, isize)> = HashMap::new();
    parent.insert(start, (start, 0));
    parent.insert(end, (end, 0));
    for (edge_opt, node) in bfs(&g, start) {
        if let Some(edge) = edge_opt {
            if g.out_degree(node.id()) > 2 || node.id() == end {
                parent.insert(node.id(), (node.id(), 0));
            } else {
                let other = edge.other(node.id());
                let (last_junction, dist) = parent[&other];
                parent.insert(node.id(), (last_junction, dist + 1));
            }
        }
        for (_, neighbor) in g.adj(node.id()).unwrap() {
            if parent.contains_key(&neighbor.id()) {
                let (parent_id, dist_parent) = parent[&node.id()];
                let (neighbor_parent, dist_neighbor_parent) = parent[&neighbor.id()];
                if parent_id != neighbor_parent {
                    prime_builder = prime_builder.edge(
                        parent_id,
                        neighbor_parent,
                        dist_parent + dist_neighbor_parent + 1,
                    );
                }
            }
        }
    }

    let gprime = prime_builder.build();

    let mut vis = HashSet::new();
    let ans = maxdfs(&gprime, &mut vis, start, end);
    println!("{}", ans);
}
