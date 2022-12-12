use astar::*;
use std::fs;

type AMap = Vec<Vec<u32>>;
type Pos = (usize, usize);

fn read_input() -> (AMap, Pos, Pos) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let map = fs::read_to_string("input.txt")
        .expect("Failed to read file.")
        .lines()
        .enumerate()
        .map(|(x, str)| {
            str.chars()
                .enumerate()
                .map(|(y, c)| match c {
                    'a'..='z' => c as u32 - 'a' as u32,
                    'S' => {
                        start = (x, y);
                        0
                    }
                    'E' => {
                        end = (x, y);
                        25
                    }
                    _ => panic!(),
                })
                .collect()
        })
        .collect();
    (map, start, end)
}

fn get_neighbors(p: &Pos, m: &AMap) -> Vec<Pos> {
    let (x, y) = *p;
    let mut neigh = vec![];
    for nx in [x+1, x.overflowing_sub(1).0] {
        if nx < m.len() {
            neigh.push((nx, y));
        }
    }
    for ny in [y+1, y.overflowing_sub(1).0] {
        if ny < m[x].len() {
            neigh.push((x, ny));
        }
    }
    neigh
}

fn get_candidates(p: &Pos, m: &(AMap, Pos), c: &u32) -> Vec<Candidate<Pos, u32>> {
    let max_h = m.0[p.0][p.1] + 1;
    get_neighbors(p, &m.0)
        .iter()
        .filter(|(x, y)| m.0[*x][*y] <= max_h)
        .map(|&(x, y)| Candidate {
            cand: (x, y),
            cost: c + 1,
            total_guess: c + 1 + (x.abs_diff(m.1 .0) + y.abs_diff(m.1 .1)) as u32,
        })
        .collect()
}

fn get_candidates2(p: &Pos, m: &AMap, c: &u32) -> Vec<Candidate<Pos, u32>> {
    let h = m[p.0][p.1];
    get_neighbors(p, m)
        .iter()
        .filter(|(x, y)| m[*x][*y] + 1 >= h)
        .map(|&(x, y)| Candidate {
            cand: (x, y),
            cost: c + 1,
            total_guess: c + 1 + m[x][y] as u32,
        })
        .collect()
}

fn main() {
    let (map, start, end) = read_input();

    let solver = AStarSolver::new(
        (map.clone(), end.clone()),
        vec![Candidate::start(start)],
        get_candidates,
        AStarGoal::Fixed(end.clone()),
    );
    let (res, _ff) = solver.solve();
    println!("Start to End: {}", res.get_cost(&end).unwrap());

    let solver = AStarSolver::new(
        map,
        vec![Candidate::start(end)],
        get_candidates2,
        AStarGoal::Func(|pos, map| map[pos.0][pos.1] == 0),
    );
    let (res, ff) = solver.solve();
    println!("End to Plains: {}", res.get_cost(&ff.unwrap()).unwrap());
}
