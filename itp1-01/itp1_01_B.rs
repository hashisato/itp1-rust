fn main(){
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let x: usize = input.trim().parse().unwrap();
    println!("{}", x * x * x);
}