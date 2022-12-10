use std::fs;

fn read_op(s: &str) -> (i32, Option<i32>) {
    let w: Vec<_> = s.split_whitespace().collect();
    match w[0] {
        "addx" => (2, Some(w[1].parse::<i32>().unwrap())),
        "noop" => (1, None),
        _ => panic!(),
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut lines = input.lines();
    let mut cycle = 0;
    let mut reg = 1;
    let mut cd = 1;
    let mut rem_add = None;
    let mut result1 = 0;
    let mut result2 = String::new();
    loop {
        cycle += 1;
        cd -= 1;
        if cd == 0 {
            if let Some(v) = rem_add {
                reg += v;
            }
            match lines.next() {
                None => break,
                Some(l) => (cd, rem_add) = read_op(l),
            }
        };
        let cross = (reg - ((cycle - 1) % 40)).abs() <= 1;
        result2.push(if cross { '\u{2b1b}' } else { '\u{2b1c}' });
        match cycle {
            c if c > 240 => break,
            c if c % 40 == 20 => result1 += cycle * reg,
            c if c % 40 == 0 => result2.push('\n'),
            _ => (),
        }
    }
    println!("{}", result1);
    println!("{}", result2);
}
