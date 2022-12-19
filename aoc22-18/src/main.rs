use std::{collections::HashSet, fs};

fn neighbors(b: &(i32, i32, i32)) -> [(i32, i32, i32); 6] {
    [
        (b.0 + 1, b.1, b.2),
        (b.0 - 1, b.1, b.2),
        (b.0, b.1 + 1, b.2),
        (b.0, b.1 - 1, b.2),
        (b.0, b.1, b.2 + 1),
        (b.0, b.1, b.2 - 1),
    ]
}

fn is_air(air: &Vec<Vec<Vec<bool>>>, b: &(i32, i32, i32)) -> bool {
    b.0 < 0
        || b.1 < 0
        || b.2 < 0
        || b.0 as usize >= air.len()
        || b.1 as usize >= air[b.0 as usize].len()
        || b.2 as usize >= air[b.0 as usize][b.1 as usize].len()
        || air[b.0 as usize][b.1 as usize][b.2 as usize]
}

fn main() {
    let blocks: HashSet<_> = fs::read_to_string("input.txt")
        .expect("Failed to read file.")
        .lines()
        .map(|l| {
            let w: Vec<_> = l.split(',').collect();
            (
                w[0].parse::<i32>().unwrap(),
                w[1].parse::<i32>().unwrap(),
                w[2].parse::<i32>().unwrap(),
            )
        })
        .collect();
    let max_dim = blocks
        .iter()
        .fold((0, 0, 0), |a, b| (a.0.max(b.0), a.1.max(b.1), a.2.max(b.2)));
    let mut air = vec![
        vec![vec![false; max_dim.2 as usize + 1]; max_dim.1 as usize + 1];
        max_dim.0 as usize + 1
    ];
    loop {
        let mut changed = false;
        for x in 0..=max_dim.0 {
            for y in 0..=max_dim.1 {
                for z in 0..=max_dim.2 {
                    if air[x as usize][y as usize][z as usize] || blocks.contains(&(x, y, z)) {
                        continue;
                    }
                    for n in neighbors(&(x, y, z)) {
                        if is_air(&air, &n) {
                            air[x as usize][y as usize][z as usize] = true;
                            changed = true;
                        }
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }
    let open_faces = blocks.iter().fold(0, |a, b| {
        a + neighbors(b)
            .iter()
            .fold(0, |a, n| if blocks.contains(&n) { a } else { a + 1 })
    });
    let air_faces = blocks.iter().fold(0, |a, b| {
        a + neighbors(b).iter().fold(0, |a, n| {
            if blocks.contains(&n) || !is_air(&air, n) {
                a
            } else {
                a + 1
            }
        })
    });
    println!("open faces: {}", open_faces);
    println!("air faces: {}", air_faces);
}
