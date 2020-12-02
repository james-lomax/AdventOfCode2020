fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Couldn't read file");

    let nums: Vec<i32> = contents.split("\n")
            .map(|s| s.trim())
            .filter(|s| s.len() > 0)
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
    
    for x in &nums {
        for y in &nums {
            for z in &nums {
                if x + y + z == 2020 {
                    println!("{}", x*y*z);
                    return;
                }
            }
        }
    }
}
