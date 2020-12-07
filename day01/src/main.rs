use itertools::Itertools;

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Couldn't read file");

    let nums: Vec<i32> = contents.split("\n")
            .map(|s| s.trim())
            .filter(|s| s.len() > 0)
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
    
    let p1: Vec<i32> = nums.iter().combinations(3)
        .filter(|v| v[0] + v[1] == 2020)
        .map(|v| v[0] * v[1])
        .collect();
    println!("Part 1 = {:?}", p1);

    let p2: Vec<i32> = nums.iter().combinations(3)
        .filter(|v| v[0] + v[1] + v[2] == 2020)
        .map(|v| v[0] * v[1] * v[2])
        .collect();
    println!("Part 2 = {:?}", p2);
}
