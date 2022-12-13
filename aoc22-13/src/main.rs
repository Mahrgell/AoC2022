use std::{cmp::Ordering, fs, iter::Skip, str::Chars};

#[derive(Clone, Debug, PartialEq)]
enum Packet {
    Nb(u32),
    List(Vec<Packet>),
}

fn peek_next_is_zero(chars: &Skip<Chars>) -> bool {
    chars.clone().next() == Some('0')
}

impl Packet {
    fn read_from(chars: &mut Skip<Chars>) -> Packet {
        let mut packets = Vec::new();
        loop {
            let p = match chars.next().unwrap() {
                ']' => break,
                '[' => Packet::read_from(chars),
                '1' => {
                    if peek_next_is_zero(chars) {
                        chars.next();
                        Packet::Nb(10)
                    } else {
                        Packet::Nb(1)
                    }
                }
                c if ('0'..='9').contains(&c) => Packet::Nb(c.to_digit(10).unwrap()),
                _ => panic!(),
            };
            packets.push(p);
            match chars.next().unwrap() {
                ']' => break,
                ',' => (),
                _ => panic!(),
            }
        }
        Packet::List(packets)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Nb(s), Packet::Nb(o)) => s.partial_cmp(o),
            (Packet::List(s), Packet::List(o)) => s.partial_cmp(o),
            (Packet::Nb(s), Packet::List(o)) => (vec![Packet::Nb(*s)]).partial_cmp(o),
            (Packet::List(s), Packet::Nb(o)) => s.partial_cmp(&vec![Packet::Nb(*o)]),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut lines = input.lines();
    let mut correct = 0;
    let mut index = 1;
    let mut packets = Vec::new();
    loop {
        let p1 = Packet::read_from(&mut lines.next().unwrap().chars().skip(1));
        let p2 = Packet::read_from(&mut lines.next().unwrap().chars().skip(1));
        if p1.partial_cmp(&p2) == Some(Ordering::Less) {
            correct += index;
        }
        packets.push(p1);
        packets.push(p2);
        if lines.next().is_none() {
            break;
        }
        index += 1;
    }
    println!("{}", correct);
    let div1 = Packet::List(vec![Packet::Nb(2)]);
    let div2 = Packet::List(vec![Packet::Nb(6)]);
    packets.push(div1.clone());
    packets.push(div2.clone());
    packets.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let i1 = 1 + packets.iter().position(|p| *p == div1).unwrap();
    let i2 = 1 + packets.iter().position(|p| *p == div2).unwrap();
    println!("{}", i1 * i2);
}
