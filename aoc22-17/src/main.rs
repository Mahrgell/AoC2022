use std::{
    collections::{HashMap, HashSet},
    fs,
};

type Pos = (i32, i32);
type Blocks = HashSet<Pos>;

struct Level {
    blocks: Blocks,
    max_h: i32,
    next_block_type: u32,
    instructions: Vec<i32>,
    instr_index: usize,
}

impl Level {
    fn new(instructions: Vec<i32>) -> Level {
        Level {
            blocks: HashSet::new(),
            max_h: 0,
            next_block_type: 0,
            instructions,
            instr_index: 0,
        }
    }

    fn spawn(&mut self) -> Option<(u32, Pos)> {
        let mut tile = Tile::spawn(self.next_block_type, self.max_h);
        self.next_block_type = (self.next_block_type + 1) % 5;
        let mut cycle_state = None;
        loop {
            tile.try_move(self.instructions[self.instr_index], 0, &self.blocks);
            self.instr_index = (self.instr_index + 1) % self.instructions.len();
            if self.instr_index == 0 {
                cycle_state = Some((self.next_block_type, (tile.pos.0, tile.pos.1 - self.max_h)));
            }
            if !tile.try_move(0, -1, &self.blocks) {
                for b in tile.get_blocks() {
                    self.max_h = self.max_h.max(b.1);
                    self.blocks.insert(b);
                }
                break;
            }
        }
        cycle_state
    }
}

struct Tile {
    pos: Pos,
    blocks: Vec<Pos>,
}

impl Tile {
    fn spawn(block_type: u32, max_h: i32) -> Tile {
        let blocks = match block_type {
            0 => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            1 => vec![(1, 1), (2, 1), (0, 1), (1, 2), (1, 0)],
            2 => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            3 => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            4 => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
            _ => panic!(),
        };
        let pos = (3, max_h + 4);
        Tile { pos, blocks }
    }

    fn get_blocks(&self) -> Vec<Pos> {
        self.blocks
            .iter()
            .map(|b| (b.0 + self.pos.0, b.1 + self.pos.1))
            .collect()
    }

    fn try_move(&mut self, dx: i32, dy: i32, b: &Blocks) -> bool {
        for bl in self.get_blocks() {
            let nx = bl.0 + dx;
            let ny = bl.1 + dy;
            if nx == 0 || nx == 8 || ny == 0 || b.contains(&(nx, ny)) {
                return false;
            }
        }
        self.pos.0 += dx;
        self.pos.1 += dy;
        true
    }
}

fn get_max_height(nb: u64, instructions: Vec<i32>) -> u64 {
    let mut cycle_states = HashMap::new();
    let mut level = Level::new(instructions.clone());
    let mut cnt = 0;
    let (divi, rem, h_per_cycle) = loop {
        cnt += 1;
        if let Some(cs) = level.spawn() {
            if let Some((last_cnt, last_h)) = cycle_states.insert(cs, (cnt, level.max_h)) {
                let divi = cnt - last_cnt;
                break (divi, nb % divi, (level.max_h - last_h) as u64);
            }
        }
        if cnt == nb {
            return level.max_h as u64;
        }
    };
    loop {
        if cnt % divi == rem {
            let base_h = level.max_h as u64 - cnt / divi * h_per_cycle;
            return nb / divi * h_per_cycle + base_h;
        }
        cnt += 1;
        level.spawn();
    }
}

fn main() {
    let instructions: Vec<_> = fs::read_to_string("input.txt")
        .expect("Failed to read file.")
        .trim()
        .chars()
        .map(|c| match c {
            '<' => -1,
            '>' => 1,
            _ => panic!(),
        })
        .collect();
    println!("2022: {}", get_max_height(2022, instructions.clone()));
    println!(
        "Obscure high number: {}",
        get_max_height(1000000000000, instructions)
    );
}
