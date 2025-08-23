fn main(){
    let (mut a, mut b) = input();
    println!("{} {}", a*b, 2*(a+b));
}

fn input() -> (usize, usize){
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let a: usize = iter.next().unwrap().parse().unwrap();
    let b: usize = iter.next().unwrap().parse().unwrap();
    (a, b)
}
