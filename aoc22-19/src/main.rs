use astar::*;
use std::fs;

type ResCost = [u32; 4];

#[derive(Clone, Debug)]
struct Blueprint {
    prodcost: [ResCost; 4],
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    t: u32,
    res: ResCost,
    robots: [u32; 4],
    fut_robots: [u32; 4],
}

impl Blueprint {
    fn from(l: &str) -> Blueprint {
        let w: Vec<_> = l.split_whitespace().collect();
        let ore = [w[6].parse().unwrap(), 0, 0, 0];
        let clay = [w[12].parse().unwrap(), 0, 0, 0];
        let obs = [w[18].parse().unwrap(), w[21].parse().unwrap(), 0, 0];
        let geode = [w[27].parse().unwrap(), 0, w[30].parse().unwrap(), 0];
        Blueprint {
            prodcost: [ore, clay, obs, geode],
        }
    }
}

impl State {
    fn new() -> State {
        State {
            t: 1,
            res: [0, 0, 0, 0],
            robots: [1, 0, 0, 0],
            fut_robots: [1, 0, 0, 0],
        }
    }

    fn advance(&mut self) {
        add(&mut self.res, &self.robots);
        self.robots = self.fut_robots;
        self.t += 1;
    }

    fn purchase(&mut self, rob_type: usize, bp: &Blueprint) -> bool {
        if less(&self.res, &bp.prodcost[rob_type]) {
            return false;
        }
        sub(&mut self.res, &bp.prodcost[rob_type]);
        self.fut_robots[rob_type] += 1;
        true
    }

    fn max_guess(&self, bp: &(Blueprint, u32)) -> u32 {
        let mut avail = [self.res.clone(); 4];
        let mut robs = self.robots.clone();
        for _ in self.t..=bp.1 {
            let mut can_buy = [true; 4];
            for rt in 0..4 {
                if less(&avail[rt], &bp.0.prodcost[rt]) {
                    can_buy[rt] = false;
                }
            }
            for a in &mut avail {
                add(a, &robs);
            }
            for rt in 0..4 {
                if can_buy[rt] {
                    robs[rt] += 1;
                    sub(&mut avail[rt], &bp.0.prodcost[rt]);
                }
            }
        }
        avail[3][3]
    }
}

fn less(lhs: &ResCost, rhs: &ResCost) -> bool {
    for (l, r) in lhs.iter().zip(rhs.iter()) {
        if *l < *r {
            return true;
        }
    }
    false
}

fn add(lhs: &mut ResCost, rhs: &ResCost) {
    for (l, r) in lhs.iter_mut().zip(rhs.iter()) {
        *l += r;
    }
}

fn sub(lhs: &mut ResCost, rhs: &ResCost) {
    for (l, r) in lhs.iter_mut().zip(rhs.iter()) {
        *l -= r;
    }
}

fn next_cand(st: &State, bp: &(Blueprint, u32), _c: &i32) -> Vec<Candidate<State, i32>> {
    
    let mut cand = Vec::new();
    for bt in 0..5 {
        let mut new_st = st.clone();
        if bt == 4 || new_st.purchase(bt, &bp.0) {
            new_st.advance();
            let new_cand = Candidate {
                cost: -(new_st.res[3] as i32),
                total_guess: -(new_st.max_guess(bp) as i32),
                cand: new_st,
            };
            cand.push(new_cand);
        }
    }
    cand
}

fn solve(bp: &Blueprint, iters: u32) -> i32 {
    let initial = State::new();
    let start = vec![Candidate::start(initial)];
    let goal = AStarGoal::Func(|st: &State, (_, days)| st.t > *days);
    let solver: AStarSolver<(Blueprint, u32), State, i32> =
        AStarSolver::new((bp.clone(), iters), start, next_cand, goal);
    let (res, ff) = solver.solve();
    -res.get_cost(ff.as_ref().unwrap()).unwrap()
}

fn main() {
    let blueprints: Vec<_> = fs::read_to_string("input.txt")
        .expect("Failed to read file.")
        .lines()
        .map(Blueprint::from)
        .collect();
    let mut result = 0;
    let mut result2 = 1;
    for (i, bp) in blueprints.iter().enumerate() {
        result += (i as i32 + 1) * solve(bp, 24);
        if i < 3 {
            result2 *= solve(&bp, 32);
        }
    }
    println!("{}", result);
    println!("{}", result2);
}
