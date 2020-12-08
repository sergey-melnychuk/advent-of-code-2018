use std::io;
use std::io::prelude::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct State {
    ip: usize,
    rs: [usize; 6]
}

impl State {
    fn make(ip: usize, rs: [usize; 6]) -> State {
        State { ip, rs }
    }

    fn get(&self, idx: usize) -> usize {
        self.rs[idx]
    }

    fn set(&mut self, idx: usize, val: usize) {
        self.rs[idx] = val;
    }

    fn inc(&mut self, idx: usize) -> usize {
        let val = self.get(idx) + 1;
        self.set(idx, val);
        val
    }

    fn ip(&mut self, ip: usize) {
        self.ip = ip;
    }
}

#[derive(Debug, Clone)]
struct Code {
    op: String,
    a: usize,
    b: usize,
    c: usize,
}

trait Instruction {
    fn call(&self, state: &mut State) -> bool;
}

impl Instruction for Code {
    fn call(&self, state: &mut State) -> bool {
        let (a, b, c) = (self.a, self.b, self.c);
        let s = state.clone();
        match self.op.as_ref() {
            "#ip"  => state.ip(a),
            "addr" => state.set(c, s.get(a) + s.get(b)),
            "addi" => state.set(c, s.get(a) + b),
            "mulr" => state.set(c, s.get(a) * s.get(b)),
            "muli" => state.set(c, s.get(a) * b),
            "banr" => state.set(c, s.get(a) & s.get(b)),
            "bani" => state.set(c, s.get(a) & b),
            "borr" => state.set(c, s.get(a) | s.get(b)),
            "bori" => state.set(c, s.get(a) | b),
            "setr" => state.set(c, s.get(a)),
            "seti" => state.set(c, a),
            "gtrr" => state.set(c, if s.get(a) > s.get(b) {1} else {0}),
            "gtri" => state.set(c, if s.get(a) > b {1} else {0}),
            "gtir" => state.set(c, if a > s.get(b) {1} else {0}),
            "eqrr" => state.set(c, if s.get(a) == s.get(b) {1} else {0}),
            "eqri" => state.set(c, if s.get(a) == b {1} else {0}),
            "eqir" => state.set(c, if a == s.get(b) {1} else {0}),
            _  => ()
        }
        self.op != "#op"
    }
}

fn process(codes: Vec<Code>, state: State) -> State {
    let mut at: usize = 0;
    let mut st = state;

    //let mut it: usize = 0;
    while at < codes.len() {
        let op = codes.get(at).unwrap();
        //println!("at={} op={:?} st={:?}", at, op, st);
        op.call(&mut st);
        let ip = st.ip;
        let to = st.inc(ip);
        at = to;

        at += 1;
    }
    //println!("it={} done", it);

    st
}

fn get_input() -> Vec<String> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().into_iter()
        .map(Result::unwrap)
        .collect();
    lines
}

fn parse_codes(input: Vec<String>) -> (usize, Vec<Code>) {
    let mut result = Vec::new();
    let mut ip = 0;
    for line in input {
        let mut split = line.split_whitespace();
        let op = split.next().unwrap();
        let a: usize = split.next().unwrap().parse().unwrap();
        if op == "#ip" {
            ip = a;
        } else {
            let b: usize = split.next().unwrap().parse().unwrap();
            let c: usize = split.next().unwrap().parse().unwrap();
            result.push(Code { op: op.to_string(), a, b, c });
        }
    }
    (ip, result)
}

pub fn main() {
    let input = get_input();
    let (ip, codes) = parse_codes(input);
    println!("codes: {}", codes.len());

    let zero = State::make(ip, [0, 0, 0, 0, 0, 0]);
    let state = process(codes.clone(), zero);
    println!("state: {:?}", state);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wrap(lines: Vec<&'static str>) -> Vec<String> {
        lines.into_iter().map(String::from).collect()
    }

    fn make_codes(lines: Vec<&'static str>) -> (usize, Vec<Code>) {
        parse_codes(wrap(lines))
    }

    #[test]
    fn test_program() {
        let (ip, codes) = make_codes(vec![
            "#ip 0",
            "seti 5 0 1",
            "seti 6 0 2",
            "addi 0 1 0",
            "addr 1 2 3",
            "setr 1 0 0",
            "seti 8 0 4",
            "seti 9 0 5",
        ]);

        let zero = State::make(ip, [0, 0, 0, 0, 0, 0]);
        let state = process(codes, zero);
        assert_eq!(state.get(0), 7);
    }
}
