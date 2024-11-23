fn main() {
    let mut a = input();
    for i in (0..a.len()).rev() {
        print!("{}",a[i]);
        if i != 0 { print!(" "); }
        else { println!(); }
    }
}

fn input() -> Vec<i32> {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("読み込みに失敗");
    let mut iter = n.trim().split_whitespace();
    let n: i32 = iter.next().unwrap().parse().unwrap();

    let mut a = String::new();
    std::io::stdin().read_line(&mut a).expect("読み込みに失敗");
    let mut iter = a.trim().split_whitespace();
    let vec: Vec<i32> = vec![];
    let mut a = Vec::new();
    for __ in 0..n {
        a.push(iter.next().unwrap().parse().unwrap());
    }
    a
}