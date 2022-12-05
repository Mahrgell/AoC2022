use std::{fs, str::Chars, str::Lines};

fn read_crate(chars: &mut Chars) -> Option<char> {
    let c = chars.next().unwrap();
    let result = if c == '[' {
        Some(chars.next().unwrap())
    } else {
        chars.next();
        None
    };
    chars.next();
    chars.next();
    result
}

fn read_stacks(lines: &mut Lines, nb_stacks: usize) -> Vec<Vec<char>> {
    let mut result = vec![Vec::new(); nb_stacks];
    loop {
        let l = lines.next().unwrap();
        if l.is_empty() {
            break;
        }
        let mut chars = l.chars();
        for i in 0..nb_stacks {
            if let Some(c) = read_crate(&mut chars) {
                result[i].push(c);
            }
        }
    }
    for s in &mut result {
        s.reverse();
    }
    result
}

fn stack_around(stacks: Vec<Vec<char>>, instructions: Lines, keep_order: bool) -> String {
    let mut stacks = stacks;
    for l in instructions {
        let words: Vec<_> = l.split_whitespace().collect();
        let nb = words[1].parse::<usize>().unwrap();
        let from = words[3].parse::<usize>().unwrap() - 1;
        let to = words[5].parse::<usize>().unwrap() - 1;
        let mut to_be_moved = Vec::new();
        for _ in 0..nb {
            to_be_moved.push(stacks[from].pop().unwrap());
        }
        if keep_order {
            to_be_moved.reverse();
        }
        for c in to_be_moved {
            stacks[to].push(c);
        }
    }
    let mut result = String::new();
    for s in stacks {
        result.push(*s.last().unwrap());
    }
    result
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut lines = contents.lines();
    let stacks = read_stacks(&mut lines, 9);
    println!(
        "CrateMover 9000: {}",
        stack_around(stacks.clone(), lines.clone(), false)
    );
    println!("CrateMover 9001: {}", stack_around(stacks, lines, true));
}
