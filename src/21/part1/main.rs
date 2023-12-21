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

fn main() {
    let grid: Vec<Vec<u8>> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().into_bytes())
        .collect();
    let n = grid.len();
    let m = grid[0].len();

    let mut positions: HashSet<Position> = HashSet::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == b'S' {
                positions.insert(Position(i, j));
            }
        }
    }

    for _ in 0..64 {
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
                    let next_opt = pos + d;
                    if let Some(next) = next_opt {
                        if next.is_valid(n, m) && grid[next.0][next.1] != b'#' {
                            adj.push(next);
                        }
                    }
                }
                adj
            })
            .flatten()
            .collect();
    }
    println!("{}", positions.len());
}
