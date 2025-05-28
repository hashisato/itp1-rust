fn main() {
    let mut a = input();
    for i in (0..a.len()).rev() {
        print!("{}",a[i]);
        if i != 0 { print!(" "); }
        else { println!(); }
    }
}

fn input() -> Vec<i32> {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("Failed to read line");
    let mut iter = n.trim().split_whitespace();
    let n: i32 = iter.next().unwrap().parse().unwrap();

    let mut a = String::new();
    std::io::stdin().read_line(&mut a).expect("Failed to read line");
    let mut iter = a.trim().split_whitespace();
    let a: Vec<i32> = iter.map(|s| s.parse().unwrap()).collect();
    a
}