#[allow(unused_imports)]
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{stdin, BufRead};
use std::ops::Sub;

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug, PartialOrd, Ord)]
struct Position {
    z: usize,
    x: usize,
    y: usize,
}

impl From<&str> for Position {
    fn from(value: &str) -> Self {
        let parts: Vec<_> = value.split(",").collect();
        assert_eq!(parts.len(), 3);
        Position {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
            z: parts[2].parse().unwrap(),
        }
    }
}

impl Sub<usize> for Position {
    type Output = Position;
    fn sub(self, rhs: usize) -> Self::Output {
        Position {
            x: self.x,
            y: self.y,
            z: self.z - rhs,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug, PartialOrd, Ord)]
struct Brick {
    start: Position,
    end: Position,
}

impl From<String> for Brick {
    fn from(value: String) -> Self {
        let (s, e) = value.split_once("~").unwrap();
        Brick {
            start: Position::from(s),
            end: Position::from(e),
        }
    }
}

impl Sub<usize> for Brick {
    type Output = Brick;
    fn sub(self, rhs: usize) -> Self::Output {
        Brick {
            start: self.start - rhs,
            end: self.end - rhs,
        }
    }
}

impl Brick {
    fn span(&self) -> Vec<Position> {
        let mut span = Vec::new();
        for z in self.start.z..=self.end.z {
            for x in self.start.x..=self.end.x {
                for y in self.start.y..=self.end.y {
                    span.push(Position { z, x, y });
                }
            }
        }
        span
    }
}

fn main() {
    let mut bricks: Vec<Brick> = stdin()
        .lock()
        .lines()
        .map(|l| Brick::from(l.unwrap()))
        .collect();
    bricks.sort();
    let mut used: HashMap<Position, usize> = HashMap::new();
    for (i, brick) in bricks.iter_mut().enumerate() {
        loop {
            if brick.start.z <= 1 {
                break;
            }
            let down = *brick - 1;
            let span = down.span();
            if span.into_iter().any(|s| used.contains_key(&s)) {
                break;
            }
            *brick = down;
        }
        for s in brick.span() {
            used.insert(s, i);
        }
    }

    let mut in_edges: HashMap<Brick, HashSet<Brick>> = HashMap::new();
    let mut out_edges: HashMap<Brick, HashSet<Brick>> = HashMap::new();
    for (i, &brick) in bricks.iter().enumerate() {
        let down = brick - 1;
        for s in down.span() {
            if used.contains_key(&s) && used[&s] != i {
                let below = bricks[used[&s]];
                in_edges.entry(below).or_default().insert(brick);
                out_edges.entry(brick).or_default().insert(below);
            }
        }
    }
    let mut ans = 0;
    for brick in &bricks {
        let mut can_remove = true;
        if !in_edges.contains_key(brick) {
            ans += 1;
            continue;
        }
        for dep in &in_edges[brick] {
            if out_edges[dep].len() == 1 {
                can_remove = false;
            }
        }
        if can_remove {
            ans += 1;
        }
    }
    println!("{}", ans);
}
