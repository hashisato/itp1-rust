fn main(){
    let mut s = input();
    let mut x: (i32, i32, i32) = watch(s);
    println!("{}:{}:{}", x.0,x.1,x.2);
}

fn input() -> (i32){
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let i: i32 = iter.next().unwrap().parse().unwrap();
    return (i);
}

fn watch(mut i: i32) -> (i32, i32, i32){
    let h = i/3600;
    i %= 3600;
    let m = i/60;
    let s = i%60;
    return (h,m,s);
}