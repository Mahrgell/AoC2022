use std::collections::HashSet;
use std::fs;

type Pos = (i32, i32);
type Range = (i32, i32);
type Coverage = Vec<Range>;

#[derive(Debug)]
struct Sensor {
    c: Pos,
    b: Pos,
    r: i32,
}

impl Sensor {
    fn from(s: &str) -> Sensor {
        let w: Vec<_> = s.split_whitespace().collect();
        let c = (get_nb(w[2]), get_nb(w[3]));
        let b = (get_nb(w[8]), get_nb(w[9]));
        let r = (c.0 - b.0).abs() + (c.1 - b.1).abs();
        Sensor { c, b, r }
    }

    fn add_to_coverage(&self, row: i32, cov: &mut Coverage) {
        let rr = self.r - (self.c.1 - row).abs();
        if rr >= 0 {
            add_range_to_coverage((self.c.0 - rr, self.c.0 + rr), cov);
        }
    }
}

fn merge_ranges(r1: Range, r2: Range) -> Option<Range> {
    if r1.0 >= r2.0 && r1.1 <= r2.1 {
        return Some(r2);
    }
    if r2.0 >= r1.0 && r2.1 <= r1.1 {
        return Some(r1);
    }
    if r1.0 <= r2.1 + 1 && r1.1 > r2.1 {
        return Some((r2.0, r1.1));
    }
    if r2.0 <= r1.1 + 1 && r2.1 > r1.1 {
        return Some((r1.0, r2.1));
    }
    None
}

fn add_range_to_coverage(mut r: Range, cov: &mut Coverage) {
    let mut ind_to_remove = Vec::new();
    for (i, cr) in cov.iter().enumerate() {
        if let Some(new_r) = merge_ranges(r, *cr) {
            ind_to_remove.push(i);
            r = new_r;
        }
    }
    for i in ind_to_remove.iter().rev() {
        cov.remove(*i);
    }
    cov.push(r);
}

fn get_nb(s: &str) -> i32 {
    let w: Vec<_> = s.split(|c| ['=', ',', ':'].contains(&c)).collect();
    w[1].parse().unwrap()
}

fn main() {
    let sensors: Vec<_> = fs::read_to_string("input.txt")
        .expect("Failed to read file.")
        .lines()
        .map(|s| Sensor::from(s))
        .collect();
    let mut cov = Vec::new();
    let row = 2000000;
    let mut beacons = HashSet::new();
    for s in &sensors {
        s.add_to_coverage(row, &mut cov);
        if s.b.1 == row {
            beacons.insert(s.b.0);
        }
    }
    let blocked = cov.iter().fold(0, |a, c| a + c.1 - c.0 + 1);
    println!("Blocked spaces: {}", blocked - beacons.len() as i32);
    for i in 0..=4000000 {
        let mut cov = Vec::new();
        for s in &sensors {
            s.add_to_coverage(i, &mut cov);
        }
        if cov.len() == 2 {
            cov.sort();
            let col = cov[0].1 as i64 +1;
            println!("{}, {} -> {}", i, col, i as i64+4000000*col);
            break;
        }
    }
}
