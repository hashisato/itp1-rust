fn main(){
    swap();
}

fn swap(){
    loop{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)
            .unwrap_or_else(|err| {
            println!("Failed to read line: {}", err);0
            });
        let mut iter = input.split_whitespace();
        let mut x = iter.next().unwrap().parse()
            .unwrap_or_else(|err| {
                println!("Index entered was not a number: {}", err);0
            });
        let mut y = iter.next().unwrap().parse()
            .unwrap_or_else(|err| {
                println!("Index entered was not a number: {}", err);0
            });
        if x == 0 && y == 0 { break; }
        else{
            if x <= y { println!("{} {}",x,y)}
            else { println!("{} {}",y,x)}
        }
    }
}