use std::f64::consts::PI;

fn main() {
    let r: f64 = input();
    let area: f64 = (r as f64) * (r as f64) * PI;
    let circ: f64 = (r as f64) * PI * 2.0;
    println!("{} {}",area,circ);
}

fn input() -> f64 {
    let mut data = String::new();
    std::io::stdin().read_line(&mut data).expect("Failed to read line");
    let mut iter = data.trim().split_whitespace();
    iter.next().unwrap().parse().unwrap()
}