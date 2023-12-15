#[allow(unused_imports)]
use std::collections::HashMap;
use std::io::{stdin, BufRead};

fn hash(label: &str) -> usize {
    label
        .chars()
        .map(|b| b as usize)
        .fold(0, |acc, b| (((acc + b) * 17) % 256))
}

fn main() {
    let input = stdin().lock().lines().next().unwrap().unwrap();
    let ops: Vec<&str> = input.split(',').collect();
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];
    for op in ops {
        if op.contains('=') {
            let (label, val_str) = op.split_at(op.find('=').unwrap());
            let val: usize = val_str.strip_prefix('=').unwrap().parse().unwrap();
            let hash = hash(&label);
            let index = boxes[hash].iter().position(|&(l, _)| l == label);
            if let Some(i) = index {
                *boxes[hash].get_mut(i).unwrap() = (label, val);
            } else {
                boxes[hash].push((label, val));
            }
        } else {
            let label = op.strip_suffix('-').unwrap();
            let hash = hash(&label);
            let index = boxes[hash].iter().position(|&(l, _)| l == label);
            if index.is_some() {
                boxes[hash].remove(index.unwrap());
            }
        }
    }

    let mut sum = 0;
    for i in 0..boxes.len() {
        for j in 0..boxes[i].len() {
            let val = boxes[i][j].1;
            sum += (i+1) * (j+1) * val;
        }
    }
    println!("{}", sum);
}
