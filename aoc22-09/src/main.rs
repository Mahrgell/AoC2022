use std::{collections::HashSet, fs};

fn follow(h: (i32, i32), t: &mut (i32, i32)) {
    let dx = t.0 - h.0;
    let dy = t.1 - h.1;
    if dx.abs() + dy.abs() > 2 {
        t.0 = h.0 + dx / 2;
        t.1 = h.1 + dy / 2;
    } else if dx.abs() == 2 {
        t.0 = h.0 + dx / 2;
    } else if dy.abs() == 2 {
        t.1 = h.1 + dy / 2;
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read file.");

    let mut snake = [(0i32, 0i32); 10];
    let mut vis_short = HashSet::new();
    let mut vis_long = HashSet::new();
    vis_short.insert(snake[1].clone());
    vis_long.insert(snake[9].clone());
    for l in input.lines() {
        let w: Vec<_> = l.split_whitespace().collect();
        let dir = match w[0] {
            "U" => (0, 1),
            "R" => (1, 0),
            "D" => (0, -1),
            "L" => (-1, 0),
            _ => panic!(),
        };
        let steps = w[1].parse::<u32>().unwrap();
        for _ in 0..steps {
            snake[0].0 += dir.0;
            snake[0].1 += dir.1;
            for i in 1..10 {
                follow(snake[i - 1], &mut snake[i]);
            }
            vis_short.insert(snake[1].clone());
            vis_long.insert(snake[9].clone());
        }
    }
    println!("Short tail visited: {}", vis_short.len());
    println!("Long tail visited: {}", vis_long.len());
}
