fn main(){
    let mut x = String::new();
    std::io::stdin()
        .read_line(&mut x)
        .unwrap_or_else(|err| {
            println!("Failed to read line: {}", err);0
        });
    let x: usize = x.trim()
        .parse()
        .unwrap_or_else(|err| {
            println!("Index entered was not a number: {}", err);0
        });
    println!("{}",x*x*x);
}