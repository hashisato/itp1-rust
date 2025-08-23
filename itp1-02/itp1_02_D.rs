fn main(){
    let mut num = input();
    println!("{}", check(num[0], num[1], num[2], num[3], num[4]));
}

fn input() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn check(w: i32, h: i32, x: i32, y: i32, r: i32) -> String{
    match (
        0 <= x - r,
        0 <= y - r,
        x + r <= w,
        y + r <= h
    ) {
        (true, true, true, true) => "Yes".to_string(),
        _ => "No".to_string(),
    }
}