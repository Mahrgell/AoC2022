use std::collections::HashSet;
use std::fs;

type Pos = (u32, u32);
type Cave = HashSet<Pos>;

fn to_coord(s: &str) -> Pos {
    let w: Vec<_> = s.split(',').collect();
    (w[0].parse().unwrap(), w[1].parse().unwrap())
}

fn add_line_to_cave(cave: &mut Cave, l: &str) {
    let w: Vec<_> = l.split_whitespace().collect();
    let mut start = to_coord(w[0]);
    for i in (2..w.len()).step_by(2) {
        let end = to_coord(w[i]);
        let from = (start.0.min(end.0), start.1.min(end.1));
        let to = (start.0.max(end.0), start.1.max(end.1));
        for x in from.0..=to.0 {
            for y in from.1..=to.1 {
                cave.insert((x, y));
            }
        }
        start = end;
    }
}

fn read_input() -> Cave {
    let mut cave = HashSet::new();
    for l in fs::read_to_string("input.txt")
        .expect("Failed to read file.")
        .lines()
    {
        add_line_to_cave(&mut cave, l);
    }
    cave
}

fn drop(cave: &Cave, p: Pos, highest: u32) -> Pos {
    if p.1 > highest {
        return p;
    }
    for nx in [p.0, p.0 - 1, p.0 + 1] {
        if !cave.contains(&(nx, p.1 + 1)) {
            return drop(cave, (nx, p.1 + 1), highest);
        }
    }
    return p;
}

fn main() {
    let mut cave = read_input();
    let highest = cave.iter().fold(0, |a, (_, y)| a.max(*y));
    let origin = (500, 0);
    let mut count = 0;
    let mut to_floor = None;
    loop {
        let p = drop(&cave, origin, highest);
        if to_floor.is_none() && p.1 == highest + 1 {
            to_floor = Some(count);
        }
        count += 1;
        if p == origin {
            break;
        }
        cave.insert(p);
    }
    println!("{} sand dropped without floor.", to_floor.unwrap());
    println!("{} sand dropped with floor.", count);
}
