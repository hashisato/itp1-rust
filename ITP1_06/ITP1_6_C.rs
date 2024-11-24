fn main() {
    let mut data = input();
    let mut checker: Vec<Vec<Vec<isize>>> = vec![];
    for _ in 0..4 {
        let mut v1 = vec![];
        for _ in 0..3 {
            let mut v2 = vec![];
            for _ in 0..10 {
                v2.push(0);
            }
            v1.push(v2)
        }
        checker.push(v1);
    }
    for (b, f, r, v) in data {
        checker[b-1][f-1][r-1] += v;
    }
    for b in 0..4 {
        for i in 0..3 {
            for j in 0..10 {
                print!(" {}",checker[b][i][j]);
            }
            println!();
        }
        if b != 3 {
            for _ in 0..20 {
                print!("#");
            }
            println!();
        }
    }
}

fn input() -> Vec<(usize, usize, usize, isize)> {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("Failed to read line!!");
    let n: u32 = n.trim().parse().unwrap();

    let mut data: Vec<(usize, usize, usize, isize)> = vec![];
    for _ in 0..n {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).expect("Failed to read line!!");
        let mut iter = s.trim().split_whitespace();
        let mut array: [isize; 4] = [0,0,0,0];
        for i in 0..4 {
            array[i] = iter.next().unwrap().parse().expect("Failed to parse");
        }
        data.push((array[0] as usize, array[1] as usize, array[2] as usize, array[3])); 
    }
    data
}