use std::fs;

const CHAR_TABLE: [char; 5] = ['0', '1', '2', '=', '-'];
const REM_TABLE: [i64; 5] = [0, 0, 0, 1, 1];

fn snafu_to_int(c: char) -> i64 {
    let d = CHAR_TABLE.iter().position(|&e| e == c).unwrap();
    d as i64 - 5 * REM_TABLE[d]
}

fn read_snafu(s: &str) -> i64 {
    s.chars().fold(0, |a, c| 5 * a + snafu_to_int(c))
}

fn to_snafu(mut nb: i64) -> String {
    let mut s = String::new();
    while nb != 0 {
        let rem = nb as usize % 5;
        s = format!("{}{}", CHAR_TABLE[rem], s);
        nb = nb / 5 + REM_TABLE[rem];
    }
    s
}

fn main() {
    let result = fs::read_to_string("input.txt")
        .expect("Failed to read file.")
        .lines()
        .map(read_snafu)
        .sum();
    println!("dec: {} -> snafu: {}", result, to_snafu(result));
}
