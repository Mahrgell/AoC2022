use std::collections::HashMap;
use std::fs;

#[derive(Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

enum Monkey {
    Nb(i64),
    Eval(String, String, Op),
}

#[derive(Clone)]
enum Monkey2 {
    Input(),
    Nb(i64),
    Eval(String, String, Op),
}

impl Op {
    fn apply(&self, a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
        match self {
            Op::Add => (a.0 + b.0, a.1 + b.1),
            Op::Sub => (a.0 - b.0, a.1 - b.1),
            Op::Mul => {
                assert_eq!(a.0 * b.0, 0.);
                (a.0 * b.1 + a.1 * b.0, a.1 * b.1)
            }
            Op::Div => {
                assert_eq!(b.0, 0.);
                (a.0 / b.1, a.1 / b.1)
            }
        }
    }
}

impl Monkey {
    fn from(s: &str) -> (String, Monkey) {
        let w: Vec<_> = s.split_whitespace().collect();
        let name = String::from(w[0].trim_end_matches(|c| c == ':'));
        let monkey = match w.len() {
            2 => Monkey::Nb(w[1].parse().unwrap()),
            4 => Monkey::Eval(
                String::from(w[1]),
                String::from(w[3]),
                match w[2] {
                    "+" => Op::Add,
                    "-" => Op::Sub,
                    "*" => Op::Mul,
                    "/" => Op::Div,
                    _ => panic!(),
                },
            ),
            _ => panic!(),
        };
        (name, monkey)
    }

    fn eval(&self, other: &HashMap<String, Monkey>) -> f64 {
        match self {
            Monkey::Nb(val) => *val as f64,
            Monkey::Eval(a, b, Op::Add) => other[a].eval(other) + other[b].eval(other),
            Monkey::Eval(a, b, Op::Sub) => other[a].eval(other) - other[b].eval(other),
            Monkey::Eval(a, b, Op::Mul) => other[a].eval(other) * other[b].eval(other),
            Monkey::Eval(a, b, Op::Div) => other[a].eval(other) / other[b].eval(other),
        }
    }
}

impl Monkey2 {
    fn from(m: &Monkey) -> Monkey2 {
        match m {
            Monkey::Nb(a) => Monkey2::Nb(*a),
            Monkey::Eval(a, b, c) => Monkey2::Eval(a.clone(), b.clone(), *c),
        }
    }

    fn eval(&self, other: &HashMap<String, Monkey2>) -> (f64, f64) {
        match self {
            Monkey2::Input() => (1., 0.),
            Monkey2::Nb(val) => (0., *val as f64),
            Monkey2::Eval(a, b, op) => op.apply(other[a].eval(other), other[b].eval(other)),
        }
    }
}

fn main() {
    let monkeys: HashMap<_, _> = fs::read_to_string("input.txt")
        .expect("Failed to read file.")
        .lines()
        .map(Monkey::from)
        .collect();
    println!("Root: {}", monkeys[&String::from("root")].eval(&monkeys));
    let mut monkeys2: HashMap<_, _>= monkeys.iter().map(|(name, m)|(name.clone(), Monkey2::from(m))).collect();
    monkeys2.insert(String::from("humn"),Monkey2::Input());
    let mut root = monkeys2[&String::from("root")].clone();
    if let Monkey2::Eval(_,_,op) = &mut root {
        *op = Op::Sub;
    }
    let (a, b) = root.eval(&monkeys2);
    println!("Human shout: {}", -b/a);
}
