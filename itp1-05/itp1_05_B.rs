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
    let mut index_j: i32 = 0;
    while index_i<h {
        if index_i == 0 || index_i == (h-1) {
            while index_j<w {
                index_j += 1;
                print!("#");
            }
        }
        else {
            while index_j<w {
                if index_j == 0 || index_j == (w-1) {
                    print!("#");
                }
                else {
                    print!(".");
                }
                index_j += 1;
            }   
        }
        index_i += 1;
        index_j = 0;
        println!();
    }
    println!();
}