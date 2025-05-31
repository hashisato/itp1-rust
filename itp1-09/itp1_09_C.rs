use std::io::{self, BufRead};

fn main() {
    let scores: [usize; 2] = cardGame();
    println!("{} {}", scores[0], scores[1]);
}

fn cardGame() -> [usize; 2] {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let num: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut scores: [usize; 2] = [0, 0];
    for _ in 0..num {
        let mut line = lines.next().unwrap().unwrap();
        let trimmed_line = line.trim();
        let words = trimmed_line.split_whitespace().collect::<Vec<&str>>();
        if words[0].to_lowercase() > words[1].to_lowercase() {
            scores[0] += 3;
        }
        else if words[0].to_lowercase() == words[1].to_lowercase() {
            scores[0] += 1;
            scores[1] += 1;
        }
        else {
            scores[1] += 3;
        }
    }
    scores
}