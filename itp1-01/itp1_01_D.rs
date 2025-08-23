fn main(){
    let mut s = input();
    let h: usize = s / 3600;
    s %= 3600;
    let m: usize = s / 60;
    s %= 60;
    println!("{}:{}:{}", h, m, s);
}

fn input() -> (usize){
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut s: usize = input.trim().parse().unwrap();
    return (s);
}
