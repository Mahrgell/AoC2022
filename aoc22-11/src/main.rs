use std::{fs, str::Lines};

#[derive(Clone)]
enum Op {
    Add,
    Mul,
}

#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    op: Op,
    op_rhs: Option<u64>,
    div_test: u64,
    true_target: usize,
    false_target: usize,
}

fn run_rounds(mut monkeys: Vec<Monkey>, rounds: usize, div_by_3: bool) {
    let divi = monkeys.iter().fold(1, |a, m| a * m.div_test);
    let mut handle_counter = vec![0; monkeys.len()];
    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            let items = monkeys[m].items.clone();
            handle_counter[m] += items.len();
            monkeys[m].items.clear();
            for i in items {
                let rhs = monkeys[m].op_rhs.unwrap_or(i);
                let mut w = match monkeys[m].op {
                    Op::Add => i + rhs,
                    Op::Mul => i * rhs,
                };
                if div_by_3 {
                    w /= 3;
                }
                w %= divi;
                let divtest = w % monkeys[m].div_test == 0;
                let target = if divtest {
                    monkeys[m].true_target
                } else {
                    monkeys[m].false_target
                };
                monkeys[target].items.push(w);
            }
        }
    }
    handle_counter.sort_unstable();
    handle_counter.reverse();
    println!(
        "{} rounds, div3: {} -> {}",
        rounds,
        div_by_3,
        handle_counter[0] * handle_counter[1]
    )
}

fn get_next_words<'a>(lines: &'a mut Lines) -> Vec<&'a str> {
    let l = lines.next().unwrap();
    l.split_whitespace().collect()
}

fn read_items(lines: &mut Lines) -> Vec<u64> {
    let w = get_next_words(lines);
    let mut items = Vec::new();
    for i in 2..w.len() {
        let item = w[i].trim_end_matches(',');
        items.push(item.parse::<u64>().unwrap());
    }
    items
}

fn read_op(lines: &mut Lines) -> (Op, Option<u64>) {
    let w = get_next_words(lines);
    let op = match w[4] {
        "+" => Op::Add,
        "*" => Op::Mul,
        _ => panic!(),
    };
    let rhs = match w[5] {
        "old" => None,
        n => Some(n.parse::<u64>().unwrap())
    };
    (op, rhs)
}

fn read_monkey(lines: &mut Lines) -> Monkey {
    lines.next(); // Monkey #
    let items = read_items(lines);
    let (op, op_rhs) = read_op(lines);    
    let w = get_next_words(lines);
    let div_test = w[3].parse::<u64>().unwrap();
    let w = get_next_words(lines);
    let true_target = w[5].parse::<usize>().unwrap();
    let w = get_next_words(lines);
    let false_target = w[5].parse::<usize>().unwrap();
    Monkey {
        items,
        op,
        op_rhs,
        div_test,
        true_target,
        false_target,
    }
}

fn read_monkeys() -> Vec<Monkey> {
    let input = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut monkeys = Vec::new();
    let mut lines = input.lines();
    loop {
        monkeys.push(read_monkey(&mut lines));
        if lines.next().is_none() {
            break;
        }
    }
    monkeys
}

fn main() {
    let monkeys = read_monkeys();

    run_rounds(monkeys.clone(), 20, true);
    run_rounds(monkeys, 10000, false);
}
