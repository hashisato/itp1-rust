fn main() {
    loop {
        let (a, b, op) = input();
        if op == "?" { break }
        calc(a, b, op);
    }
}

fn input() -> (i32, i32, String) {
    let mut data = String::new();
    std::io::stdin().read_line(&mut data).expect("Failed to read line");
    let mut iter = data.trim().split_whitespace();
    let a: i32 = iter.next().unwrap().parse().unwrap();
    let op: String = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();
    (a, b, op)
}

fn calc(a: i32, b: i32, op: String) {
    match op.as_str() {
        "+" => println!("{}", a + b),
        "-" => println!("{}", a - b),
        "*" => println!("{}", a * b),
        "/" => println!("{}", a / b),
        _ => {}
    }
}