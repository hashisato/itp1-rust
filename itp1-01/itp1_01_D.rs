fn main(){
    let mut s = input();
    let mut x: (i32, i32, i32) = watch(s);
    println!("{}:{}:{}", x.0,x.1,x.2);
}

fn input() -> (i32){
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
        .unwrap_or_else(|err| {
            println!("Failed to read line: {}", err);0
        });
    let mut iter = input.split_whitespace();
    let i: i32 = iter.next().unwrap().parse()
        .unwrap_or_else(|err| {
            println!("Index entered was not a number: {}", err);0
        });
    return (i);
}

fn watch(mut i: i32) -> (i32, i32, i32){
    let h = i/3600;
    i %= 3600;
    let m = i/60;
    let s = i%60;
    return (h,m,s);
}