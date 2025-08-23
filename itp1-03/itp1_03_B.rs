fn main() {
    let mut case = 1;
    for line in std::io::stdin().lines() {
        let x: i32 = line.unwrap().trim().parse().unwrap();
        if x == 0 { break; }
        println!("Case {}: {}", case, x);
        case += 1;
    }
}