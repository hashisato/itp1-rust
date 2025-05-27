fn main() {
    let mut card = input();
    let mut checker: Vec<Vec<bool>> = vec![];
    for _ in 0..4 {
        let mut v = vec![];
        for _ in 0..13 {
            v.push(false);
        }
        checker.push(v);
    }
    for mark in card {
        match mark {
            ('S', n) => checker[0][n-1] = true,
            ('H', n) => checker[1][n-1] = true,
            ('C', n) => checker[2][n-1] = true,
            (_, n) => checker[3][n-1] = true,
        }
    }
    for i in 0..4 {
        for j in 0..13 {
            if checker[i][j] == false {
                let mark = match i {
                    0 => 'S',
                    1 => 'H',
                    2 => 'C',
                    _ => 'D',
                };
                println!("{} {}",mark,j+1);
            }
        }
    }
}

fn input() -> Vec<(char, usize)> {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("Failed to read line!!");
    let n: u32 = n.trim().parse().unwrap();

    let mut card: Vec<(char, usize)> = vec![];
    for __ in 0..n {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).expect("Failed to read line!!");
        let mut iter = s.trim().split_whitespace();

        let mark: char = iter.next().unwrap().chars().nth(0).unwrap();
        let num: usize = iter.next().unwrap().parse().unwrap();

        card.push((mark, num));
    }
    card
}