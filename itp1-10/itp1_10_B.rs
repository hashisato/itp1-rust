use std::io::{self, BufRead};
use std::str::FromStr;
use std::f64::consts::PI;

fn main() {
    let data = input();
    triangle(data);
}

fn input() -> Vec<f64> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut line = lines.next().unwrap().unwrap().trim().to_string();
    let words = line.split_whitespace().collect::<Vec<&str>>();
    let mut data: Vec<f64> = Vec::new();
    for i in 0..3 {
        let x = f64::from_str(words[i]).unwrap();
        data.push(x);
    }
    data
}

fn triangle(data: Vec<f64>) {
    let a: f64 = data[0];
    let b: f64 = data[1];
    let rad: f64 = data[2]*PI/180.0;
    let S: f64 = data[0]*data[1]*rad.sin()/2.0;
    let L: f64 = (data[0].powf(2.0)+data[1].powf(2.0)-2.0*data[0]*data[1]*rad.cos()).sqrt()+data[0]+data[1];
    let h: f64 = S*2.0/data[0];
    println!("{}\n{}\n{}",S,L,h);
}