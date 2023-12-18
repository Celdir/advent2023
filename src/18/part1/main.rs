#[allow(unused_imports)]
use std::io::{stdin, BufRead};
use std::ops::Add;

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Could not parse direction {}", value),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
struct Position(isize, isize);

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
struct Vector {
    len: isize,
    direction: Direction,
}

impl Add<Vector> for Position {
    type Output = Position;

    fn add(self, v: Vector) -> Position {
        let (i, j) = (self.0, self.1);
        match v.direction {
            Direction::Up => Position(i - v.len, j),
            Direction::Down => Position(i + v.len, j),
            Direction::Left => Position(i, j - v.len),
            Direction::Right => Position(i, j + v.len),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Edge {
    origin: Position,
    vector: Vector,
    color: String,
}

impl From<(&str, Position)> for Edge {
    fn from(value: (&str, Position)) -> Self {
        let (s, pos) = value;
        let parts: Vec<_> = s.split(" ").collect();
        assert_eq!(parts.len(), 3);
        Edge {
            origin: pos,
            vector: Vector {
                len: parts[1].parse::<isize>().unwrap(),
                direction: Direction::from(parts[0]),
            },
            color: parts[2].replace("(", ""),
        }
    }
}

fn area(edges: &[Edge]) -> isize {
    let n = edges.len();
    (0..n)
        .map(|i| {
            let j = (i + 1) % n;
            edges[i].origin.0 * edges[j].origin.1 - edges[i].origin.1 * edges[j].origin.0
        })
        .sum::<isize>().abs()
        / 2
}

fn main() {
    let edges: Vec<Edge> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .scan(Position(0, 0), |state, s| {
            let edge = Edge::from((&s[..], *state));
            *state = edge.origin + edge.vector;
            Some(edge)
        })
        .collect();

    let area = area(&edges);
    let boundary: isize = edges.iter().map(|e| e.vector.len).sum();
    let interior = area - boundary / 2 + 1;
    let ans = boundary + interior;
    println!("{}", ans);
}
