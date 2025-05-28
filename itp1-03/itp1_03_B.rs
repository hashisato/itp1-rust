fn main(){
    testCase();
}

fn testCase(){
    let mut i: i32 = 0;
    loop{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut x = input.trim().parse().unwrap_or_else(|_| 0);
        if x == 0 { break; }
        else{
            i = i + 1;
            println!("Case {}: {}",i,x);
        }
    }
}