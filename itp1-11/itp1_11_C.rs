use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let (data1, data2) = input();
    for i in 0..4 {
        if checkDice(&data1, data1[0], data1[i + 1]) != checkDice(&data2, data1[0], data1[i + 1]) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

fn input() -> (Vec<usize>, Vec<usize>) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut data: Vec<Vec<usize>> = Vec::new();
    for _ in 0..2 {
        let mut line = lines.next().unwrap().unwrap().trim().to_string();
        let mut words = line.split_whitespace().collect::<Vec<&str>>();
        let mut innerData: Vec<usize> = Vec::new();
        for i in 0..6 {
            let x = usize::from_str(words[i]).unwrap();
            innerData.push(x);
        }
        data.push(innerData);
    }
    (data.remove(0), data.remove(0))
}

fn checkDice(data: &[usize], top: usize, front: usize) -> usize {
    let tmp: [[usize; 5]; 3] = [[1, 2, 4, 3, 1], [0, 3, 5, 2, 0], [0, 1, 5, 4, 0]];
    for i in 0..3 {
        for j in 0..4 {
            if (data.iter().position(|&x| x == top) == Some(tmp[i][j])) && (data.iter().position(|&x| x == front) == Some(tmp[i][j + 1])) {
                return data[i];
            }else if (data.iter().position(|&x| x == front) == Some(tmp[i][j])) && (data.iter().position(|&x| x == top) == Some(tmp[i][j + 1])) {
                return data[5 - i];
            }
        }
    }
    return 0;
}