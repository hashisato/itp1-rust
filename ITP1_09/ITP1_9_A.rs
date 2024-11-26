use std::io::{self, Read};

fn main() {
    let (w, t) = input();
    search(w, t);
}

fn input() -> (String, String) {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("Failed to read line!!");
    let w : String = s.trim().parse::<String>().unwrap().to_lowercase();
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).expect("Failed to read line!!");
    let mut t:String = s.parse::<String>().unwrap().to_lowercase();
    (w, t)
}

fn search(w: String, t: String) {
    let count = t.split_whitespace().filter(|&word| word == w).count();
    println!("{}",count);
}