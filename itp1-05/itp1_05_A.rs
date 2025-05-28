fn main() {
    loop {
        let (mut h, mut w) = input();
        let mut index_i: i32 = 0;
        let mut index_j: i32 = 0;
        if h == 0 && w == 0 { break }
        else {
            while index_i<h {
                index_i += 1;
                while index_j<w {
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
    std::io::stdin().read_line(&mut num).expect("Failed to read line");
    let mut iter = num.trim().split_whitespace();
    let h: i32 = iter.next().unwrap().parse().unwrap();
    let w: i32 = iter.next().unwrap().parse().unwrap();
    (h, w)
}