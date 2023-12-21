use std::collections::{HashMap, VecDeque};
#[allow(unused_imports)]
use std::io::{stdin, BufRead};

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum State {
    Broadcaster,
    Flip(bool),
    Conjunction(Vec<(String, Pulse)>),
}

impl State {
    fn put_incoming(&mut self, other: &String) {
        match self {
            State::Conjunction(v) => {
                if v.iter().find(|e| e.0 == *other).is_none() {
                    v.push((other.clone(), Pulse::Low));
                }
            }
            _ => (),
        }
    }

    fn pulse(&mut self, pulse: Pulse, sender: &String) -> Option<Pulse> {
        match self {
            State::Broadcaster => Some(pulse),
            State::Flip(on) => match pulse {
                Pulse::Low => {
                    let out = if *on { Pulse::Low } else { Pulse::High };
                    *on = !*on;
                    Some(out)
                }
                Pulse::High => None,
            },
            State::Conjunction(v) => {
                let elem = v.iter_mut().find(|e| e.0 == *sender).unwrap();
                elem.1 = pulse;
                if v.iter().map(|e| e.1).all(|p| p == Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Module {
    name: String,
    dest: Vec<String>,
    state: State,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
enum Pulse {
    High,
    Low,
}

impl Module {
    fn pulse(&mut self, pulse: Pulse, sender: &String) -> Vec<(String, String, Pulse)> {
        let out: Option<Pulse> = self.state.pulse(pulse, sender);
        if out.is_none() {
            Vec::new()
        } else {
            let outpulse = out.unwrap();
            self.dest
                .iter()
                .map(|s| (self.name.clone(), s.clone(), outpulse))
                .collect()
        }
    }
}

impl From<&str> for Module {
    fn from(value: &str) -> Self {
        let (head, tail) = value.split_once(" -> ").unwrap();
        let (state, name) = match &head[0..1] {
            "%" => (State::Flip(false), head[1..].to_owned()),
            "&" => (State::Conjunction(Vec::new()), head[1..].to_owned()),
            _ => (State::Broadcaster, head.to_owned()),
        };
        let dest = tail.split(", ").map(|s| s.to_owned()).collect();
        Module { name, state, dest }
    }
}

fn init(modules: &mut HashMap<String, Module>) {
    let out_edges: Vec<(String, Vec<String>)> = modules
        .values()
        .map(|m| (m.name.clone(), m.dest.clone()))
        .collect();
    for (name, dest) in out_edges {
        for d in dest {
            let opt = modules.get_mut(&d);
            if let Some(module) = opt {
                module.state.put_incoming(&name);
            }
        }
    }
}

fn button(modules: &mut HashMap<String, Module>) -> (usize, usize) {
    let mut queue: VecDeque<(String, String, Pulse)> = VecDeque::new();
    queue.extend(
        modules
            .get_mut("broadcaster")
            .unwrap()
            .pulse(Pulse::Low, &String::new())
            .into_iter(),
    );
    let mut low_count = queue.len() + 1;
    let mut high_count = 0;
    loop {
        if queue.is_empty() {
            break;
        }
        let (sender, receiver, pulse) = queue.pop_front().unwrap();
        let opt = modules.get_mut(&receiver);
        if let Some(module) = opt {
            let outgoing = module.pulse(pulse, &sender);
            low_count += outgoing.iter().filter(|(_, _, p)| *p == Pulse::Low).count();
            high_count += outgoing
                .iter()
                .filter(|(_, _, p)| *p == Pulse::High)
                .count();
            queue.extend(outgoing);
        }
    }
    (low_count, high_count)
}

fn main() {
    let mut modules: HashMap<String, Module> = stdin()
        .lock()
        .lines()
        .map(|line| Module::from(&line.unwrap()[..]))
        .map(|m| (m.name.to_owned(), m))
        .collect();
    init(&mut modules);
    let mut low_count = 0;
    let mut high_count = 0;
    for _ in 0..1000 {
        let (l, h) = button(&mut modules);
        low_count += l;
        high_count += h;
    }
    println!("{}", low_count * high_count);
}
