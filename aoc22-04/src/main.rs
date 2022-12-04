use std::fs;

fn read_limits(s: &str) -> (u32, u32) {
    let l: Vec<_> = s.split('-').collect();
    (l[0].parse::<u32>().unwrap(), l[1].parse::<u32>().unwrap())
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut score1 = 0;
    let mut score2 = 0;
    for l in contents.lines() {
        let words: Vec<_> = l.split(',').collect();
        let (e1_lower, e1_upper) = read_limits(&words[0]);
        let (e2_lower, e2_upper) = read_limits(&words[1]);
        if e1_lower >= e2_lower && e1_upper <= e2_upper
            || e1_lower <= e2_lower && e1_upper >= e2_upper
        {
            score1 += 1;
        }
        if e1_lower <= e2_upper && e1_upper >= e2_lower
            || e2_lower <= e1_upper && e2_upper >= e1_lower
        {
            score2 += 1;
        }
    }
    println!("score1: {}", score1);
    println!("score2: {}", score2);
}
