use std::{collections::HashSet, io::BufRead};

/*

#ip 4
00: seti 123 0 2        # r[2] = 123;
01: bani 2 456 2        # r[2] = r[2] & 456;
02: eqri 2 72 2         # if r[2] == 72 { r[2] = 1; } else { r[2] = 0; }
03: addr 2 4 4          # r[4] = r[2] + r[4]; // when r[2] == 1, jump over line 04
04: seti 0 0 4          # // r[4] = 0; // executed only if r[2] was 0
05: seti 0 0 2          # r[2] = 0;
06: bori 2 65536 5      # r[5] = r[2] | 65536;
07: seti 5234604 6 2    # r[2] = 5234604;

08: bani 5 255 3        # r[3] = r[5] & 255;
09: addr 2 3 2          # r[2] = r[2] + r[3];
10: bani 2 16777215 2   # r[2] = r[2] & 16777215;
11: muli 2 65899 2      # r[2] = r[2] * 65899;
12: bani 2 16777215 2   # r[2] = r[2] & 16777215; // r[2] = 344955168996, r[5] = 65536,
13: gtir 256 5 3        # if 256 > r[5] { r[3] = 1; } else { r[3] = 0; };
14: addr 3 4 4          # r[4] = r[4] + r[3]; // r[3] = 0
15: addi 4 1 4          # r[4] = r[4] + 1;
16: seti 27 2 4         # // r[4] = 27; // jump to 28
17: seti 0 0 3          # r[3] = 0;

18: addi 3 1 1          # r[1] = r[3] + 1;
19: muli 1 256 1        # r[1] = r[1] * 256;
20: gtrr 1 5 1          # if r[1] > r[5] { r[1] = 1 } else { r[1] = 0 }
21: addr 1 4 4          # r[4] = r[1] + r[4];
22: addi 4 1 4          # r[4] = r[4] + 1;
23: seti 25 6 4         # // r[4] = 25; // jump to line 26
24: addi 3 1 3          # r[3] = r[3] + 1;
25: seti 17 7 4         # r[4] = 14; // jump to line 18

26: setr 3 4 5          # r[5] = r[3];
27: seti 7 8 4          # r[4] = 7; // jump to line 8

28: eqrr 2 0 3          # if r[2] == r[0] { r[3] = 1; } else { r[3] = 0; }
29: addr 3 4 4          # r[4] = r[3] + r[4]; // halt: jump to 31
30: seti 5 6 4          # r[5] = 5; // jump to line 6

*/

pub fn main() {
    let input = get_input();
    let (ip, codes) = parse_codes(input);

    let zero = State::new(ip, [0, 0, 0, 0, 0, 0]);
    process(&codes, zero);
    // 13522479
    // 14626276
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct State {
    ip: usize,
    rs: [usize; 6]
}

impl State {
    fn new(ip: usize, rs: [usize; 6]) -> State {
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

impl Code {
    fn call(&self, state: &mut State) {
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
    }
}

fn process(codes: &[Code], mut state: State) -> State {
    let mut at: usize = 0;

    let mut seen = HashSet::new();

    let mut min1 = usize::MAX;
    let mut last = usize::MAX;
    let mut n: usize = 0;
    while at < codes.len() {
        n += 1;
        if n >= usize::MAX {
            break;
        }

        let op = &codes[at];
        //println!("at={} op={:?} st={:?}", at, op, st);

        if at == 28 {
            let r2 = state.get(2);

            if min1 == usize::MAX {
                min1 = r2;
                println!("{}", min1);
                //println!("eqrr: r[2] = {}, n = {}", state.get(2), n);
            }

            if seen.contains(&r2) {
                break;
            }
            seen.insert(r2);
            last = r2;
        }

        op.call(&mut state);
        let ip = state.ip;
        let to = state.inc(ip);
        at = to;
    }

    //println!("at={} done", at);
    println!("{}", last);
    state
}

fn get_input() -> Vec<String> {
    let stdin = std::io::stdin();
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

        let zero = State::new(ip, [0, 0, 0, 0, 0, 0]);
        let state = process(&codes, zero);
        assert_eq!(state.get(0), 6);
    }
}
