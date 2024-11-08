fn main(){
    let (mut W,mut H,mut x,mut y,mut r) = input();
    println!("{}",check(W,H,x,y,r));
}

fn input() -> (i32,i32,i32,i32,i32){
    let mut data: [i32; 5] = [0,0,0,0,0];
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
        .unwrap_or_else(|err| {
            println!("Failed to read line: {}", err);0
        });
    let mut iter = input.split_whitespace();
    let mut i = 0;
    loop{
        data[i] = iter.next().unwrap().parse()
        .unwrap_or_else(|err| {
            println!("Index entered was not a number: {}", err);0
        });
        i = i+1;
        if(i == 5){ break; }
    }
    return (data[0],data[1],data[2],data[3],data[4]);
}

fn check(W: i32,H: i32,x: i32,y: i32,r: i32) -> String{
    if(0<=(x-r) && 0<=(y-r)){
        if((x+r)<=W && (y+r)<=H){ return "Yes".to_string(); }
        else{ return "No".to_string(); }
    }
    else{ return "No".to_string(); }
}