use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let (data, word) = input();
    dice(data, word);
}

fn input() -> (Vec<usize>, String) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut line = lines.next().unwrap().unwrap().trim().to_string();
    let words = line.split_whitespace().collect::<Vec<&str>>();
    let mut data: Vec<usize> = Vec::new();
    for i in 0..6 {
        let x = usize::from_str(words[i]).unwrap();
        data.push(x);
    }
    line = lines.next().unwrap().unwrap().trim().to_string();
    (data, line)
}

fn dice(data: Vec<usize>, word: String) {
    let mut surface: [usize; 6] = [data[0], data[1], data[2], data[3], data[4], data[5]];
    for c in word.chars() {
        match c {
            'N' => {
                surface = [surface[1], surface[5], surface[2], surface[3], surface[0], surface[4]];
            }
            'S' => {
                surface = [surface[4], surface[0], surface[2], surface[3], surface[5], surface[1]];
            }
            'E' => {
                surface = [surface[3], surface[1], surface[0], surface[5], surface[4], surface[2]];
            }
            'W' => {
                surface = [surface[2], surface[1], surface[5], surface[0], surface[4], surface[3]];
            }
            _ => unreachable!()
        }
    }
    println!("{}",surface[0]);
}