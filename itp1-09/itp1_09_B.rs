use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    while let Some(Ok(line)) = lines.next() {
        let initial_order = line.trim().to_string();
        if initial_order == "-" { break; }
        let m: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
        let mut current_order = initial_order;
        for _ in 0..m {
            if let Some(Ok(line)) = lines.next() {
                let h: usize = line.trim().parse().unwrap();
                current_order = shuffle(&current_order, h);
            }
        }
        println!("{}", current_order);
    }
}

fn shuffle(order: &str, h: usize) -> String {
    let len = order.len();
    let (left, right) = order.split_at(h);
    format!("{}{}", right, left)
}