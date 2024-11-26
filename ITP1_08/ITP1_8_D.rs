use std::io::{self, Read};

fn main() {
    let (mut s_String, mut p) = input();
    let s = format!("{}{}",s_String,s_String);
    ring(s, p);
}

fn input() -> (String, String) {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).expect("Failed to read line!!");
    let mut iter = s.trim().split_whitespace();
    let s_String: String = iter.next().unwrap().parse().unwrap();
    let p: String = iter.next().unwrap().parse().unwrap();
    (s_String, p)
}

fn ring(s: String, p: String) {
    if s.contains(&p) { println!("Yes"); }
    else { println!("No"); }
}