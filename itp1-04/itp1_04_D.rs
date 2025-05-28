fn main() {
    let mut data = input();
    let min = data.iter().min().unwrap();
    let max = data.iter().max().unwrap();
    let total: i64 = data.iter().sum();
    println!("{} {} {}",min,max,total);
}

fn input() -> Vec<i64> {
    let mut num = String::new();
    std::io::stdin().read_line(&mut num).expect("Failed to read line");
    let num: i32 = num.trim().parse().unwrap();
    
    let mut data = String::new();
    std::io::stdin().read_line(&mut data).unwrap();
    let mut iter = data.trim().split_whitespace();

    let mut v: Vec<i64> = vec![];
    for _ in 0..num {
        v.push(iter.next().unwrap().parse().unwrap());
    }
    v
}