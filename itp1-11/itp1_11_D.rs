use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let data = input();
    let n = data.len();
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            if checkDice(&data[i], &data[j]) {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}

fn input() -> Vec<Vec<usize>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut data = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let dice = line
            .split_whitespace()
            .map(|s| usize::from_str(s).unwrap())
            .collect();
        data.push(dice);
    }
    data
}

fn checkDice(data1: &Vec<usize>, data2: &Vec<usize>) -> bool {
    let mut data = data1.clone();
    for i in 0..6 {
        for _ in 0..4 {
            if data == *data2{
                return true;
            }
            rotateRight(&mut data);
        }
        if i % 2 == 0 {
            rotateForward(&mut data);
        } else {
            rotateLeft(&mut data);
        }
    }
    false
}

fn rotateRight(data: &mut Vec<usize>) {
    let tmp = data[1];
    data[1] = data[2];
    data[2] = data[4];
    data[4] = data[3];
    data[3] = tmp;
}

fn rotateForward(d: &mut Vec<usize>) {
    let tmp = d[0];
    d[0] = d[1];
    d[1] = d[5];
    d[5] = d[4];
    d[4] = tmp;
}

fn rotateLeft(d: &mut Vec<usize>) {
    let tmp = d[0];
    d[0] = d[2];
    d[2] = d[5];
    d[5] = d[3];
    d[3] = tmp;
}