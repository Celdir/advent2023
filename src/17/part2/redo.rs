#[allow(unused_imports)]
use std::collections::HashMap;
use std::io::{stdin, BufRead};

use cgraph::{
    algo::shortest_paths::dijkstra::dijkstra,
    graph::grid::{Direction, Grid, GridBounds, Position},
};
use std::ops::Add;

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
struct State {
    pos: Position,
    dir: Direction,
    straight: usize,
}

impl Add<Direction> for State {
    type Output = State;

    fn add(self, d: Direction) -> State {
        let straight = if d == self.dir { self.straight + 1 } else { 1 };
        State {
            pos: self.pos + d,
            dir: d,
            straight,
        }
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
    let n = grid.len() as isize;
    let m = grid[0].len() as isize;

    let bounds = GridBounds::new(0..n, 0..m);
    let graph = Grid::four_connected(
        |_: State| (),
        |_, v: State| {
            let (i, j) = v.pos.into();
            grid[i as usize][j as usize]
        },
        |u| u.straight <= 10 && bounds.check(u.pos),
        |u, v| v.dir != u.dir.opposite() && (u.straight >= 4 || u.straight == 0 || u.dir == v.dir),
    );

    let ans = dijkstra(
        &graph,
        State {
            pos: (0, 0).into(),
            dir: Direction::Right,
            straight: 0,
        },
    )
    .unwrap()
    .find(|(_, node, _)| node.id().pos == (n - 1, m - 1).into())
    .map(|(_, _, dist)| dist)
    .unwrap();
    println!("{}", ans);
}
