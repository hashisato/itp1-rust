fn main() {
    let mut data = input();
    let min: i64 = *data.iter().min().unwrap();
    let max: i64 = *data.iter().max().unwrap();
    let total: i64 = data.iter().sum();
    println!("{} {} {}",min,max,total);
}

fn input() -> Vec<i64> {
    let mut num = String::new();
    std::io::stdin().read_line(&mut num).expect("Failed to read line");

    let mut data = String::new();
    std::io::stdin().read_line(&mut data).unwrap();
    data.trim().split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<i64>>()
}
