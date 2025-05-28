fn main() {
    let (mut n, mut m, mut l, mut a, mut b) = input();
    let mut c: Vec<Vec<usize>> = vec![];
    for _ in 0..n {
        let mut v = vec![];
        for _ in 0..l {
            v.push(0);
        }
        c.push(v);
    }
    let c = calc(n, m, l, a, b, c);
    for i in 0..n {
        for j in 0..l {
            if j != l-1 { print!("{} ",c[i][j]); }
            else { println!("{}",c[i][j]); }
        }
    }
}

fn input() -> (usize, usize, usize, Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("Failed to read line!!");
    let mut iter = s.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Failed to parse");
    let m: usize = iter.next().unwrap().parse().expect("Failed to parse");
    let l: usize = iter.next().unwrap().parse().expect("Failed to parse");
    let mut a: Vec<Vec<usize>> = vec![];
    for _ in 0..n {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).expect("Failed to read line!!");
        let mut iter = s.trim().split_whitespace();
        let mut v = vec![];
        for _ in 0..m {
            let element: usize = iter.next().unwrap().parse().expect("Failed to parse");
            v.push(element);
        }
        a.push(v);
    }
    let mut b: Vec<Vec<usize>> = vec![];
    for _ in 0..m {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).expect("Failed to read line!!");
        let mut iter = s.trim().split_whitespace();
        let mut v = vec![];
        for _ in 0..l {
            let element: usize = iter.next().unwrap().parse().expect("Failed to parse");
            v.push(element);
        }
        b.push(v);
    }
    (n, m, l, a, b)
}

fn calc(n: usize, m: usize, l: usize, a: Vec<Vec<usize>>, b: Vec<Vec<usize>>, mut c: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    for i in 0..n {
        for j in 0..m {
            for k in 0..l {
                c[i][k] += a[i][j] * b[j][k];
            }
        }
    }
    c
}
