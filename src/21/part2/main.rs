#[allow(unused_imports)]
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{stdin, BufRead};

use std::ops::Add;

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
struct Position(isize, isize);

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, d: Direction) -> Position {
        let (i, j) = (self.0, self.1);
        match d {
            Direction::Up => Position(i - 1, j),
            Direction::Down => Position(i + 1, j),
            Direction::Left => Position(i, j - 1),
            Direction::Right => Position(i, j + 1),
        }
    }
}

fn main() {
    let grid: Vec<Vec<u8>> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().into_bytes())
        .collect();
    let n = grid.len() as isize;
    let m = grid[0].len() as isize;

    let mut positions: HashSet<Position> = HashSet::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == b'S' {
                positions.insert(Position(i as isize, j as isize));
            }
        }
    }

    for step in 1..1001 {
        positions = positions
            .into_iter()
            .map(|pos| {
                let mut adj = Vec::new();
                for d in [
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                ] {
                    let next = pos + d;
                    let (i, j) = (next.0.rem_euclid(n) as usize, next.1.rem_euclid(m) as usize);
                    if grid[i][j] != b'#' {
                        adj.push(next);
                    }
                }
                adj
            })
            .flatten()
            .filter(|pos| pos.0 >= -n && pos.0 < 2 * n && pos.1 >= -m && pos.1 < 2 * m)
            .collect();
        let center = positions
            .iter()
            .filter(|pos| pos.0 >= 0 && pos.0 < n && pos.1 >= 0 && pos.1 < m);
        let left = positions
            .iter()
            .filter(|pos| pos.0 >= 0 && pos.0 < n && pos.1 >= -m && pos.1 < 0);
        let right = positions
            .iter()
            .filter(|pos| pos.0 >= 0 && pos.0 < n && pos.1 >= m && pos.1 < 2 * m);
        let top = positions
            .iter()
            .filter(|pos| pos.0 >= -n && pos.0 < 0 && pos.1 >= 0 && pos.1 < m);
        let bottom = positions
            .iter()
            .filter(|pos| pos.0 >= n && pos.0 < 2*n && pos.1 >= 0 && pos.1 < m);
        let upper_left = positions
            .iter()
            .filter(|pos| pos.0 >= -n && pos.0 < 0 && pos.1 >= -m && pos.1 < 0);
        let upper_right = positions
            .iter()
            .filter(|pos| pos.0 >= -n && pos.0 < 0 && pos.1 >= m && pos.1 < 2*m);
        let lower_left = positions
            .iter()
            .filter(|pos| pos.0 >= n && pos.0 < 2*n && pos.1 >= -m && pos.1 < 0);
        let lower_right = positions
            .iter()
            .filter(|pos| pos.0 >= n && pos.0 < 2*n && pos.1 >= m && pos.1 < 2*m);
        println!(
            "{} total: {}, center: {}, left: {}, right: {}, top: {}, bottom: {}, upper_left: {}, upper_right: {}, lower_left: {}, lower_right: {}",
            step,
            positions.len(),
            center.count(),
            left.count(),
            right.count(),
            top.count(),
            bottom.count(),
            upper_left.count(),
            upper_right.count(),
            lower_left.count(),
            lower_right.count()
        );
    }
}
