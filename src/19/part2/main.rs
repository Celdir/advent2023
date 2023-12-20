use std::cmp::{max, min};
use std::fmt::Debug;
use std::io::stdin;
use std::ops::Range;
use std::{collections::HashMap, io::Read};

#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy)]
enum Xmas {
    X,
    M,
    A,
    S,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum Op {
    Less,
    Greater,
}

impl From<&str> for Op {
    fn from(value: &str) -> Self {
        match value {
            "<" => Op::Less,
            ">" => Op::Greater,
            _ => panic!("No Xmas :("),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Rule {
    var: Xmas,
    op: Op,
    val: usize,
    next: String,
}

impl Rule {
    fn eval(&self, rating: &Rating) -> (Xmas, DisjointRange, DisjointRange) {
        let var = match self.var {
            Xmas::X => &rating.x,
            Xmas::M => &rating.m,
            Xmas::A => &rating.a,
            Xmas::S => &rating.s,
        };
        let (allowed_range, inverse) = match self.op {
            Op::Less => (1..self.val, self.val..4001),
            Op::Greater => ((self.val + 1)..4001, 1..(self.val + 1)),
        };
        (
            self.var,
            var.intersection(&allowed_range.into()),
            var.intersection(&inverse.into()),
        )
    }
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let (head, next) = value.split_once(":").unwrap();
        let var = match &head[0..1] {
            "x" => Xmas::X,
            "m" => Xmas::M,
            "a" => Xmas::A,
            "s" => Xmas::S,
            _ => panic!("No Xmas :("),
        };
        let op = Op::from(&head[1..2]);
        let val = (&head[2..]).parse().unwrap();
        Rule {
            var,
            op,
            val,
            next: next.to_string(),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    terminal: String,
}

impl Workflow {
    fn eval(&self) -> Vec<(Rating, String)> {
        let mut r = Rating::full();
        let mut ans = Vec::new();
        for rule in &self.rules {
            let (xmas, yes, no) = rule.eval(&r);
            ans.push((r.set(xmas, yes), rule.next.clone()));
            r = r.set(xmas, no);
        }
        ans.push((r, self.terminal.clone()));
        ans
    }
}

impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        let (name, mut tail) = value.split_once("{").unwrap();
        tail = tail.strip_suffix("}").unwrap();
        let mut split: Vec<_> = tail.split(",").collect();
        let terminal = split.pop().unwrap();
        let rules = split.iter().map(|&s| Rule::from(s)).collect();
        Workflow {
            name: name.to_string(),
            rules,
            terminal: terminal.to_string(),
        }
    }
}

trait Intersection {
    fn intersection(&self, other: &Self) -> Self;
}

impl<T: Ord + PartialOrd + Copy + Debug> Intersection for Range<T> {
    fn intersection(&self, other: &Self) -> Self {
        let start = max(self.start, other.start);
        let end = min(self.end, other.end);
        if start >= end {
            return Range { start, end: start };
        }
        Range { start, end }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct DisjointRange(Vec<Range<usize>>);

impl Intersection for DisjointRange {
    fn intersection(&self, other: &Self) -> DisjointRange {
        let mut result = Vec::new();
        for r1 in &self.0 {
            for r2 in &other.0 {
                let inter = r1.intersection(r2);
                if !inter.is_empty() {
                    result.push(inter);
                }
            }
        }
        DisjointRange(result).dedup()
    }
}

impl DisjointRange {
    fn len(&self) -> usize {
        self.0.iter().map(|range| range.len()).sum()
    }

    fn dedup(mut self) -> Self {
        self.0
            .sort_by(|a, b| (a.start, a.end).cmp(&(b.start, b.end)));
        DisjointRange(
            self.0
                .into_iter()
                .fold(Vec::new(), |mut acc: Vec<Range<usize>>, range| {
                    match acc.last() {
                        None => vec![range],
                        Some(prev) => {
                            if !prev.intersection(&range).is_empty() || prev.end == range.start {
                                let rep = prev.start..range.end;
                                *acc.last_mut().unwrap() = rep;
                                acc
                            } else {
                                acc.push(range);
                                acc
                            }
                        }
                    }
                }),
        )
    }
}

impl From<Range<usize>> for DisjointRange {
    fn from(value: Range<usize>) -> Self {
        DisjointRange(vec![value])
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Rating {
    x: DisjointRange,
    m: DisjointRange,
    a: DisjointRange,
    s: DisjointRange,
}

impl Rating {
    fn count(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }

    fn set(&self, xmas: Xmas, val: DisjointRange) -> Self {
        let mut r = self.clone();
        match xmas {
            Xmas::X => r.x = val,
            Xmas::M => r.m = val,
            Xmas::A => r.a = val,
            Xmas::S => r.s = val,
        }
        r
    }

    fn full() -> Self {
        Rating {
            x: DisjointRange(vec![(1..4001)]),
            m: DisjointRange(vec![(1..4001)]),
            a: DisjointRange(vec![(1..4001)]),
            s: DisjointRange(vec![(1..4001)]),
        }
    }

    fn empty() -> Self {
        Rating {
            x: DisjointRange(vec![]),
            m: DisjointRange(vec![]),
            a: DisjointRange(vec![]),
            s: DisjointRange(vec![]),
        }
    }
}

impl Intersection for Rating {
    fn intersection(&self, other: &Self) -> Self {
        Rating {
            x: self.x.intersection(&other.x),
            m: self.m.intersection(&other.m),
            a: self.a.intersection(&other.a),
            s: self.s.intersection(&other.s),
        }
    }
}

fn accepted_ratings(workflows: &HashMap<String, Workflow>, workflow: &Workflow) -> Vec<Rating> {
    let transitions = workflow.eval();
    transitions
        .into_iter()
        .map(|(rating, name)| match name.as_str() {
            "A" => vec![rating],
            "R" => vec![Rating::empty()],
            _ => accepted_ratings(workflows, &workflows[&name])
                .into_iter()
                .map(|r| r.intersection(&rating))
                .collect(),
        })
        .flatten()
        .collect()
}

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let (winput, _) = input.split_once("\n\n").unwrap();

    let workflows: HashMap<String, Workflow> = winput
        .lines()
        .map(|line| Workflow::from(line))
        .map(|w| (w.name.to_owned(), w))
        .collect();
    let ans: usize = accepted_ratings(&workflows, &workflows["in"])
        .into_iter()
        .map(|rating| rating.count())
        .sum();
    println!("{}", ans);
}
