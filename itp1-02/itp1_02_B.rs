fn main(){
    let (a, b, c) = input();
    if a<b && b<c { println!("Yes"); }
    else { println!("No"); }
}

fn input() -> (i32, i32, i32){
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();
    let c: i32 = iter.next().unwrap().parse().unwrap();
    (a, b, c)
}
