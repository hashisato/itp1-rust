fn main(){
    let vec: Vec<i32> = input();
    match (
        vec[0] < vec[1],
        vec[1] < vec[2]
    ) {
        (true, true) => println!("Yes"),
        _ => println!("No"),
    }
}

fn input() -> Vec<i32>{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
