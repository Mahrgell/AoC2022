use std::{collections::HashSet, fs};

type Pos = (i32, i32);
type Dir = u32;
type Elves = HashSet<Pos>;

fn has_neighbor(elves: &Elves, p: Pos, from: Pos, to: Pos) -> bool {
    for x in p.0 + from.0..=p.0 + to.0 {
        for y in p.1 + from.1..=p.1 + to.1 {
            if p != (x, y) && elves.contains(&(x, y)) {
                return true;
            }
        }
    }
    false
}

fn get_move(elves: &Elves, p: Pos, start_dir: Dir) -> Option<Pos> {
    for i in 0..4 {
        let (from, to) = match (start_dir + i) % 4 {
            0 => ((-1, -1), (1, -1)),
            1 => ((-1, 1), (1, 1)),
            2 => ((-1, -1), (-1, 1)),
            3 => ((1, -1), (1, 1)),
            _ => panic!(),
        };
        if !has_neighbor(elves, p, from, to) {
            return Some((p.0 + (from.0 + to.0) / 2, p.1 + (from.1 + to.1) / 2));
        }
    }
    None
}

fn update_elves(elves: &mut Elves, start_dir: Dir) -> bool {
    let mut move_happened = false;
    let mut moving_elves = Vec::new();
    for p in &*elves {
        if !has_neighbor(elves, *p, (-1, -1), (1, 1)) {
            continue;
        }
        if let Some(next_pos) = get_move(elves, *p, start_dir) {
            moving_elves.push((*p, next_pos));
        }
    }
    for (old, new) in &moving_elves {
        if moving_elves.iter().any(|(o, n)| o != old && n == new) {
            continue;
        }
        move_happened = true;
        elves.remove(old);
        elves.insert(*new);
    }
    move_happened
}

fn read_input() -> Elves {
    let input = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut elves: Elves = HashSet::new();
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '#' {
                elves.insert((x as i32, y as i32));
            }
        }
    }
    elves
}

fn main() {
    let mut elves = read_input();
    let mut r = 0;
    while update_elves(&mut elves, r % 4) {
        r += 1;
        if r == 10 {
            let min_x = elves.iter().map(|(x, _)| x).min().unwrap();
            let max_x = elves.iter().map(|(x, _)| x).max().unwrap();
            let min_y = elves.iter().map(|(_, y)| y).min().unwrap();
            let max_y = elves.iter().map(|(_, y)| y).max().unwrap();
            let empty = (1 + max_x - min_x) * (1 + max_y - min_y) - elves.len() as i32;
            println!("{} empty spaces.", empty);
        }
    }
    println!("First round without movement: {}", r + 1)
}
