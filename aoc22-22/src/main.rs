#![feature(iter_advance_by)]

use std::collections::HashMap;
use std::str::Chars;
use std::{fs, iter::Peekable};

const SZ: usize = 50;
type SubField = [[bool; SZ]; SZ];

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Move(usize),
    Turn(usize),
}

struct Field {
    subfields: [(SubField, (usize, usize)); 6],
    next_sub_fn: [[(usize, usize); 4]; 6],
}

#[derive(Clone, Debug)]
struct Pos {
    subf: usize,
    pos: (usize, usize),
    facing: usize,
}

impl Field {
    fn read_from_input() -> (Field, Vec<Instruction>) {
        let input = fs::read_to_string("input.txt").expect("Failed to read file.");
        let mut subf = HashMap::new();
        let mut subf_idx = 0;
        let mut instr = Vec::new();
        for (x, l) in input.lines().enumerate() {
            let sub_x = x / SZ;
            let local_x = x % SZ;
            let mut c = l.chars().peekable();
            if let Some(n) = c.peek() {
                if ('0'..='9').contains(n) {
                    instr = parse_instructions(l);
                    break;
                }
            }
            let mut sub_y = 0;
            loop {
                match c.peek() {
                    None => break,
                    Some(' ') => {
                        assert!(!subf.contains_key(&(sub_x, sub_y)));
                        c.advance_by(SZ).unwrap();
                    }
                    _ => {
                        let mut current_subf = if local_x == 0 {
                            assert!(!subf.contains_key(&(sub_x, sub_y)));
                            let new_subf = (subf_idx, [[false; SZ]; SZ]);
                            subf_idx += 1;
                            new_subf
                        } else {
                            subf.remove(&(sub_x, sub_y)).unwrap()
                        };
                        current_subf.1[local_x] = read_line_block(&mut c);
                        subf.insert((sub_x, sub_y), current_subf);
                    }
                }
                sub_y += 1;
            }
        }
        assert_eq!(6, subf.len());
        let next_sub_fn = generate_next_sub_fn(&subf);
        let mut subfields = [([[false; SZ]; SZ], (0, 0)); 6];
        for (pos, (i, sf)) in subf.into_iter() {
            subfields[i] = (sf, pos);
        }
        (
            Field {
                subfields,
                next_sub_fn,
            },
            instr,
        )
    }
}

impl Pos {
    fn turn(&mut self, turn: usize) {
        self.facing += turn;
        self.facing %= 4;
    }

    fn advance(&self, f: &Field) -> Option<Pos> {
        let mut adv = self.clone();
        if would_wrap(&adv.pos, adv.facing, SZ) {
            let rot = f.next_sub_fn[adv.subf][adv.facing].1;
            for _ in 0..rot {
                rotate(&mut adv.pos);
            }
            adv.subf = f.next_sub_fn[adv.subf][adv.facing].0;
            adv.turn(rot);  
        }
        adv.pos = advance(&adv.pos, adv.facing, SZ);
        if f.subfields[adv.subf].0[adv.pos.0][adv.pos.1] {
            return None;
        }
        Some(adv)
    }

    fn apply(&mut self, instr: Instruction, f: &Field) {
        match instr {
            Instruction::Move(n) => {
                for _ in 0..n {
                    match self.advance(&f) {
                        None => break,
                        Some(new_p) => *self = new_p,
                    }
                }
            }
            Instruction::Turn(n) => self.turn(n),
        }
    }

    fn score(&self, f: &Field) -> usize {
        let subf_pos = f.subfields[self.subf].1;
        let real_x = self.pos.0 + SZ * subf_pos.0 + 1;
        let real_y = self.pos.1 + SZ * subf_pos.1 + 1;
        let real_facing = (self.facing + 3) % 4;
        println!("{} {} {}", real_x, real_y, real_facing);
        real_x * 1000 + real_y * 4 + real_facing
    }
}

fn read_line_block(p: &mut Peekable<Chars>) -> [bool; SZ] {
    let mut block = [false; SZ];
    for y in 0..SZ {
        block[y] = p.next() == Some('#');
    }
    block
}

fn rotate(p: &mut (usize, usize)) {
    *p = (p.1, SZ-1-p.0)
}

fn would_wrap(p: &(usize, usize), dir: usize, wrap_d: usize) -> bool {
    match dir {
        0 => p.0 == 0,
        1 => p.1 == wrap_d - 1,
        2 => p.0 == wrap_d - 1,
        3 => p.1 == 0,
        _ => panic!(),
    }
}

fn advance(p: &(usize, usize), dir: usize, wrap_d: usize) -> (usize, usize) {
    match dir {
        0 => ((p.0 + wrap_d - 1) % wrap_d, p.1),
        1 => (p.0, (p.1 + 1) % wrap_d),
        2 => ((p.0 + 1) % wrap_d, p.1),
        3 => (p.0, (p.1 + wrap_d - 1) % wrap_d),
        _ => panic!(),
    }
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
    let mut instr = Vec::new();
    let mut current_mv = 0;
    for c in s.chars() {
        match c {
            'L' => {
                if current_mv != 0 {
                    instr.push(Instruction::Move(current_mv));
                    current_mv = 0;
                }
                instr.push(Instruction::Turn(3));
            }
            'R' => {
                if current_mv != 0 {
                    instr.push(Instruction::Move(current_mv));
                    current_mv = 0;
                }
                instr.push(Instruction::Turn(1));
            }
            cc if ('0'..='9').contains(&cc) => {
                current_mv = current_mv * 10 + cc.to_digit(10).unwrap() as usize
            }
            _ => panic!(),
        }
    }
    if current_mv != 0 {
        instr.push(Instruction::Move(current_mv));
    }
    instr
}

fn generate_next_sub_fn(
    hm: &HashMap<(usize, usize), (usize, SubField)>,
) -> [[(usize, usize); 4]; 6] {
    let mut nsf = [[(0, 0); 4]; 6];
    for (p, (i, _)) in hm {
        for dir in 0..4 {
            let mut np = advance(p, dir, 4);
            while !hm.contains_key(&np) {
                np = advance(&np, dir, 4);
            }
            nsf[*i][dir].0 = hm[&np].0;
        }
    }
    nsf
}

fn main() {
    let (mut field, instructions) = Field::read_from_input();
    let mut p = Pos {
        pos: (0, 0),
        subf: 0,
        facing: 1,
    };
    for instr in &instructions {
        p.apply(*instr, &field);
    }
    
    println!("{}", p.score(&field));

    field.next_sub_fn = [
        [(5, 1), (1, 0), (2, 0), (3, 2)],
        [(5, 0), (4, 2), (2, 1), (0, 0)],
        [(0, 0), (1, 3), (4, 0), (3, 3)],
        [(2, 1), (4, 0), (5, 0), (0, 2)],
        [(2, 0), (1, 2), (5, 1), (3, 0)],
        [(3, 0), (4, 3), (1, 0), (0, 3)],
    ];

    // example net
    // field.next_sub_fn = [
    //     [(1, 2), (5, 2), (3, 0), (2, 3)],
    //     [(0, 2), (2, 0), (4, 2), (5, 1)],
    //     [(0, 1), (3, 0), (4, 3), (1, 0)],
    //     [(0, 0), (5, 1), (4, 0), (2, 0)],
    //     [(3, 0), (5, 0), (1, 2), (2, 1)],
    //     [(3, 3), (0, 2), (1, 3), (4, 0)],
    // ];

    let mut p = Pos {
        pos: (0, 0),
        subf: 0,
        facing: 1,
    };
    for instr in instructions {
        p.apply(instr, &field);
    }
    println!("{}", p.score(&field));

    additions
        .iter()
        .filter(|(x,y,z)| blocks.contains(&(x+block.0, y+block.1, z+block.2)))
        .count() as i32
}
