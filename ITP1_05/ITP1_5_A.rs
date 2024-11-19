fn main() {
    loop {
        let (mut H, mut W) = input();
        let mut index_i: i32 = 0;
        let mut index_j: i32 = 0;
        if H == 0 && W == 0 { break }
        else {
            while index_i<H {
                index_i += 1;
                while index_j<W {
                    index_j += 1;
                    print!("#");
                }
                index_j = 0;
                println!("");
            }
        }
        println!();
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