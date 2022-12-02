use std::fs;
#[derive(Clone, Copy, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissor,
}

#[derive(Clone, Copy)]
enum RPSResult {
    Win,
    Draw,
    Loss,
}

impl RPS {
    fn from(s: &str) -> RPS {
        match s {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => RPS::Scissor,
            _ => panic!(),
        }
    }

    fn to_get_result(opponent: RPS, result: RPSResult) -> RPS {
        match result {
            RPSResult::Draw => opponent,
            RPSResult::Win => match opponent {
                RPS::Rock => RPS::Paper,
                RPS::Paper => RPS::Scissor,
                RPS::Scissor => RPS::Rock,
            },
            RPSResult::Loss => match opponent {
                RPS::Rock => RPS::Scissor,
                RPS::Paper => RPS::Rock,
                RPS::Scissor => RPS::Paper,
            },
        }
    }

    fn score(&self) -> u32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissor => 3,
        }
    }
}

impl RPSResult {
    fn from(s: &str) -> RPSResult {
        match s {
            "X" => RPSResult::Loss,
            "Y" => RPSResult::Draw,
            "Z" => RPSResult::Win,
            _ => panic!(),
        }
    }

    fn from_rps(player: RPS, opponent: RPS) -> RPSResult {
        match [player, opponent] {
            [p, o] if p == o => RPSResult::Draw,
            [p, o] if p == RPS::to_get_result(o, RPSResult::Win) => RPSResult::Win,
            _ => RPSResult::Loss,
        }
    }

    fn score(&self) -> u32 {
        match self {
            RPSResult::Win => 6,
            RPSResult::Draw => 3,
            RPSResult::Loss => 0,
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut score1 = 0;
    let mut score2 = 0;
    for l in contents.lines() {
        let w: Vec<_> = l.split_whitespace().collect();
        let elf = RPS::from(&w[0]);

        let me1 = RPS::from(&w[1]);
        let result1 = RPSResult::from_rps(me1, elf);
        score1 += me1.score() + result1.score();

        let result2 = RPSResult::from(&w[1]);
        let me2 = RPS::to_get_result(elf, result2);
        score2 += me2.score() + result2.score();
    }
    println!("score1: {}", score1);
    println!("score2: {}", score2);
}
