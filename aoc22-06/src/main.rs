use std::{collections::VecDeque, fs};

fn find_first_n_not_repeating(s: &str, n: usize) -> usize {
    let mut last_elems = VecDeque::with_capacity(n - 1);
    for _ in 0..n - 1 {
        last_elems.push_back('\u{1F648}');
    }
    let mut blocked = n;
    for (pos, c) in s.chars().enumerate() {
        let p = last_elems.iter().rev().position(|&l| l == c);
        blocked -= 1;
        if let Some(p) = p {
            blocked = blocked.max(n - 1 - p);
        } else if blocked == 0 {
            return pos + 1;
        }
        last_elems.pop_front();
        last_elems.push_back(c);
    }
    panic!();
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    println!(
        "First 4 non repeating: {}",
        find_first_n_not_repeating(&contents, 4)
    );
    println!(
        "First 14 non repeating: {}",
        find_first_n_not_repeating(&contents, 14)
    );
}
