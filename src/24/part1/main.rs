#[allow(unused_imports)]
use std::collections::HashMap;
use std::io::{stdin, BufRead};
use std::ops::{Add, Mul};

#[derive(PartialEq, Copy, Clone, Debug)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl From<&str> for Vector {
    fn from(value: &str) -> Self {
        let parts: Vec<_> = value.split(", ").collect();
        assert_eq!(parts.len(), 3);
        Vector {
            x: parts[0].trim().parse().unwrap(),
            y: parts[1].trim().parse().unwrap(),
            z: 0.0, //parts[2].trim().parse().unwrap(),
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
struct Ray {
    position: Vector,
    velocity: Vector,
}

impl From<String> for Ray {
    fn from(value: String) -> Self {
        let (s, e) = value.split_once(" @ ").unwrap();
        Ray {
            position: Vector::from(s),
            velocity: Vector::from(e),
        }
    }
}

fn intersects(a: &Ray, b: &Ray) -> bool {
    let (ap, ad) = (a.position, a.velocity);
    let (bp, bd) = (b.position, b.velocity);
    let u = (ap.y * bd.x + bd.y * bp.x - bp.y * bd.x - bd.y * ap.x) / (ad.x * bd.y - ad.y * bd.x);
    let v = (ap.x + ad.x * u - bp.x) / bd.x;

    if u > 0.0 && v > 0.0 {
        let lower_bound = 200000000000000.0;
        let upper_bound = 400000000000000.0;

        let intersection = ap + ad * u;
        return intersection.x >= lower_bound
            && intersection.x <= upper_bound
            && intersection.y >= lower_bound
            && intersection.y <= upper_bound;
    }
    false
}

fn main() {
    let stone: Vec<Ray> = stdin()
        .lock()
        .lines()
        .map(|l| Ray::from(l.unwrap()))
        .collect();
    let mut ans = 0;
    for i in 0..stone.len() {
        for j in (i + 1)..stone.len() {
            if intersects(&stone[i], &stone[j]) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
