#[allow(unused_imports)]
use std::collections::HashMap;
use std::io::{stdin, BufRead};

use cgraph::{algo::shortest_paths::dijkstra::dijkstra, graph::builder::GraphBuilder};
use std::ops::Add;

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
struct Position(usize, usize);

impl Add<Direction> for Position {
    type Output = Option<Position>;

    fn add(self, d: Direction) -> Option<Position> {
        let (i, j) = (self.0, self.1);
        match d {
            Direction::Up => match i {
                0 => None,
                _ => Some(Position(i - 1, j)),
            },
            Direction::Down => Some(Position(i + 1, j)),
            Direction::Left => match j {
                0 => None,
                _ => Some(Position(i, j - 1)),
            },
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

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
struct State {
    pos: Position,
    dir: Direction,
    straight: usize,
}

impl Add<Direction> for State {
    type Output = Option<State>;

    fn add(self, d: Direction) -> Option<State> {
        let straight = if d == self.dir { self.straight + 1 } else { 1 };
        if straight > 3 || d == self.dir.opposite() {
            return None;
        }
        Some(State {
            pos: (self.pos + d)?,
            dir: d,
            straight,
        })
    }
}

impl State {
    fn is_valid(&self, n: usize, m: usize) -> bool {
        self.pos.is_valid(n, m)
    }
}

fn main() {
    let grid: Vec<Vec<u32>> = stdin()
        .lock()
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();
    let n = grid.len();
    let m = grid[0].len();
    let mut builder = GraphBuilder::<(), u32>::new()
        .adj_list()
        .directed()
        .keyed::<State>();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let pos = Position(i, j);
            for dir in [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                for straight in 0..4 {
                    let state = State { pos, dir, straight };
                    builder = builder.node(state, ());
                }
            }
        }
    }

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let pos = Position(i, j);
            for cur_dir in [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                for straight in 0..4 {
                    let cur = State {
                        pos,
                        dir: cur_dir,
                        straight,
                    };
                    for adj_dir in [
                        Direction::Up,
                        Direction::Down,
                        Direction::Left,
                        Direction::Right,
                    ] {
                        if let Some(adj) = cur + adj_dir {
                            if !adj.is_valid(n, m) {
                                continue;
                            }
                            builder = builder.edge(cur, adj, grid[adj.pos.0][adj.pos.1]);
                        }
                    }
                }
            }
        }
    }

    let graph = builder.build();

    let ans = dijkstra(
        &graph,
        State {
            pos: Position(0, 0),
            dir: Direction::Right,
            straight: 0,
        },
    )
    .unwrap()
    .find(|(_, node, _)| node.id().pos == Position(n - 1, m - 1))
    .map(|(_, _, dist)| dist)
    .unwrap();
    println!("{}", ans);
}
