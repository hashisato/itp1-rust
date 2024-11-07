fn main(){
    let (mut a,mut b,mut c) = input();
    let mut numbers = vec![a,b,c];
    numbers.sort();
    println!("{} {} {}",numbers[0],numbers[1],numbers[2]);
}

fn input() -> (i32,i32,i32){
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
    let c: i32 = iter.next().unwrap().parse()
        .unwrap_or_else(|err| {
            println!("Index entered was not a number: {}", err);0
        });
    return (a,b,c);
}