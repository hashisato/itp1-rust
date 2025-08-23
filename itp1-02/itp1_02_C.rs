fn main(){
    let mut numbers = input();
    numbers.sort();
    println!("{} {} {}",numbers[0],numbers[1],numbers[2]);
}

fn input() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
