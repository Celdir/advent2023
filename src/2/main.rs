#[allow(unused_imports)]
use std::collections::HashMap;
use std::io::{stdin, BufRead};

fn main() {
    //let limits = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut sum = 0;
    for line in stdin().lock().lines() {
        let mut buf = line.unwrap();
        let mut game = buf.split_off(buf.find(":").unwrap());
        //let id: usize = buf.split_off(5).parse().unwrap();

        game.remove(0);


        //let mut possible = true;
        let mut max = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        for round in game.split(";") {
            for cubes in round.split(",") {
                let parts: Vec<_> = cubes.trim().split(" ").collect();
                let count: usize = parts[0].parse().unwrap();
                let color = parts[1];
                /*if limits[color] < count {
                    possible = false;
                }*/
                max.entry(color)
                    .and_modify(|e| *e = std::cmp::max(*e, count))
                    .or_insert(count);
            }
        }
        /*if possible {
            sum += id;
        }*/
        sum += max["red"] * max["green"] * max["blue"];
    }
    println!("{}", sum);
}
