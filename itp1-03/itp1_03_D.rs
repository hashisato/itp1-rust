fn main(){
    let data = input();
    let ans = (data[0]..=data[1]).filter(|&i| data[2]%i == 0).count();
    println!("{}",ans);
}

fn input() -> Vec<u32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let data: Vec<u32> = iter.map(|x| x.parse().unwrap()).collect();
    data
}