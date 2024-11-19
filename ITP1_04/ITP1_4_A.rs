fn main() {
    let (a, b) = input();
    let d = a / b;
    let r = a % b;
    let f = a as f64 / b as f64;
    println!("{} {} {}",d,r,f);
}

fn input() -> (i32,i32) {
    let mut data = String::new();
    std::io::stdin().read_line(&mut data).expect("読み込みに失敗");
    let mut iter = data.trim().split_whitespace();
    let a: i32 = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();
    (a, b)
}