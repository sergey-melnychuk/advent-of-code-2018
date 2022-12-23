extern crate regex;

use std::io;
use std::io::prelude::*;

use self::regex::Regex;

#[derive(Debug, Eq, PartialEq)]
struct Pos(i64, i64, i64);

impl Pos {
    fn distance_to(&self, that: &Pos) -> u64 {
        ((that.0 - self.0).abs() + (that.1 - self.1).abs() + (that.2 - self.2).abs()) as u64
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Rec {
    pos: Pos,
    r: u64,
}

impl Rec {
    fn in_range(&self, that: &Pos) -> bool {
        self.pos.distance_to(that) <= self.r
    }
}

fn get_input() -> Vec<String> {
    let stdin = io::stdin();
    let lines = stdin
        .lock()
        .lines()
        .into_iter()
        .map(Result::unwrap)
        .collect();
    lines
}

fn parse_input(lines: Vec<String>) -> Vec<Rec> {
    let re = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
    lines
        .into_iter()
        .filter_map(|line| parse_rec(&line, &re))
        .collect()
}

fn parse_rec(line: &str, regex: &Regex) -> Option<Rec> {
    if let Some(cap) = regex.captures_iter(line).next() {
        let x: i64 = cap[1].parse().unwrap();
        let y: i64 = cap[2].parse().unwrap();
        let z: i64 = cap[3].parse().unwrap();
        let r: u64 = cap[4].parse().unwrap();
        let rec = Rec {
            pos: Pos(x, y, z),
            r,
        };
        return Some(rec);
    }
    None
}

fn find_strongest(recs: &[Rec]) -> usize {
    let mut max: u64 = 0;
    let mut idx: usize = 0;
    for (i, rec) in recs.iter().enumerate() {
        if rec.r > max {
            max = rec.r;
            idx = i;
        }
    }
    idx
}

fn find_in_range(idx: usize, recs: &[Rec]) -> usize {
    let mut count = 0;
    let strongest = &recs[idx];
    for (_i, rec) in recs.iter().enumerate() {
        if strongest.in_range(&rec.pos) {
            count += 1;
        }
    }
    count
}

fn solve1(recs: &[Rec]) -> usize {
    let idx = find_strongest(recs);
    find_in_range(idx, recs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = vec![
            String::from("pos=<77257099,50252980,47219056>, r=92361671"),
            String::from("pos=<53167623,26016086,28898789>, r=74188754"),
            String::from("pos=<8924567,89634339,-10531384>, r=73102870"),
        ];

        assert_eq!(
            parse_input(input),
            vec![
                Rec {
                    pos: Pos(77257099, 50252980, 47219056),
                    r: 92361671
                },
                Rec {
                    pos: Pos(53167623, 26016086, 28898789),
                    r: 74188754
                },
                Rec {
                    pos: Pos(8924567, 89634339, -10531384),
                    r: 73102870
                },
            ]
        );
    }
}

pub fn main() {
    let input = parse_input(get_input());
    println!("input: {}", input.len());

    let s1 = solve1(&input);
    println!("{}", s1); // 580
}
