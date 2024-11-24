use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).expect("Failed to read line!!");
    
    for c in b'a'..(b'z' + 1) {
        let mut ans: u32 = 0;
        for &x in s.as_bytes() {
            if c == x || c == (x ^ 32) {
                ans += 1;
            }
        }
        println!("{} : {}",c as char,ans);
    }
}