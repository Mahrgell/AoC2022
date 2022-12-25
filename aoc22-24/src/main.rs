use astar::*;
use std::{fs, mem::swap};

#[derive(Clone)]
struct BlizzMap {
    blizzards: Vec<((i32, i32), (i32, i32))>,
    goal: (i32, i32),
    dim: (i32, i32),
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct State {
    pos: (i32, i32),
    t: i32,
}

impl BlizzMap {
    fn read() -> BlizzMap {
        let mut blizzards = Vec::new();
        let input = fs::read_to_string("input.txt").expect("Failed to read file.");
        for (x, l) in input.lines().enumerate() {
            for (y, c) in l.chars().enumerate() {
                let blizz_dir = match c {
                    '<' => (0, -1),
                    'v' => (1, 0),
                    '^' => (-1, 0),
                    '>' => (0, 1),
                    _ => continue,
                };
                blizzards.push(((x as i32 - 1, y as i32 - 1), blizz_dir));
            }
        }
        let max_x = input.lines().count() as i32 - 1;
        let max_y = input.lines().next().unwrap().chars().count() as i32 - 1;
        let goal = (max_x - 1, max_y - 2);
        let dim = (max_x - 1, max_y - 1);
        BlizzMap {
            blizzards,
            goal,
            dim,
        }
    }
}

fn neighbors(p: (i32, i32)) -> Vec<(i32, i32)> {
    vec![
        (p.0, p.1),
        (p.0 - 1, p.1),
        (p.0 + 1, p.1),
        (p.0, p.1 - 1),
        (p.0, p.1 + 1),
    ]
}

fn build_cand(bm: &BlizzMap, t: i32, pos: (i32, i32)) -> Candidate<State, i32> {
    let rem_dist = (bm.goal.0 - pos.0).abs() + (bm.goal.1 - pos.1).abs();
    Candidate {
        cand: State { t, pos },
        cost: t,
        total_guess: t + rem_dist,
    }
}

fn next_cand(st: &State, bm: &BlizzMap, _c: &i32) -> Vec<Candidate<State, i32>> {
    let t = st.t + 1;
    let neighbors = neighbors(st.pos);
    if neighbors.contains(&bm.goal) {
        return vec![build_cand(bm, t, bm.goal)];
    }
    let blizz_pos: Vec<_> = bm
        .blizzards
        .iter()
        .map(|(p, d)| {
            (
                (p.0 + t * d.0).rem_euclid(bm.dim.0),
                (p.1 + t * d.1).rem_euclid(bm.dim.1),
            )
        })
        .collect();
    neighbors
        .iter()
        .filter(|&n| *n == st.pos || (n.0 >= 0 && n.0 < bm.dim.0 && n.1 >= 0 && n.1 < bm.dim.1))
        .filter(|&n| !blizz_pos.contains(n))
        .map(|&n| build_cand(bm, t, n))
        .collect()
}

fn run(mut bm: BlizzMap, t_start: i32, rev: bool) -> i32 {
    let goal = AStarGoal::Func(|st: &State, bm: &BlizzMap| bm.goal == st.pos);
    let mut start = (-1, 0);
    if rev {
        swap(&mut start, &mut bm.goal);
    }
    let initial = build_cand(&bm, t_start, start);

    let solver = AStarSolver::new(bm, vec![initial], next_cand, goal);
    let (res, finalf) = solver.solve();
    res.get_cost(&finalf.unwrap()).unwrap()
}

fn main() {
    let blizzards = BlizzMap::read();
    let t1 = run(blizzards.clone(), 0, false);
    println!("There: {:?}", t1);

    let t2 = run(blizzards.clone(), t1, true);
    println!("And back...: {:?}", t2);

    let t3 = run(blizzards, t2, false);
    println!("And there again: {:?}", t3);
}
