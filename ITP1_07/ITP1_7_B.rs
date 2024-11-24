fn main() {
    loop {
        let (n, x) = input();
        if n == 0 && x == 0 { break }
        checker(n, x);
    }
}
fn input() -> (u64, u64) {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("Failed to read line!!");
    let mut iter = s.trim().split_whitespace();
    let n: u64 = iter.next().unwrap().parse().expect("Failed to parse");
    let x: u64 = iter.next().unwrap().parse().expect("Failed to parse");
    (n, x)
}
fn checker(n: u64, x: u64) {
    let mut count: u64 = 0;
    for i in 1..n-1 {
        for j in i+1..n {
            for k in j+1..n+1 {
                if i+j+k == x { count += 1; }
                else { count += 0; }
            }
        }
    }
    println!("{}",count);
}