use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut max = [0; 4];
    for l in contents.lines() {
        if let Ok(v) = l.parse::<u32>() {
            max[0] += v;
        } else {
            max.sort_unstable();
            max[0] = 0;
        }
    }
    println!("max: {}", max[3]);
    println!("max3 sum: {}", max[1] + max[2] + max[3]);
}
