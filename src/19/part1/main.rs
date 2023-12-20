#[allow(unused_imports)]
use std::io::{stdin, BufRead};
use std::{collections::HashMap, io::Read};

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
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
    fn eval(&self, rating: &Rating) -> bool {
        let var = match self.var {
            Xmas::X => rating.x,
            Xmas::M => rating.m,
            Xmas::A => rating.a,
            Xmas::S => rating.s,
        };
        match self.op {
            Op::Less => var < self.val,
            Op::Greater => var > self.val,
        }
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
    fn eval(&self, rating: &Rating) -> String {
        for rule in &self.rules {
            if rule.eval(rating) {
                return rule.next.clone();
            }
        }
        self.terminal.clone()
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

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Rating {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Rating {
    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

impl From<&str> for Rating {
    fn from(value: &str) -> Self {
        let nums: Vec<_> = (&value[1..value.len() - 1])
            .split(",")
            .map(|s| (&s[2..]).parse::<usize>().unwrap())
            .collect();
        assert_eq!(nums.len(), 4);
        Rating {
            x: nums[0],
            m: nums[1],
            a: nums[2],
            s: nums[3],
        }
    }
}

fn is_accepted(s: &str) -> bool {
    match s {
        "A" => true,
        "R" => false,
        _ => panic!("invalid terminal: {}", s),
    }
}

fn run(workflows: &HashMap<String, Workflow>, workflow: &Workflow, rating: &Rating) -> bool {
    let next = workflow.eval(rating);
    match workflows.get(&next) {
        Some(w) => run(workflows, w, rating),
        _ => is_accepted(&next),
    }
}

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let (winput, rinput) = input.split_once("\n\n").unwrap();

    let workflows: HashMap<String, Workflow> = winput
        .lines()
        .map(|line| Workflow::from(line))
        .map(|w| (w.name.to_owned(), w))
        .collect();
    let ratings: Vec<_> = rinput.lines().map(|line| Rating::from(line)).collect();

    let ans: usize = ratings
        .iter()
        .filter(|r| run(&workflows, &workflows["in"], r))
        .map(|r| r.sum())
        .sum();
    println!("{}", ans);
}
