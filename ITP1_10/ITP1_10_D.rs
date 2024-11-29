use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let data = input();
    Minkowski(data);
}

fn input() -> Vec<Vec<f64>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut data: Vec<Vec<f64>> = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        let n: usize = line.parse().unwrap();
        for _ in 0..2 {
            let mut line = lines.next().unwrap().unwrap().trim().to_string();
            let words = line.split_whitespace().collect::<Vec<&str>>();
            let mut v = Vec::new();            
            for i in 0..n {
                let x = f64::from_str(words[i]).unwrap();
                v.push(x);
            }
            data.push(v);
        }
    }
    data
}

fn Minkowski(data: Vec<Vec<f64>>) {
    for i in 1..=3 {
        let mut ans: f64 = 0.0;
        for j in 0..data[0].len() {
            ans += (data[0][j]-data[1][j]).abs().powf(i as f64);
        }
        println!("{}",ans.powf(1.0/i as f64));
    }
    let mut now: f64 = 0.0;
    let mut max: f64 = 0.0;
    for j in 0..data[0].len() {
        now = (data[0][j]-data[1][j]).abs();
        if max < now { max = now; }
    }
    println!("{}",max);
}