fn main(){
    testCase();
}

fn testCase(){
    let mut i: i32 = 0;
    loop{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)
            .unwrap_or_else(|err| {
            println!("Failed to read line: {}", err);0
            });
        let mut x = input.trim().parse()
        .unwrap_or_else(|err| {
            println!("Index entered was not a number: {}", err);0
        });
        if x == 0 { break; }
        else{
            i = i + 1;
            println!("Case {}: {}",i,x);
        }
    }
}