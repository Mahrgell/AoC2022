use std::collections::HashSet;
use std::fs;

fn char_to_int(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => panic!(),
    }
}

fn vec_to_hashset(values: &Vec<u32>, from: usize, to: usize) -> HashSet<u32> {
    let mut hs = HashSet::new();
    for i in from..to {
        hs.insert(values[i]);
    }
    hs
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut score1 = 0;
    let mut score2 = 0;
    let mut badge_intersec = HashSet::new();
    let mut nb_rucksacks = 0;
    for l in contents.lines() {
        let values: Vec<_> = l.chars().map(|c| char_to_int(c)).collect();
        let first_half = vec_to_hashset(&values, 0, values.len() / 2);
        let second_half = vec_to_hashset(&values, values.len() / 2, values.len());
        score1 += first_half.intersection(&second_half).next().unwrap();

        nb_rucksacks += 1;
        let rucksack = vec_to_hashset(&values, 0, values.len());
        if nb_rucksacks == 1 {
            badge_intersec = rucksack;
        } else {
            badge_intersec = badge_intersec.intersection(&rucksack).cloned().collect();
            if nb_rucksacks == 3 {
                score2 += badge_intersec.iter().next().unwrap();
                nb_rucksacks = 0;
            }
        }
    }
    println!("score1: {}", score1);
    println!("score2: {}", score2);
}
