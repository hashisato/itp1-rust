fn main() {
    loop {
        let (m, f, r) = input();
        if m == -1 && f == -1 && r ==-1 { break }
        else if m == -1 || f == -1 { println!("F") }
        else if 80 <= (m + f) { println!("A") }
        else if 65 <= (m + f) { println!("B") }
        else if 50 <= (m + f) || 50 <= r { println!("C") }
        else if 30 <= (m + f) { println!("D") }
        else { println!("F") }
    }
}
fn input() -> (i8, i8, i8) {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("Failed to read line!!");
    let mut iter = s.trim().split_whitespace();
    let m: i8 = iter.next().unwrap().parse().expect("Failed to parse");
    let f: i8 = iter.next().unwrap().parse().expect("Failed to parse");
    let r: i8 = iter.next().unwrap().parse().expect("Failed to parse");
    (m, f, r)
}