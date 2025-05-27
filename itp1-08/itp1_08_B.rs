fn main() {
    loop {
        let mut s = input();
        if s == "0" { break; }
        else { sum(s); }
    }
}

fn input() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("Failed to read line!!");
    s.trim().to_string()
}

fn sum(s: String) {
    let mut ans: u32 = 0;
    for c in s.chars() {
        ans += c.to_digit(10).unwrap();
    }
    println!("{}",ans);
}