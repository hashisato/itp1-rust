fn main() {
    let (n, m, mut a, mut b) = input();
    for i in 0..n {
        let mut ans = 0;
        for j in 0..m {
            ans += a[i][j] * b[j];
        }
        println!("{}", ans);
    }
}

fn input() -> (usize, usize, Vec<Vec<isize>>, Vec<isize>) {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("Failed to read line!!");
    let mut iter = s.trim().split_whitespace();
    let n = iter.next().unwrap().parse().expect("Failed to parse");
    let m = iter.next().unwrap().parse().expect("Failed to parse");
    let mut a: Vec<Vec<isize>> = vec![];
    for _ in 0..n {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).expect("Failed to read line!!");
        let mut iter = s.trim().split_whitespace();
        let mut v = vec![];
        for i in 0..m {
            let element = iter.next().unwrap().parse().expect("Failed to parse");
             v.push(element);
        }
        a.push(v); 
    }
    let mut b: Vec<isize> = vec![];
    for _ in 0..m {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).expect("Failed to read line!!");
        let mut iter = s.trim().split_whitespace();
        let element = iter.next().unwrap().parse().expect("Failed to parse");
        b.push(element);
    }
    (n, m, a, b)
}