fn main() {
    for line in std::io::stdin().lines() {
        let input = line.unwrap();
        let nums: Vec<i32> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
        if nums[0] == 0 && nums[1] == 0 { break; }
        else{
            if nums[0] <= nums[1] { println!("{} {}",nums[0],nums[1])}
            else { println!("{} {}",nums[1],nums[0])}
        }
    }
}
