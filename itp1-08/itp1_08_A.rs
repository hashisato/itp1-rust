fn main() {
    let mut s = input();
    toggle(s);
}

fn input() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("Failed to read line!!");
    s
}

fn toggle(s: String) {
    for c in s.chars() {
        if !c.is_alphabetic() { print!("{}",c); }
        else {
            if c.is_uppercase() { print!("{}", c.to_lowercase()); }
            else { print!("{}", c.to_uppercase()); }
        }
    }
}