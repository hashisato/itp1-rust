fn main() {
    loop {
        let (mut H, mut W) = input();
        if H == 0 && W == 0 { break }
        else { draw(H,W); }
    }
}

fn input() -> (i32, i32) {
    let mut num = String::new();
    std::io::stdin().read_line(&mut num).expect("読み込みに失敗");
    let mut iter = num.trim().split_whitespace();
    let H: i32 = iter.next().unwrap().parse().unwrap();
    let W: i32 = iter.next().unwrap().parse().unwrap();
    (H, W)
}

fn draw(H:i32, W:i32) {
    let mut index_i: i32 = 0;
    while index_i < H {
        let mut index_j: i32 = index_i % 2;
        while index_j < W {
            if index_j % 2 == 0 { print!("#"); }
            else { print!("."); }
            index_j += 1;
        }
        if index_i % 2 == 1 && index_j % 2 == 0 { print!("#"); }
        else if index_i % 2 == 1 && index_j % 2 == 1 { print!("."); }
        index_i += 1;
        println!();
    }
    println!();
}