#[allow(unused_imports)]
use std::collections::HashMap;
use std::io::{stdin, BufRead};

fn main() {
    let input = stdin().lock().lines().next().unwrap().unwrap();
    let seqs: Vec<Vec<u8>> = input.split(',').map(|s| s.as_bytes().to_vec()).collect();
    let mut sum = 0;
    for seq in seqs {
        let hash = seq
            .iter()
            .map(|&b| b as isize)
            .fold(0, |acc, b| (((acc + b) * 17) % 256));
        sum += hash;
    }
    println!("{}", sum);
}
