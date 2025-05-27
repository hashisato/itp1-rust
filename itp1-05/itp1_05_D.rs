fn main() {
    let mut n = input();
    let mut i = 1;
    while i <= n {
        let mut x = i;
        if x % 3 == 0 {
            print!(" {}",i);
        }
        else {
            while x != 0 {
                if x % 10 == 3 {
                    print!(" {}",i);
                    break;
                }
                x /= 10;
            }
        }
        i += 1;
    }
    println!();
}

fn input() -> i32 {
    let mut num = String::new();
    std::io::stdin().read_line(&mut num).expect("読み込みに失敗");
    let mut iter = num.trim().split_whitespace();
    let n: i32 = iter.next().unwrap().parse().unwrap();
    n
}