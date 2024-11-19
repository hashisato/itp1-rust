use std::f64::consts::PI;

fn main() {
    let r: f64 = input();
    let area = (r as f64) * (r as f64) * PI;
    let circ = (r as f64) * PI * 2.0;
    println!("{} {}",area,circ);
}

fn input() -> f64 {
    let mut data = String::new();
    std::io::stdin().read_line(&mut data).expect("読み込みに失敗");
    let mut iter = data.trim().split_whitespace();
    let r: f64 = iter.next().unwrap().parse().unwrap();
    r
}