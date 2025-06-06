fn main(){
    let (mut a,mut b) = input();
    println!("a {} b",compare(a,b));
}

fn input() -> (i32,i32){
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();
    return (a,b)
}

fn compare(a: i32,b: i32) -> String{
    if(a<b){
        return "<".to_string();
    }
    else if(a>b){
        return ">".to_string();
    }
    else{
        return "==".to_string();
    }
}