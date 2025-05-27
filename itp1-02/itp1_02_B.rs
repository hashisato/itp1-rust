fn main(){
    let (mut a,mut b,mut c) = input();
    println!("{}",compare(a,b,c));
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

fn compare(a: i32,b: i32,c: i32) -> String{
    if(a<b && b<c){ return "Yes".to_string(); }
    else{ return "No".to_string(); }
}