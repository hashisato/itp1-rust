use std::io::{self, BufRead};
use std::str::FromStr;

fn main (){
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    while let Some(Ok(line)) = lines.next() {
        let n: usize = line.parse().unwrap();
        if n == 0 { break; }
        let mut line = lines.next().unwrap().unwrap().trim().to_string();
        let words = line.split_whitespace().collect::<Vec<&str>>();
        let mut data: Vec<f64> = Vec::new();
        for i in 0..n {
            let x = f64::from_str(words[i]).unwrap();
            data.push(x);
        }
        SD(data);
    }
}

fn SD(data: Vec<f64>) {
    let avg: f64 = data.iter().sum::<f64>()/data.len() as f64;
    let mut a: f64 = 0.0;
    for i in 0..data.len() {
        a += (data[i]-avg).powf(2.0);
    }
    a /= data.len() as f64;
    println!("{}",a.sqrt());
}