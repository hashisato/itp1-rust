fn main(){
    let (mut a, mut b) = input();
    println!("{} {}", a*b,2*(a+b));
}

fn input() -> (i32, i32){
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
        .unwrap_or_else(|err| {
            println!("Failed to read line: {}", err);0
        });
    let mut iter = input.split_whitespace();
    let a: i32 = iter.next().unwrap().parse()
        .unwrap_or_else(|err| {
            println!("Index entered was not a number: {}", err);0
        });
    let b: i32 = iter.next().unwrap().parse()
        .unwrap_or_else(|err| {
            println!("Index entered was not a number: {}", err);0
        });
    (a, b)
}