#[allow(unused_imports)]
use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let mut sum = 0;
    for line in stdin().lock().lines() {
        let buf = line.unwrap();

        let words = vec![
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
        ];

        let first = words
            .iter()
            .map(|(word, val)| (buf.find(word), val))
            .filter(|x| x.0.is_some())
            .map(|x| (x.0.unwrap(), x.1))
            .min()
            .unwrap()
            .1;
        let second = words
            .iter()
            .map(|(word, val)| (buf.rfind(word), val))
            .filter(|x| x.0.is_some())
            .map(|x| (x.0.unwrap(), x.1))
            .max()
            .unwrap()
            .1;
        let val: u32 = format!("{}{}", first, second).parse().unwrap();
        sum += val;
    }
    println!("{}", sum);
}
