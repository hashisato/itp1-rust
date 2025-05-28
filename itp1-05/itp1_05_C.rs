fn main() {
    loop {
        let (mut h, mut w) = input();
        if h == 0 && w == 0 { break }
        else { draw(h,w); }
    }
}

fn input() -> (i32, i32) {
    let mut num = String::new();
    std::io::stdin().read_line(&mut num).expect("Failed to read line");
    let mut iter = num.trim().split_whitespace();
    let h: i32 = iter.next().unwrap().parse().unwrap();
    let w: i32 = iter.next().unwrap().parse().unwrap();
    (h, w)
}

fn draw(h:i32, w:i32) {
    let mut index_i: i32 = 0;
    while index_i < h {
        let mut index_j: i32 = index_i % 2;
        while index_j < w {
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