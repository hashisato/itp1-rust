fn main(){
    swap();
}

fn swap(){
    loop{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let mut x = iter.next().unwrap().parse().unwrap_or(0);
        let mut y = iter.next().unwrap().parse().unwrap_or(0);
        if x == 0 && y == 0 { break; }
        else{
            if x <= y { println!("{} {}",x,y)}
            else { println!("{} {}",y,x)}
        }
    }
}