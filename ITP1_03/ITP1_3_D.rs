fn main(){
    let (mut a,mut b,mut c) = input();
    let ans = divisor(a,b,c);
    println!("{}",ans);
}

fn input() -> (u32,u32,u32){
    let mut data: [u32; 3] = [0,0,0];
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
        .unwrap_or_else(|err| {
            println!("Failed to read line: {}", err);0
        });
    let mut iter = input.split_whitespace();
    for i in 0..3{
        data[i] = iter.next().unwrap().parse()
            .unwrap_or_else(|err| {
                println!("Index entered was not a number: {}", err);0
            });
    }
    return (data[0],data[1],data[2]);
}

fn divisor(a: u32,b: u32,c: u32) -> (u32){
    let mut count: u32 = 0;
    for i in a..(b+1){
        if(c%i == 0){ count+= 1; }
    }
    return (count);
}