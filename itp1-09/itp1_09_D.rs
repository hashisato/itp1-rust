use std::io::{self, BufRead};

fn main() {
    string_conversion();
}

fn string_conversion() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut s = lines.next().unwrap().unwrap().trim().to_string();
    let num: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..num {
        let line = lines.next().unwrap().unwrap();
        let mut words = line.split_whitespace();
        let cmd = words.next().unwrap();
        let head: usize = words.next().unwrap().parse().unwrap();
        let tail: usize = words.next().unwrap().parse().unwrap();
        match cmd {
            "print" => println!("{}",&s[head..=tail]),
            "reverse" => {
                let mid = &s[head..=tail].chars().rev().collect::<String>();
                s = format!("{}{}{}", &s[..head], mid, &s[tail+1..]);
            }
            "replace" => {
                let rep = words.next().unwrap();
                s = format!("{}{}{}", &s[..head], rep, &s[tail+1..]);
            }
            _ => unreachable!()
        }
    }
}