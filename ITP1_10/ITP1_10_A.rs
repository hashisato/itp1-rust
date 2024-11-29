use std::io::{self, BufRead};
use std::str::FromStr;

struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let P = input();
    distance(P);
}

fn input() -> Vec<Point> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut cmd = lines.next().unwrap().unwrap().trim().to_string();
    let words = cmd.split_whitespace().collect::<Vec<&str>>();
    let mut points: Vec<Point> = Vec::new();
    for i in (0..4).step_by(2) {
        let x = f64::from_str(words[i]).unwrap();
        let y = f64::from_str(words[i+1]).unwrap();
        points.push(Point { x, y });
    }
    points
}

fn distance(P: Vec<Point>) {
    let x: f64 = (P[0].x - P[1].x).powf(2.0);
    let y: f64 = (P[0].y - P[1].y).powf(2.0);
    println!("{}",(x+y).sqrt());
}