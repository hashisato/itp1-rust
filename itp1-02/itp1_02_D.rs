fn main(){
    let (mut w,mut h,mut x,mut y,mut r) = input();
    println!("{}",check(w,h,x,y,r));
}

fn input() -> (i32,i32,i32,i32,i32){
    let mut data: [i32; 5] = [0,0,0,0,0];
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let v: Vec<i32> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
    return (v[0],v[1],v[2],v[3],v[4])
}

fn check(w: i32,h: i32,x: i32,y: i32,r: i32) -> String{
    if(0<=(x-r) && 0<=(y-r)){
        if((x+r)<=w && (y+r)<=h){ 
            return "Yes".to_string();
        }
        else{
            return "No".to_string();
        }
    }
    else{
        return "No".to_string();
    }
}