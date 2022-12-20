use std::fs;

type Data = Vec<(i64, usize, usize)>;

fn step(d: &Data, i: usize, steps: i64, jump_yourself: bool) -> usize {
    let mut nb_steps = steps.abs();
    if jump_yourself {
        nb_steps %= d.len() as i64 - 1;
    }
    let mut ii = i;
    if steps < 0 {
        for _ in 0..nb_steps {
            ii = d[ii].1;
        }
    } else {
        for _ in 0..nb_steps {
            ii = d[ii].2;
        }
    }
    ii
}

fn move_to(d: &mut Data, from: usize, to: usize, left: bool) {
    let (l, r) = match left {
        true => (step(d, to, -1, false), to),
        false => (to, step(d, to, 1, false)),
    };
    if l == from || r == from {
        return;
    }
    let old_l = d[from].1;
    let old_r = d[from].2;
    d[old_l].2 = old_r;
    d[old_r].1 = old_l;
    d[l].2 = from;
    d[r].1 = from;
    d[from].1 = l;
    d[from].2 = r;
}

fn process_round(data: &mut Data) {
    for i in 0..data.len() {
        let new_i = step(&data, i, data[i].0, true);
        let left = data[i].0 < 0;
        move_to(data, i, new_i, left);
    }
}

fn result(data: &Data) -> i64 {
    let mut pos = data.iter().position(|d| d.0 == 0).unwrap();
    let mut result = 0;
    for _ in 0..3 {
        pos = step(&data, pos, 1000, false);
        result += data[pos].0;
    }
    result
}

fn main() {
    let input: Vec<_> = fs::read_to_string("input.txt")
        .expect("Failed to read file.")
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
    let mut data: Vec<_> = input
        .iter()
        .enumerate()
        .map(|(i, v)| {
            (
                *v,
                (i + input.len() - 1) % input.len(),
                (i + 1) % input.len(),
            )
        })
        .collect();
    let mut data1 = data.clone();
    process_round(&mut data1);
    println!("Result 1: {}", result(&data1));
    data.iter_mut().for_each(|d| d.0 *= 811589153);
    for _ in 0..10 {
        process_round(&mut data);
    }
    println!("Result 2: {}", result(&data));
}
