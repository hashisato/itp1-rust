use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let (data, q) = input();
    for _ in 0..q {
        let mut nums = String::new();
        io::stdin().read_line(&mut nums).unwrap();
        let (top, front) = {
            let nums: Vec<usize> = nums.split_whitespace().map(|s| s.parse().unwrap()).collect();
            (nums[0], nums[1])
        };
        checkDice(&data, top, front);
    }
}

fn input() -> (Vec<usize>, usize) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut line = lines.next().unwrap().unwrap().trim().to_string();
    let words = line.split_whitespace().collect::<Vec<&str>>();
    let mut data: Vec<usize> = Vec::new();
    for i in 0..6 {
        let x = usize::from_str(words[i]).unwrap();
        data.push(x);
    }
    let q: usize = lines.next().unwrap().unwrap().parse().unwrap();
    (data, q)
}

fn checkDice(data: &[usize], top: usize, front: usize) {
    let tmp: [[usize; 5]; 3] = [[1, 2, 4, 3, 1], [0, 3, 5, 2, 0], [0, 1, 5, 4, 0]];
    for i in 0..3 {
        for j in 0..4 {
            if (data.iter().position(|&x| x == top) == Some(tmp[i][j])) && (data.iter().position(|&x| x == front) == Some(tmp[i][j + 1])) {
                println!("{}", data[i]);
                return;
            }else if (data.iter().position(|&x| x == front) == Some(tmp[i][j])) && (data.iter().position(|&x| x == top) == Some(tmp[i][j + 1])) {
                println!("{}", data[5 - i]);
                return;
            }
        }
    }
}