fn main() {
    let (mut r, mut c, mut data) = input();
    draw(r, c, data);
}

fn input() -> (u64, u64, Vec<Vec<usize>>) {
    let mut s = String::new();     //r,cの入力
    std::io::stdin().read_line(&mut s).expect("Failed to read line!!");
    let mut iter = s.trim().split_whitespace();
    let r: u64 = iter.next().unwrap().parse().expect("Failed to parse");
    let c: u64 = iter.next().unwrap().parse().expect("Failed to parse");
    let mut data: Vec<Vec<usize>> = vec![];     //表の入力
    for _ in 0..r {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).expect("Failed to read line!!");
        let mut iter = s.trim().split_whitespace();
        let mut v = vec![];
        for _ in 0..c {
            let element: usize = iter.next().unwrap().parse().expect("Failed to parse");
            v.push(element);
        }
        data.push(v);
    }
    (r, c, data)
}

fn draw(r: u64, c: u64, mut data: Vec<Vec<usize>>) {
    let r: usize = r as usize;
    let c: usize = c as usize;
    for i in 0..r {
        let mut sum_r = 0;
        for j in 0..c {
            print!("{} ",data[i][j]);
            sum_r += data[i][j];
        }
        data[i].push(sum_r);     //i行のc列目にsum_rを追加
        println!("{}",sum_r);
    }
    for j in 0..c+1 {
        let mut sum_c = 0;
        for i in 0..r {
            sum_c += data[i][j];
        }
        if j != c { print!("{} ",sum_c); }
        else { println!("{}",sum_c); }
    }
}
