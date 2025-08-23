fn main(){
    let (a, b) = input();
    println!("a {} b",compare(a,b));
}

fn input() -> (i32, i32){
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();
    (a, b)
}

fn compare(a: i32,b: i32) -> String{
    match a.cmp(&b) {
        std::cmp::Ordering::Less => "<".to_string(),
        std::cmp::Ordering::Greater => ">".to_string(),
        std::cmp::Ordering::Equal => "==".to_string(),
    }
}