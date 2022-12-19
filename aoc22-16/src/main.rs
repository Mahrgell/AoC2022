use astar::*;
use std::{collections::HashMap, fs};
use std::hash::{Hash, Hasher};

type ID = usize;
type Time = u32;

struct StaticData {
    top_valves: Vec<(ID, u32)>,
    valves: Vec<Valve>,
    turn_limit: Time,
}

#[derive(Clone)]
struct Valve {
    id: ID,
    flow_rate: u32,
    neighbors: Vec<(ID, u32)>,
}

#[derive(Clone, Debug, Eq)]
struct State {
    t: Time,
    pos: [ID; 2],
    ready_in: [Time; 2],
    opened: Vec<bool>,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.pos.eq(&other.pos)
            && self.opened.eq(&other.opened)
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
        self.opened.hash(state);
    }
}

impl Valve {
    fn from(s: &str) -> Valve {
        let w: Vec<_> = s.split_whitespace().collect();
        let id = to_id(w[1]);
        let wfr: Vec<_> = w[4].split('=').collect();
        let wfr = wfr[1].trim_matches(|c| c == ';');
        let flow_rate = wfr.parse().unwrap();
        let mut neighbors = Vec::new();
        for i in 9..w.len() {
            neighbors.push((to_id(w[i].trim_end_matches(|c| c == ',')), 1));
        }
        Valve {
            id,
            flow_rate,
            neighbors,
        }
    }
}

fn to_id(s: &str) -> ID {
    let mut c = s.chars();
    (c.next().unwrap() as ID - 'A' as ID + 1) * 100 + (c.next().unwrap() as ID - 'A' as ID + 1)
}

fn guess_remaining(st: &State, sd: &StaticData) -> u32 {
    let mut rem = 0;
    let mut rem_t = [st.t + st.ready_in[0], st.t + st.ready_in[1]];
    rem_t.sort();
    for (id, fr) in &sd.top_valves {
        if *fr == 0 || rem_t[0] > sd.turn_limit {
            break;
        }
        if st.opened[*id] {
            continue;
        }
        rem += (sd.turn_limit - rem_t[0]) * fr;
        rem_t[0] += 2;
        rem_t.sort();
    }
    rem
}

fn advance(st: &mut State) {
    let time_adv = st.ready_in[0].min(st.ready_in[1]);
    st.t += time_adv;
    st.ready_in[0] -= time_adv;
    st.ready_in[1] -= time_adv;
}

static mut MAX: u32 = 0;
static mut ITERS: u64 = 0;

fn next_candidates(st: &State, sd: &StaticData, c: &i32) -> Vec<Candidate<State, i32>> {
    unsafe {
        ITERS += 1;
        if st.t > MAX {
            MAX = st.t;
            println!("{}: {}", MAX, ITERS);
        }
    }
    let mut cand = Vec::new();
    let active = if st.ready_in[0] == 0 { 0 } else { 1 };
    let v = &sd.valves[st.pos[active]];
    if v.flow_rate != 0 && !st.opened[st.pos[active]] {
        let mut new_st = st.clone();
        new_st.opened[st.pos[active]] = true;
        new_st.ready_in[active] = 1;
        advance(&mut new_st);
        let cost = c - (sd.turn_limit - st.t) as i32 * v.flow_rate as i32;
        let total_guess = cost - guess_remaining(&new_st, sd) as i32;
        let cn = Candidate {
            cost,
            cand: new_st,
            total_guess,
        };
        cand.push(cn);
    } else {
        let rem = guess_remaining(&st, sd);
        if rem > 0 {
            for (n, dt) in &v.neighbors {
                if st.opened[*n] {
                    continue;
                }
                if st.pos[1-active] == *n && st.ready_in[1-active] <= *dt{
                    continue;
                }
                let mut new_st = st.clone();
                new_st.pos[active] = *n;
                new_st.ready_in[active] = *dt;
                if new_st.pos[1] < new_st.pos[0] {
                    new_st.pos.swap(0, 1);
                    new_st.ready_in.swap(0, 1);
                }
                advance(&mut new_st);
                let total_guess = c - guess_remaining(&new_st, sd) as i32;
                let cn = Candidate {
                    cost: *c,
                    cand: new_st,
                    total_guess,
                };
                cand.push(cn);
            }
        } else {
            let new_st = State {
                t: sd.turn_limit,
                ..st.clone()
            };
            let cn = Candidate {
                cost: *c,
                total_guess: *c,
                cand: new_st,
            };
            cand.push(cn);
        }
    }
    cand
}

fn simplify(mut valves: HashMap<ID, Valve>) -> (Vec<Valve>, usize) {
    let aa = to_id("AA");
    let to_remove: Vec<_> = valves
        .iter()
        .filter(|&(_, v)| v.id != aa && v.flow_rate == 0 && v.neighbors.len() == 2)
        .map(|(id, _)| *id)
        .collect();
    for id in &to_remove {
        let neighbors = &valves[&id].neighbors.clone();
        let dist = neighbors[0].1 + neighbors[1].1;
        for n in [0, 1] {
            let from = neighbors[n].0;
            let to = neighbors[1 - n].0;
            let p = valves[&from]
                .neighbors
                .iter()
                .position(|(nn, _d)| nn == id)
                .unwrap();
            valves.get_mut(&from).unwrap().neighbors[p] = (to, dist);
        }
    }
    for id in to_remove {
        valves.remove(&id);
    }
    let mut old_to_new = HashMap::new();
    for (i, (_id, v)) in valves.iter().enumerate() {
        old_to_new.insert(v.id, i);
    }
    for (_id, v) in &mut valves {
        v.id = old_to_new[&v.id];
        for (n, _c) in &mut v.neighbors {
            *n = old_to_new[&n];
        }
    }
    let mut simplified: Vec<_> = valves.into_iter().map(|(_, v)| v).collect();
    simplified.sort_by(|a, b| a.id.cmp(&b.id));
    (simplified, old_to_new[&aa])
}

fn complete_distances(valves: &mut Vec<Valve>) {
    let sz = valves.len();
    let mut dm = vec![vec![0; sz]; sz];
    let mut todo = Vec::new();
    for from in 0..sz {
        for (n, d) in &valves[from].neighbors {
            if from < *n {
                todo.push((from, *n, *d));
            }
        }
    }
    while !todo.is_empty() {
        todo.sort_by(|a, b| b.2.cmp(&a.2));
        let (from, to, d) = todo.pop().unwrap();
        if dm[from][to] != 0 {
            continue;
        }
        dm[from][to] = d;
        dm[to][from] = d;
        for (i1, i2) in [(from, to), (to, from)] {
            for i3 in 0..sz {
                if i3 == i1 || dm[i1][i3] != 0 || dm[i2][i3] == 0 {
                    continue;
                }
                todo.push((i1, i3, d + dm[i2][i3]));
            }
        }
    }
    for (i, v) in valves.iter_mut().enumerate() {
        v.neighbors.clear();
        for (n, d) in dm[i].iter().enumerate() {
            if i != n {
                v.neighbors.push((n, *d));
            }
        }
    }
}

fn solve(valves: Vec<Valve>, start: ID, turn_limit: u32, elephant: bool) -> i32 {
    unsafe {
        MAX = 0;
        ITERS = 0;
    }
    let start = Candidate::start(State {
        pos: [start; 2],
        ready_in: [0, if elephant { 0 } else { turn_limit }],
        t: 1,
        opened: vec![false; valves.len()],
    });
    let mut top_valves: Vec<_> = valves.iter().map(|v| (v.id, v.flow_rate)).collect();
    top_valves.sort_by(|a, b| b.1.cmp(&a.1));

    let sd = StaticData {
        top_valves,
        valves,
        turn_limit,
    };

    let goal = AStarGoal::Func(|st: &State, sd: &StaticData| st.t >= sd.turn_limit);
    let astar = AStarSolver::new(sd, vec![start], next_candidates, goal);
    let (res, last_st) = astar.solve();
    -res.get_cost(&last_st.unwrap()).unwrap()
}

fn main() {
    let valves: HashMap<_, _> = fs::read_to_string("input.txt")
        .expect("Failed to read file.")
        .lines()
        .map(|s| {
            let v = Valve::from(s);
            (v.id, v)
        })
        .collect();
    let (mut valves, aa) = simplify(valves);
    complete_distances(&mut valves);

    dbg!(solve(valves.clone(), aa, 30, false));
    dbg!(solve(valves, aa, 26, true));
}
