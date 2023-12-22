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
    println!("{} {}", n, m);

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
            .collect();
        println!("{} {}", step, positions.len());
    }
    // After analysis of input and x and y value outputs, I noticed that after 65 steps the
    // boundary of the first grid is reached, then every 131 steps the end of another diamond of
    // grids is reached. Using the y values where x=65, x=65+131, x=65+2*131, etc, I did a
    // quadratic curve fit resulting in 3648 + 14604 * x + 14529 * x^2 where x is the diamond's
    // radius (excluding the origin grid) in grid widths. x_ans = (steps - 65 / 131), since steps
    // is 26501365, x_ans = 202300. Plugging x_ans into the quadratic yields the correct answer.
}
