extern crate regex;

use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

use self::regex::Regex;

#[derive(Debug, Eq, PartialEq)]
struct State {
    ra: usize,
    rb: usize,
    rc: usize,
    rd: usize,
}

impl State {
    fn regs(&self) -> [usize; 4] {
        [self.ra, self.rb, self.rc, self.rd]
    }

    fn make(regs: [usize; 4]) -> State {
        State {
            ra: regs[0],
            rb: regs[1],
            rc: regs[2],
            rd: regs[3],
        }
    }
}

#[derive(Debug)]
struct Code {
    op: usize,
    a: usize,
    b: usize,
    c: usize,
}

#[derive(Debug)]
struct Record {
    was: State,
    now: State,
    code: Code,
}

fn addr(a: usize, b: usize, c: usize, regs: &[usize; 4]) -> [usize; 4] {
    let mut out = [regs[0], regs[1], regs[2], regs[3]];
    out[c] = regs[a] + regs[b];
    out
}

fn addi(a: usize, b: usize, c: usize, regs: &[usize; 4]) -> [usize; 4] {
    let mut out = [regs[0], regs[1], regs[2], regs[3]];
    out[c] = regs[a] + b;
    out
}

fn mulr(a: usize, b: usize, c: usize, regs: &[usize; 4]) -> [usize; 4] {
    let mut out = [regs[0], regs[1], regs[2], regs[3]];
    out[c] = regs[a] * regs[b];
    out
}

fn muli(a: usize, b: usize, c: usize, regs: &[usize; 4]) -> [usize; 4] {
    let mut out = [regs[0], regs[1], regs[2], regs[3]];
    out[c] = regs[a] * b;
    out
}

fn banr(a: usize, b: usize, c: usize, regs: &[usize; 4]) -> [usize; 4] {
    let mut out = [regs[0], regs[1], regs[2], regs[3]];
    out[c] = regs[a] & regs[b];
    out
}

fn bani(a: usize, b: usize, c: usize, regs: &[usize; 4]) -> [usize; 4] {
    let mut out = [regs[0], regs[1], regs[2], regs[3]];
    out[c] = regs[a] & b;
    out
}

fn borr(a: usize, b: usize, c: usize, regs: &[usize; 4]) -> [usize; 4] {
    let mut out = [regs[0], regs[1], regs[2], regs[3]];
    out[c] = regs[a] | regs[b];
    out
}

fn bori(a: usize, b: usize, c: usize, regs: &[usize; 4]) -> [usize; 4] {
    let mut out = [regs[0], regs[1], regs[2], regs[3]];
    out[c] = regs[a] | b;
    out
}

fn setr(a: usize, _: usize, c: usize, regs: &[usize; 4]) -> [usize; 4] {
    let mut out = [regs[0], regs[1], regs[2], regs[3]];
    out[c] = regs[a];
    out
}

fn seti(a: usize, _: usize, c: usize, regs: &[usize; 4]) -> [usize; 4] {
    let mut out = [regs[0], regs[1], regs[2], regs[3]];
    out[c] = a;
    out
}

fn gtir(a: usize, b: usize, c: usize, regs: &[usize; 4]) -> [usize; 4] {
    let mut out = [regs[0], regs[1], regs[2], regs[3]];
    out[c] = if a > regs[b] { 1 } else { 0 };
    out
}

fn gtri(a: usize, b: usize, c: usize, regs: &[usize; 4]) -> [usize; 4] {
    let mut out = [regs[0], regs[1], regs[2], regs[3]];
    out[c] = if regs[a] > b { 1 } else { 0 };
    out
}

fn gtrr(a: usize, b: usize, c: usize, regs: &[usize; 4]) -> [usize; 4] {
    let mut out = [regs[0], regs[1], regs[2], regs[3]];
    out[c] = if regs[a] > regs[b] { 1 } else { 0 };
    out
}

fn eqir(a: usize, b: usize, c: usize, regs: &[usize; 4]) -> [usize; 4] {
    let mut out = [regs[0], regs[1], regs[2], regs[3]];
    out[c] = if a == regs[b] { 1 } else { 0 };
    out
}

fn eqri(a: usize, b: usize, c: usize, regs: &[usize; 4]) -> [usize; 4] {
    let mut out = [regs[0], regs[1], regs[2], regs[3]];
    out[c] = if regs[a] == b { 1 } else { 0 };
    out
}

fn eqrr(a: usize, b: usize, c: usize, regs: &[usize; 4]) -> [usize; 4] {
    let mut out = [regs[0], regs[1], regs[2], regs[3]];
    out[c] = if regs[a] == regs[b] { 1 } else { 0 };
    out
}

fn fits<F>(rec: &Record, f: F) -> bool
where
    F: Fn(usize, usize, usize, &[usize; 4]) -> [usize; 4],
{
    let (a, b, c) = (rec.code.a, rec.code.b, rec.code.c);
    f(a, b, c, &rec.was.regs()) == rec.now.regs()
}

fn nfits(rec: &Record) -> Vec<usize> {
    let mut out = Vec::new();
    if fits(rec, addr) {
        out.push(0);
    }
    if fits(rec, addi) {
        out.push(1);
    }
    if fits(rec, mulr) {
        out.push(2);
    }
    if fits(rec, muli) {
        out.push(3);
    }
    if fits(rec, banr) {
        out.push(4);
    }
    if fits(rec, bani) {
        out.push(5);
    }
    if fits(rec, borr) {
        out.push(6);
    }
    if fits(rec, bori) {
        out.push(7);
    }
    if fits(rec, setr) {
        out.push(8);
    }
    if fits(rec, seti) {
        out.push(9);
    }
    if fits(rec, gtir) {
        out.push(10);
    }
    if fits(rec, gtri) {
        out.push(11);
    }
    if fits(rec, gtrr) {
        out.push(12);
    }
    if fits(rec, eqir) {
        out.push(13);
    }
    if fits(rec, eqri) {
        out.push(14);
    }
    if fits(rec, eqrr) {
        out.push(15);
    }
    out
}

fn call(code: Code, state: State, mapping: [usize; 16]) -> State {
    match mapping[code.op] {
        0 => State::make(addr(code.a, code.b, code.c, &state.regs())),
        1 => State::make(addi(code.a, code.b, code.c, &state.regs())),
        2 => State::make(mulr(code.a, code.b, code.c, &state.regs())),
        3 => State::make(muli(code.a, code.b, code.c, &state.regs())),
        4 => State::make(banr(code.a, code.b, code.c, &state.regs())),
        5 => State::make(bani(code.a, code.b, code.c, &state.regs())),
        6 => State::make(borr(code.a, code.b, code.c, &state.regs())),
        7 => State::make(bori(code.a, code.b, code.c, &state.regs())),
        8 => State::make(setr(code.a, code.b, code.c, &state.regs())),
        9 => State::make(seti(code.a, code.b, code.c, &state.regs())),
        10 => State::make(gtir(code.a, code.b, code.c, &state.regs())),
        11 => State::make(gtri(code.a, code.b, code.c, &state.regs())),
        12 => State::make(gtrr(code.a, code.b, code.c, &state.regs())),
        13 => State::make(eqir(code.a, code.b, code.c, &state.regs())),
        14 => State::make(eqri(code.a, code.b, code.c, &state.regs())),
        15 => State::make(eqrr(code.a, code.b, code.c, &state.regs())),
        _ => state,
    }
}

fn process(codes: Vec<Code>, state: State, mapping: [usize; 16]) -> State {
    let mut st = state;
    for code in codes {
        let s = call(code, st, mapping);
        st = s;
    }
    st
}

fn get_input() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let n = handle.read_to_string(&mut buffer).unwrap();
    //println!("n: {}\n{}", n, buffer);
    buffer
}

fn parse_records(input: &String) -> Vec<Record> {
    let re = Regex::new(r"Before: \[(\d+), (\d+), (\d+), (\d+)]\n(\d+) (\d+) (\d+) (\d+)\nAfter:  \[(\d+), (\d+), (\d+), (\d+)]\n?").unwrap();
    let mut result: Vec<Record> = Vec::new();

    for cap in re.captures_iter(input) {
        //println!("cap: {}", cap[0].to_owned());
        let was = State {
            ra: cap[1].parse().unwrap(),
            rb: cap[2].parse().unwrap(),
            rc: cap[3].parse().unwrap(),
            rd: cap[4].parse().unwrap(),
        };
        let code = Code {
            op: cap[5].parse().unwrap(),
            a: cap[6].parse().unwrap(),
            b: cap[7].parse().unwrap(),
            c: cap[8].parse().unwrap(),
        };
        let now = State {
            ra: cap[9].parse().unwrap(),
            rb: cap[10].parse().unwrap(),
            rc: cap[11].parse().unwrap(),
            rd: cap[12].parse().unwrap(),
        };

        let record = Record { was, now, code };
        result.push(record);
    }

    result
}

fn parse_codes(input: &String) -> Vec<Code> {
    let mut result = Vec::new();
    let mut split = input.split("\n\n\n");
    split.next();
    let rest: String = split.next().unwrap().to_string();

    let re = Regex::new(r"(\d+) (\d+) (\d+) (\d+)\n").unwrap();
    for cap in re.captures_iter(&rest) {
        let code = Code {
            op: cap[1].parse().unwrap(),
            a: cap[2].parse().unwrap(),
            b: cap[3].parse().unwrap(),
            c: cap[4].parse().unwrap(),
        };
        result.push(code);
    }
    result
}

fn part1(recs: &Vec<Record>) -> usize {
    let mut n = 0;
    for rec in recs {
        if nfits(rec).len() >= 3 {
            n += 1;
        }
    }
    n
}

fn fitness(recs: &Vec<Record>) -> HashMap<usize, HashSet<usize>> {
    let mut map = HashMap::new();
    for rec in recs {
        if !map.contains_key(&rec.code.op) {
            map.insert(rec.code.op, HashSet::new());
        }
        for op in nfits(rec) {
            map.get_mut(&rec.code.op).unwrap().insert(op);
        }
    }
    map
}

// Returns projection: result[a] => b means call function
fn reduce(mut fit: HashMap<usize, HashSet<usize>>) -> [usize; 16] {
    let mut mapping = [0; 16];

    while fit.len() > 0 {
        //        println!();
        //        for (k, v) in fit.iter() {
        //            println!("{} -> {:?}", k, v);
        //        }

        // 1. Find element that has only 1-1 mapping

        let mut found: Vec<(usize, usize)> = Vec::new();
        for (k, vals) in &fit {
            if vals.len() == 1 {
                let v = *vals.iter().next().unwrap();
                //println!("k={} maps only to v={}", k, v);
                mapping[*k] = v;
                found.push((*k, v));
            }
        }

        // 2. Remove found element from all other fits
        for f in found {
            let (k, v) = f;
            fit.remove(&k);

            for vals in fit.values_mut() {
                vals.remove(&v);
            }
        }
    }

    mapping
}

pub fn main() {
    let input = get_input();
    let records = parse_records(&input);
    println!("frames: {}", records.len());

    let val1 = part1(&records);
    println!("{}", val1); // 612

    let fit = fitness(&records);
    let map = reduce(fit);
    //println!("{:?}", map);

    let codes: Vec<Code> = parse_codes(&input);
    println!("codes: {}", codes.len());
    let zero = State::make([0, 0, 0, 0]);
    let state = process(codes, zero, map);

    println!("{}", state.ra); // 485
}
